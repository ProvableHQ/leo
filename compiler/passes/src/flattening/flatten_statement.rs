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
use leo_errors::{FlattenError, TypeCheckerError};

use crate::{Declaration, Flattener, Value, VariableSymbol};

fn map_const((expr, val): (Expression, Option<Value>)) -> Expression {
    val.map(|v| Expression::Literal(v.into())).unwrap_or(expr)
}

impl<'a> StatementReconstructor for Flattener<'a> {
    fn reconstruct_return(&mut self, input: ReturnStatement) -> Statement {
        Statement::Return(ReturnStatement {
            expression: map_const(self.reconstruct_expression(input.expression)),
            span: input.span,
        })
    }

    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> Statement {
        let (value, const_val) = self.reconstruct_expression(input.value);
        let mut st = self.symbol_table.borrow_mut();

        if let Some(const_val) = const_val.clone() {
            input.variable_names.iter().for_each(|var| {
                if !st.set_variable(&var.identifier.name, const_val.clone()) {
                    if let Err(err) = st.insert_variable(
                        var.identifier.name,
                        VariableSymbol {
                            type_: (&const_val).into(),
                            span: var.identifier.span,
                            declaration: match &input.declaration_type {
                                Declare::Const => Declaration::Const(Some(const_val.clone())),
                                Declare::Let => Declaration::Mut(Some(const_val.clone())),
                            },
                        },
                    ) {
                        self.handler.emit_err(err);
                    }
                }
            });
        } else if const_val.is_none() && self.create_iter_scopes {
            input.variable_names.iter().for_each(|var| {
                if let Err(err) = st.insert_variable(
                    var.identifier.name,
                    VariableSymbol {
                        type_: input.type_,
                        span: var.identifier.span,
                        declaration: match &input.declaration_type {
                            Declare::Const => Declaration::Const(None),
                            Declare::Let => Declaration::Mut(None),
                        },
                    },
                ) {
                    self.handler.emit_err(err);
                }
            });
        }

        Statement::Definition(DefinitionStatement {
            declaration_type: input.declaration_type,
            variable_names: input.variable_names.clone(),
            type_: input.type_,
            value: map_const((value, const_val)),
            span: input.span,
        })
    }

