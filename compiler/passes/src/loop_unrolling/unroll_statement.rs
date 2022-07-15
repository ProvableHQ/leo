// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use std::cell::RefCell;

use leo_ast::*;
use leo_errors::FlattenError;

use crate::unroller::Unroller;
use crate::{VariableSymbol, VariableType};

impl StatementReconstructor for Unroller<'_> {
    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> Statement {
        // If we are unrolling a loop, then we need to repopulate the symbol table.
        if self.is_unrolling {
            let declaration = if input.declaration_type == DeclarationType::Const {
                VariableType::Const
            } else {
                VariableType::Mut
            };

            // TODO: Do we need to obey shadowing rules?
            input.variable_names.iter().for_each(|v| {
                if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
                    v.identifier.name,
                    VariableSymbol {
                        type_: input.type_.clone(),
                        span: input.span(),
                        declaration: declaration.clone(),
                    },
                ) {
                    self.handler.emit_err(err);
                }
            });
        }
        Statement::Definition(input)
    }

    // TODO: Handle errors. This pass should not fail, rather at most it should warn that some iteration statements were not unrolled.
    fn reconstruct_iteration(&mut self, input: IterationStatement) -> Statement {
        // We match on start and stop cause loops require
        // bounds to be constants.
        match (
            input.start_value.clone().into_inner(),
            input.stop_value.clone().into_inner(),
        ) {
            (Some(start), Some(stop)) => {
                // Closure to check that the constant values are valid usize.
                // We already know these are integers since loop unrolling occurs after type checking.
                let cast_to_usize = |v: Value| -> Result<u128, Statement> {
                    match v.try_into() {
                        Ok(val_as_usize) => Ok(val_as_usize),
                        Err(err) => {
                            self.handler.emit_err(err);
                            Err(Statement::dummy(input.span))
                        }
                    }
                };
                let start = match cast_to_usize(start) {
                    Ok(v) => v,
                    Err(s) => return s,
                };
                let stop = match cast_to_usize(stop) {
                    Ok(v) => v,
                    Err(s) => return s,
                };

                // Create iteration range accounting for inclusive bounds.
                let range = if start < stop {
                    if let Some(stop) = stop.checked_add(input.inclusive as u128) {
                        start..stop
                    } else {
                        self.handler
                            .emit_err(FlattenError::incorrect_loop_bound("stop", "usize::MAX + 1", input.span));
                        Default::default()
                    }
                } else if let Some(start) = start.checked_sub(input.inclusive as u128) {
                    stop..(start)
                } else {
                    self.handler
                        .emit_err(FlattenError::incorrect_loop_bound("start", "-1", input.span));
                    Default::default()
                };

                let scope_index = self.get_current_block();

                // Enter the scope of the loop body.
                let prev_st = std::mem::take(&mut self.symbol_table);
                self.symbol_table
                    .swap(prev_st.borrow().get_block_scope(scope_index).unwrap());
                self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));
                self.block_index = 0;

                // Clear the symbol table for the loop body.
                // This is necessary because loop unrolling transforms the program, which requires reconstructing the symbol table.
                self.symbol_table.borrow_mut().variables.clear();
                self.symbol_table.borrow_mut().scopes.clear();
                self.symbol_table.borrow_mut().scope_index = 0;

                // Create a block statement to replace the iteration statement.
                // Creates a new block per iteration inside the outer block statement.
                let iter_blocks = Statement::Block(Block {
                    statements: range
                        .into_iter()
                        .map(|iteration_count| {
                            let scope_index = self.symbol_table.borrow_mut().insert_block();
                            let prev_st = std::mem::take(&mut self.symbol_table);
                            self.symbol_table
                                .swap(prev_st.borrow().get_block_scope(scope_index).unwrap());
                            self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));

                            let prev_create_iter_scopes = self.is_unrolling;
                            self.is_unrolling = true;

                            // Reconstruct `iteration_count` as a `Literal`.
                            let value = match &input.type_ {
                                Type::I8 => Literal::I8(iteration_count.to_string(), Default::default()),
                                Type::I16 => Literal::I16(iteration_count.to_string(), Default::default()),
                                Type::I32 => Literal::I32(iteration_count.to_string(), Default::default()),
                                Type::I64 => Literal::I64(iteration_count.to_string(), Default::default()),
                                Type::I128 => Literal::I128(iteration_count.to_string(), Default::default()),
                                Type::U8 => Literal::U8(iteration_count.to_string(), Default::default()),
                                Type::U16 => Literal::U16(iteration_count.to_string(), Default::default()),
                                Type::U32 => Literal::U32(iteration_count.to_string(), Default::default()),
                                Type::U64 => Literal::U64(iteration_count.to_string(), Default::default()),
                                Type::U128 => Literal::U128(iteration_count.to_string(), Default::default()),
                                _ => unreachable!("The iteration variable must be an integer type. This should be enforced by type checking."),
                            };


                            // The first statement in the block is the assignment of the loop variable to the current iteration count.
                            let mut statements = vec![
                                self.reconstruct_definition(DefinitionStatement {
                                    declaration_type: DeclarationType::Const,
                                    type_: input.type_.clone(),
                                    value: Expression::Literal(value),
                                    span: Default::default(),
                                    variable_names: vec![VariableName { mutable: false, identifier: input.variable, span: Default::default() }]
                                }),
                            ];

                            // Reconstruct the statements in the loop body.
                            input.block.statements.clone().into_iter().for_each(|s| {
                                statements.push(self.reconstruct_statement(s));
                            });

                            let block = Statement::Block(Block {
                                statements,
                                span: input.block.span,
                            });

                            self.is_unrolling = prev_create_iter_scopes;

                            // Restore the previous symbol table.
                            let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
                            self.symbol_table.swap(prev_st.get_block_scope(scope_index).unwrap());
                            self.symbol_table = RefCell::new(prev_st);

                            block
                        })
                        .collect(),
                    span: input.span,
                });

                // Restore the previous symbol table.
                let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
                self.symbol_table.swap(prev_st.get_block_scope(scope_index).unwrap());
                self.symbol_table = RefCell::new(prev_st);
                self.block_index = scope_index + 1;

                iter_blocks
            }
            // If both loop bounds are not constant, then the loop is not unrolled.
            _ => Statement::Iteration(Box::from(input)),
        }
    }

    fn reconstruct_block(&mut self, input: Block) -> Block {
        let current_block = self.get_current_block();

        // Enter block scope.
        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table
            .swap(prev_st.borrow().get_block_scope(current_block).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));
        self.block_index = 0;

        let block = Block {
            statements: input
                .statements
                .into_iter()
                .map(|s| self.reconstruct_statement(s))
                .collect(),
            span: input.span,
        };

        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table.swap(prev_st.get_block_scope(current_block).unwrap());
        self.symbol_table = RefCell::new(prev_st);
        self.block_index = current_block + 1;

        block
    }
}
