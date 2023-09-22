// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use itertools::Itertools;
use leo_ast::*;
use leo_span::{Span, Symbol};

use crate::{unroller::Unroller, VariableSymbol, VariableType};

impl StatementReconstructor for Unroller<'_> {
    fn reconstruct_block(&mut self, input: Block) -> (Block, Self::AdditionalOutput) {
        let scope_index = self.current_scope_index();

        // Enter the block scope.
        let previous_scope_index = self.enter_scope(scope_index);

        let block = Block {
            statements: input.statements.into_iter().map(|s| self.reconstruct_statement(s).0).collect(),
            span: input.span,
            id: input.id,
        };

        // Exit the block scope.
        self.exit_scope(previous_scope_index);

        (block, Default::default())
    }

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        // Helper function to add  variables to symbol table
        let insert_variable = |symbol: Symbol, type_: Type, span: Span, declaration: VariableType| {
            if let Err(err) =
                self.symbol_table.borrow_mut().insert_variable(symbol, VariableSymbol { type_, span, declaration })
            {
                self.handler.emit_err(err);
            }
        };

        let declaration =
            if input.declaration_type == DeclarationType::Const { VariableType::Const } else { VariableType::Mut };

        // If we are unrolling a loop, then we need to repopulate the symbol table.
        // If we are not unrolling a loop, the we need to remove constants from the symbol table.
        // We always need to add constant variables to the constant variable table.
        if declaration == VariableType::Mut && self.is_unrolling {
            match &input.place {
                Expression::Identifier(identifier) => {
                    insert_variable(identifier.name, input.type_.clone(), input.span, declaration);
                }
                Expression::Tuple(tuple_expression) => {
                    let tuple_type = match input.type_ {
                        Type::Tuple(ref tuple_type) => tuple_type,
                        _ => unreachable!(
                            "Type checking guarantees that if the lhs is a tuple, its associated type is also a tuple."
                        ),
                    };
                    tuple_expression.elements.iter().zip_eq(tuple_type.0.iter()).for_each(|(expression, _type_)| {
                        let identifier = match expression {
                            Expression::Identifier(identifier) => identifier,
                            _ => unreachable!("Type checking guarantees that if the lhs is a tuple, all of its elements are identifiers.")
                        };
                        insert_variable(identifier.name, input.type_.clone(), input.span, declaration);
                    });
                }
                _ => unreachable!(
                    "Type checking guarantees that the lhs of a `DefinitionStatement` is either an identifier or tuple."
                ),
            }
        } else if declaration == VariableType::Const {
            return (Statement::Definition(self.reconstruct_const(input.clone())), true);
        }

        // Reconstruct the expression and return
        (
            Statement::Definition(DefinitionStatement {
                declaration_type: input.declaration_type,
                place: input.place,
                type_: input.type_,
                value: self.reconstruct_expression(input.value).0,
                span: input.span,
                id: input.id,
            }),
            false,
        )
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        // Reconstruct the bound expressions
        let (new_start, _) = self.reconstruct_expression(input.start);
        let (new_stop, _) = self.reconstruct_expression(input.stop);

        // Convert into values
        match (new_start.clone(), new_stop.clone()) {
            (Literal(start_lit), Literal(stop_lit)) => {
                input.start_value.replace(Some(Value::try_from(&start_lit).unwrap()));
                input.stop_value.replace(Some(Value::try_from(&stop_lit).unwrap()));
            }
            (Literal(_), _) => self.emit_err(LoopUnrollerError::loop_bound_must_be_a_literal(new_stop.span())),
            (_, _) => self.emit_err(LoopUnrollerError::loop_bound_must_be_a_literal(new_start.span())),
        };

        // Ensure loop bounds are increasing. This cannot be done in the type checker because constant propagation occurs in this pass.
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
            _ => {
                self.emit_err(LoopUnrollerError::loop_bounds_must_have_same_type_as_loop_variable(
                    input.variable.span(),
                ));
                false
            }
        } {
            self.emit_err(LoopUnrollerError::loop_range_decreasing(new_stop.span()));
        }

        (
            self.unroll_iteration_statement::<i128>(IterationStatement {
                variable: input.variable,
                type_: input.type_,
                start: new_start,
                stop: new_stop,
                start_value: input.start_value.clone(),
                stop_value: input.stop_value.clone(),
                inclusive: false,
                block: input.block,
                span: input.span,
                id: input.id,
            }),
            Default::default(),
        )
    }
}
