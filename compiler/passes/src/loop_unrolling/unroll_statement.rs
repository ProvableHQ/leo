// Copyright (C) 2019-2025 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_ast::{Expression::Literal, Type::Integer, *};
use leo_errors::loop_unroller::LoopUnrollerError;

use crate::unroller::Unroller;

impl StatementReconstructor for Unroller<'_> {
    fn reconstruct_block(&mut self, mut input: Block) -> (Block, Self::AdditionalOutput) {
        self.in_scope(input.id(), |slf| {
            input.statements = input.statements.into_iter().map(|stmt| slf.reconstruct_statement(stmt).0).collect();

            (input, Default::default())
        })
    }

    fn reconstruct_const(&mut self, input: ConstDeclaration) -> (Statement, Self::AdditionalOutput) {
        (Statement::Const(input), Default::default())
    }

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        (Statement::Definition(input), Default::default())
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        // There's no need to reconstruct the bound expressions - they must be constants
        // which can be evaluated through constant propagation.

        let Literal(start_lit) = &input.start else {
            self.loop_not_unrolled = Some(input.start.span());
            return (Statement::Iteration(Box::new(input)), Default::default());
        };

        let Literal(stop_lit) = &input.stop else {
            self.loop_not_unrolled = Some(input.stop.span());
            return (Statement::Iteration(Box::new(input)), Default::default());
        };

        input.start_value.replace(Some(Value::try_from(start_lit).unwrap()));
        input.stop_value.replace(Some(Value::try_from(stop_lit).unwrap()));

        // Ensure loop bounds are increasing. This cannot be done in the type checker because constant propagation must happen first.
        if match (input.type_.clone(), input.start_value.borrow().as_ref(), input.stop_value.borrow().as_ref()) {
            (Integer(IntegerType::I8), Some(Value::I8(lower_bound, _)), Some(Value::I8(upper_bound, _))) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::I16), Some(Value::I16(lower_bound, _)), Some(Value::I16(upper_bound, _))) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::I32), Some(Value::I32(lower_bound, _)), Some(Value::I32(upper_bound, _))) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::I64), Some(Value::I64(lower_bound, _)), Some(Value::I64(upper_bound, _))) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::I128), Some(Value::I128(lower_bound, _)), Some(Value::I128(upper_bound, _))) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::U8), Some(Value::U8(lower_bound, _)), Some(Value::U8(upper_bound, _))) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::U16), Some(Value::U16(lower_bound, _)), Some(Value::U16(upper_bound, _))) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::U32), Some(Value::U32(lower_bound, _)), Some(Value::U32(upper_bound, _))) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::U64), Some(Value::U64(lower_bound, _)), Some(Value::U64(upper_bound, _))) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::U128), Some(Value::U128(lower_bound, _)), Some(Value::U128(upper_bound, _))) => {
                lower_bound >= upper_bound
            }
            _ => unreachable!("Type checking guarantees that the loop bounds have same type as loop variable."),
        } {
            self.emit_err(LoopUnrollerError::loop_range_decreasing(input.stop.span()));
        }

        self.loop_unrolled = true;

        (self.unroll_iteration_statement::<i128>(input), Default::default())
    }
}
