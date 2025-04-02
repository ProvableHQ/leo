// Copyright (C) 2019-2025 Provable Inc.
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
use leo_errors::LoopUnrollerError;

use super::UnrollingVisitor;

impl StatementReconstructor for UnrollingVisitor<'_> {
    fn reconstruct_block(&mut self, mut input: Block) -> (Block, Self::AdditionalOutput) {
        self.in_scope(input.id(), |slf| {
            input.statements = input.statements.into_iter().map(|stmt| slf.reconstruct_statement(stmt).0).collect();

            (input, Default::default())
        })
    }

    fn reconstruct_const(&mut self, input: ConstDeclaration) -> (Statement, Self::AdditionalOutput) {
        (input.into(), Default::default())
    }

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        (input.into(), Default::default())
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

        // These unwraps work because these literals were either found during parsing and validated
        // during type checking or else created during const folding.
        let start_value = Value::try_from(start_lit).unwrap();
        let stop_value = Value::try_from(stop_lit).unwrap();

        // Ensure loop bounds are increasing. This cannot be done in the type checker because constant propagation must happen first.
        if match (input.type_.clone(), &start_value, &stop_value) {
            (Integer(IntegerType::I8), Value::I8(lower_bound, _), Value::I8(upper_bound, _)) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::I16), Value::I16(lower_bound, _), Value::I16(upper_bound, _)) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::I32), Value::I32(lower_bound, _), Value::I32(upper_bound, _)) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::I64), Value::I64(lower_bound, _), Value::I64(upper_bound, _)) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::I128), Value::I128(lower_bound, _), Value::I128(upper_bound, _)) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::U8), Value::U8(lower_bound, _), Value::U8(upper_bound, _)) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::U16), Value::U16(lower_bound, _), Value::U16(upper_bound, _)) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::U32), Value::U32(lower_bound, _), Value::U32(upper_bound, _)) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::U64), Value::U64(lower_bound, _), Value::U64(upper_bound, _)) => {
                lower_bound >= upper_bound
            }
            (Integer(IntegerType::U128), Value::U128(lower_bound, _), Value::U128(upper_bound, _)) => {
                lower_bound >= upper_bound
            }
            _ => panic!("Type checking guarantees that the loop bounds have same type as loop variable."),
        } {
            self.emit_err(LoopUnrollerError::loop_range_decreasing(input.stop.span()));
        }

        self.loop_unrolled = true;

        (self.unroll_iteration_statement::<i128>(input, start_value, stop_value), Default::default())
    }
}
