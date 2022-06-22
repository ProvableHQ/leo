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

use crate::{Declaration, Flattener, Value, VariableSymbol};

impl<'a> StatementReconstructor for Flattener<'a> {
    fn reconstruct_assign(&mut self, input: AssignStatement) -> Statement {
        self.in_assign = true;
        let (place, _) = self.reconstruct_expression(input.place);
        self.in_assign = false;

        let (value, const_val) = self.reconstruct_expression(input.value);
        let mut st = self.symbol_table.borrow_mut();

        if let (Expression::Identifier(var), Some(const_val)) = (&place, const_val) {
            st.set_variable(var.name, const_val);
        }

        Statement::Assign(Box::new(AssignStatement {
            operation: input.operation,
            place,
            value,
            span: input.span,
        }))
    }

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> Statement {
        let (value, const_val) = self.reconstruct_expression(input.value);
        let mut st = self.symbol_table.borrow_mut();

        if let Some(const_val) = const_val {
            input.variable_names.iter().for_each(|var| {
                st.set_variable(var.identifier.name, const_val.clone());
            });
        }

        Statement::Definition(DefinitionStatement {
            declaration_type: input.declaration_type,
            variable_names: input.variable_names.clone(),
            type_: input.type_,
            value,
            span: input.span,
        })
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> Statement {
        let (start_expr, start) = self.reconstruct_expression(input.start);
        let (stop_expr, stop) = self.reconstruct_expression(input.stop);

        match (start, stop) {
            (Some(start), Some(stop)) => {
                let cast_to_usize = |v: Value| -> Result<usize, Statement> {
                    match v.try_into() {
                        Ok(val_as_usize) => Ok(val_as_usize),
                        Err(err) => {
                            self.handler.emit_err(err);
                            Err(Statement::Block(Block {
                                statements: Vec::new(),
                                span: input.span,
                            }))
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

                let range = if start < stop {
                    if let Some(stop) = stop.checked_add(input.inclusive as usize) {
                        start..stop
                    } else {
                        self.handler
                            .emit_err(FlattenError::incorrect_loop_bound("stop", "usize::MAX + 1", input.span));
                        Default::default()
                    }
                } else if let Some(start) = start.checked_sub(input.inclusive as usize) {
                    stop..(start)
                } else {
                    self.handler
                        .emit_err(FlattenError::incorrect_loop_bound("start", "-1", input.span));
                    Default::default()
                };

                return Statement::Block(Block {
                    statements: range
                        .into_iter()
                        .map(|iter_var| {
                            self.symbol_table.borrow_mut().variables.insert(
                                input.variable.name,
                                VariableSymbol {
                                    type_: input.type_,
                                    span: input.variable.span,
                                    declaration: Declaration::Const(Some(Value::from_usize(
                                        input.type_,
                                        iter_var,
                                        input.variable.span,
                                    ))),
                                },
                            );

                            // TODO we should add new scopes for iteration.
                            let block = Statement::Block(Block {
                                statements: input
                                    .block
                                    .statements
                                    .clone()
                                    .into_iter()
                                    .map(|s| self.reconstruct_statement(s))
                                    .collect(),
                                span: input.block.span,
                            });

                            self.symbol_table.borrow_mut().variables.remove(&input.variable.name);

                            block
                        })
                        .collect(),
                    span: input.span,
                });
            }
            (None, Some(_)) => self
                .handler
                .emit_err(FlattenError::non_const_loop_bounds("stop", stop_expr.span())),
            (Some(_), None) => self
                .handler
                .emit_err(FlattenError::non_const_loop_bounds("start", start_expr.span())),
            (None, None) => {
                self.handler
                    .emit_err(FlattenError::non_const_loop_bounds("start", start_expr.span()));
                self.handler
                    .emit_err(FlattenError::non_const_loop_bounds("stop", stop_expr.span()));
            }
        }

        Statement::Block(Block {
            statements: Vec::new(),
            span: input.span,
        })
    }

    fn reconstruct_block(&mut self, input: Block) -> Block {
        let current_block = self.block_index;
        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table
            .swap(prev_st.borrow().get_block_scope(current_block).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));
        self.block_index = 0;

        let b = Block {
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
        self.block_index = current_block;
        self.block_index += 1;

        b
    }
}
