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

/// Returns the literal value if the value is const.
/// Otherwise returns the const.
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
        // We grab the place and its possible const value.
        let (value, const_val) = self.reconstruct_expression(input.value);
        let mut st = self.symbol_table.borrow_mut();

        // If it has a const value, we can assign constantly to it.
        if let Some(const_val) = const_val.clone() {
            input.variable_names.iter().for_each(|var| {
                // This sets the variable to the new constant value if it exists.
                if !st.set_variable(&var.identifier.name, const_val.clone()) {
                    // Otherwise we insert it.
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
            // Otherwise if const_value is none or we are in a iteration scope.
            // We always insert the variable but do not try to update it.
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
        // Gets the target and its const value
        let (place_expr, place_const) = self.reconstruct_expression(input.place);
        let var_name = if let Expression::Identifier(var) = place_expr {
            var.name
        } else {
            unreachable!()
        };

        // If the target has a constant value, asserts that the target wasn't declared as constant
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

        // Gets the rhs value and its possible const value
        let (value, const_val) = self.reconstruct_expression(input.value);

        let mut st = self.symbol_table.borrow_mut();
        let var_in_local = st.variable_in_local_scope(&var_name);

        // Sets the variable in scope as needed and returns if the value should be deconstified or not
        let deconstify = if let Some(c) = const_val.clone() {
            if !self.non_const_block || var_in_local {
                // Find the value in a parent scope and updates it
                st.set_variable(&var_name, c);
                false
            } else {
                // SHADOWS the variable with a constant in the local scope
                st.locally_constify_variable(var_name, c);
                true
            }
        } else {
            true
        };

        match &mut self.deconstify_buffer {
            // If deconstify buffer exists, value is set to deconstify,
            // And the value is not locally declared then slates the value for deconstification at the end of scope
            Some(buf) if deconstify && !var_in_local => buf.push(var_name),
            // immediately deconstifies value in all parent scopes
            _ if deconstify => st.deconstify_variable(&var_name),
            _ => {}
        }

        Statement::Assign(Box::new(AssignStatement {
            operation: input.operation,
            place: place_expr,
            value: map_const((value, const_val)),
            span: input.span,
        }))
    }

    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> Statement {
        // Flattens the condition and gets its expression and possible const value
        let (condition, const_value) = self.reconstruct_expression(input.condition);

        // Stores any current const buffer so it doesn't get cleared during a child scope
        let prev_buffered = self.deconstify_buffer.replace(Vec::new());
        // Stores the flag for the blocks global constyness and updates the flag with the current blocks const-ness
        let prev_non_const_block = self.non_const_block;
        self.non_const_block = const_value.is_none() || prev_non_const_block;
        // Stores the flag for the blocks local constyness, and updates the flag with the current blocks const-ness
        let prev_non_const_flag = self.next_block_non_const;
        self.next_block_non_const = const_value.is_none() || prev_non_const_flag;

        // TODO: in future if symbol table is used for other passes.
        // We will have to remove these scopes instead of skipping over them.
        let out = match const_value {
            // If branch const true
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
            // If branch const false and another branch follows this one
            Some(Value::Boolean(false, _)) if input.next.is_some() => {
                self.block_index += 1;
                self.reconstruct_statement(*input.next.unwrap())
            }
            // If branch const false and no branch follows it
            Some(Value::Boolean(false, _)) => {
                self.block_index += 1;
                Statement::Block(Block {
                    statements: Vec::new(),
                    span: input.span,
                })
            }
            // If conditional is non-const
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

        // Clears out any values that were slated for deconstification at end of conditional
        self.deconstify_buffered();
        // Restores previous buffers/flags
        self.deconstify_buffer = prev_buffered;
        self.non_const_block = prev_non_const_block;
        self.next_block_non_const = prev_non_const_flag;
        out
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> Statement {
        let (start_expr, start) = self.reconstruct_expression(input.start);
        let (stop_expr, stop) = self.reconstruct_expression(input.stop);

        // We match on start and stop cause loops require
        // bounds to be constants.
        match (start, stop) {
            (Some(start), Some(stop)) => {
                // Closure to check constant value is valid usize.
                // We already know these are integers because of tyc pass happened.
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

                // Create iteration range accounting for inclusive bounds.
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

                // Create the iteration scope if it does not exist.
                // Otherwise grab the existing one.
                let scope_index = if self.create_iter_scopes {
                    self.symbol_table.borrow_mut().insert_block(self.next_block_non_const)
                } else {
                    self.block_index
                };
                // Stores local const flag
                let prev_non_const_flag = self.next_block_non_const;
                // Iterations are always locally constant inside.
                self.next_block_non_const = false;
                let prev_st = std::mem::take(&mut self.symbol_table);
                self.symbol_table
                    .swap(prev_st.borrow().get_block_scope(scope_index).unwrap());
                self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));
                self.block_index = 0;

                // Create a block statement to replace the iteration statement.
                // Creates a new block per iteration inside the outer block statement.
                let iter_blocks = Statement::Block(Block {
                    statements: range
                        .into_iter()
                        .map(|iter_var| {
                            let scope_index = self.symbol_table.borrow_mut().insert_block(self.next_block_non_const);
                            let prev_st = std::mem::take(&mut self.symbol_table);
                            self.symbol_table
                                .swap(prev_st.borrow().get_block_scope(scope_index).unwrap());
                            self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));

                            // Insert the loop variable as a constant variable in the scope as its current value.
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

                            let prev_create_iter_scopes = self.create_iter_scopes;
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
                            self.create_iter_scopes = prev_create_iter_scopes;

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
                self.next_block_non_const = prev_non_const_flag;

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
        // If we are in an iteration scope we create any sub scopes for it.
        // This is because in TYC we remove all its sub scopes to avoid clashing variables
        // during flattening.
        let current_block = if self.create_iter_scopes {
            self.symbol_table.borrow_mut().insert_block(self.next_block_non_const)
        } else {
            self.block_index
        };

        // Store previous block's local constyness.
        let prev_non_const_flag = self.next_block_non_const;
        self.next_block_non_const = false;

        // Enter block scope.
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
        self.next_block_non_const = prev_non_const_flag;

        b
    }
}