    fn reconstruct_assign(&mut self, input: AssignStatement) -> Statement {
        let (place_expr, place_const) = self.reconstruct_expression(input.place);
        let var_name = if let Expression::Identifier(var) = place_expr {
            var.name
        } else {
            unreachable!()
        };

        if place_const.is_some() {
            if let Some(var) = self.symbol_table.borrow().lookup_variable(&var_name) {
                match &var.declaration {
                    Declaration::Const(_) => self.handler.emit_err(TypeCheckerError::cannot_assign_to_const_var(
                        var_name,
                        place_expr.span(),
                    )),
                    Declaration::Input(_, ParamMode::Const) => self.handler.emit_err(
                        TypeCheckerError::cannot_assign_to_const_input(var_name, place_expr.span()),
                    ),
                    _ => {}
                }
            }
        }

        let (value, const_val) = self.reconstruct_expression(input.value);

        let mut st = self.symbol_table.borrow_mut();
        let var_in_local = st.variable_in_local_scope(&var_name);

        if let Some(c) = const_val.clone() {
            if !self.non_const_block || var_in_local {
                st.set_variable(&var_name, c);
            } else {
                st.deconstify_variable(&var_name);
                st.locally_constify_variable(var_name, c);
            }
        } else if const_val.is_none() {
            st.deconstify_variable(&var_name);
        }

        Statement::Assign(Box::new(AssignStatement {
            operation: input.operation,
            place: place_expr,
            value: map_const((value, const_val)),
            span: input.span,
        }))
    }

    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> Statement {
        let (condition, const_value) = self.reconstruct_expression(input.condition);

        let prev_non_const_block = self.non_const_block;
        self.non_const_block = const_value.is_none() || prev_non_const_block;

        // TODO: in future if symbol table is used for other passes.
        // We will have to remove these scopes instead of skipping over them.
        let out = match const_value {
            Some(Value::Boolean(true, _)) => {
                let block = Statement::Block(self.reconstruct_block(input.block));
                if input.next.is_some() {
                    self.block_index += 1;
                }
                let mut next = input.next;
                while let Some(Statement::Conditional(c)) = next.as_deref() {
                    if c.next.is_some() {
                        self.block_index += 1;
                    }
                    next = c.next.clone();
                }

                block
            }
            Some(Value::Boolean(false, _)) if input.next.is_some() => {
                self.block_index += 1;
                self.reconstruct_statement(*input.next.unwrap())
            }
            Some(Value::Boolean(false, _)) => {
                self.block_index += 1;
                // TODO: creates empty block, should instead figure out how to return none here.
                Statement::Block(Block {
                    statements: Vec::new(),
                    span: input.span,
                })
            }
            _ => {
                let block = self.reconstruct_block(input.block);
                let next = input.next.map(|n| Box::new(self.reconstruct_statement(*n)));
                Statement::Conditional(ConditionalStatement {
                    condition,
                    block,
                    next,
                    span: input.span,
                })
            }
        };

        self.non_const_block = prev_non_const_block;
        out
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

                let scope_index = if self.create_iter_scopes {
                    self.symbol_table.borrow_mut().insert_block()
                } else {
                    self.block_index
                };
                let prev_st = std::mem::take(&mut self.symbol_table);
                self.symbol_table
                    .swap(prev_st.borrow().get_block_scope(scope_index).unwrap());
                self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));
                self.block_index = 0;

                let iter_blocks = Statement::Block(Block {
                    statements: range
                        .into_iter()
                        .map(|iter_var| {
                            let scope_index = self.symbol_table.borrow_mut().insert_block();
                            let prev_st = std::mem::take(&mut self.symbol_table);
                            self.symbol_table
                                .swap(prev_st.borrow().get_block_scope(scope_index).unwrap());
                            self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));

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

                            self.create_iter_scopes = true;
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
                            self.create_iter_scopes = false;

                            self.symbol_table.borrow_mut().variables.remove(&input.variable.name);

                            let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
                            self.symbol_table.swap(prev_st.get_block_scope(scope_index).unwrap());
                            self.symbol_table = RefCell::new(prev_st);

                            block
                        })
                        .collect(),
                    span: input.span,
                });
                let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
                self.symbol_table.swap(prev_st.get_block_scope(scope_index).unwrap());
                self.symbol_table = RefCell::new(prev_st);
                self.block_index = scope_index + 1;

                return iter_blocks;
            }
            (None, Some(_)) => self
                .handler
                .emit_err(FlattenError::non_const_loop_bounds("start", start_expr.span())),
            (Some(_), None) => self
                .handler
                .emit_err(FlattenError::non_const_loop_bounds("stop", stop_expr.span())),
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

    fn reconstruct_console(&mut self, input: ConsoleStatement) -> Statement {
        Statement::Console(ConsoleStatement {
            function: match input.function {
                ConsoleFunction::Assert(expr) => ConsoleFunction::Assert(map_const(self.reconstruct_expression(expr))),
                ConsoleFunction::Error(fmt) => ConsoleFunction::Error(ConsoleArgs {
                    string: fmt.string,
                    parameters: fmt
                        .parameters
                        .into_iter()
                        .map(|p| map_const(self.reconstruct_expression(p)))
                        .collect(),
                    span: fmt.span,
                }),
                ConsoleFunction::Log(fmt) => ConsoleFunction::Log(ConsoleArgs {
                    string: fmt.string,
                    parameters: fmt
                        .parameters
                        .into_iter()
                        .map(|p| map_const(self.reconstruct_expression(p)))
                        .collect(),
                    span: fmt.span,
                }),
            },
            span: input.span,
        })
    }

    fn reconstruct_block(&mut self, input: Block) -> Block {
        let current_block = if self.create_iter_scopes {
            self.symbol_table.borrow_mut().insert_block()
        } else {
            self.block_index
        };

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
        self.block_index = current_block + 1;

        b
    }
}
