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
use leo_errors::TypeCheckerError;

use crate::{ConstantFolder, VariableSymbol, VariableType};

/// Returns the literal value if the value is const.
/// Otherwise returns the const.
fn map_const((expr, val): (Expression, Option<Value>)) -> Expression {
    val.map(|v| Expression::Literal(v.into())).unwrap_or(expr)
}

impl<'a> StatementReconstructor for ConstantFolder<'a> {
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
                            variable_type: match input.declaration_type {
                                DeclarationType::Const => VariableType::Const,
                                DeclarationType::Let => VariableType::Mut,
                            },
                            value: Some(const_val.clone()),
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
                        variable_type: match &input.declaration_type {
                            DeclarationType::Const => VariableType::Const,
                            DeclarationType::Let => VariableType::Mut,
                        },
                        value: None,
                    },
                ) {
                    self.handler.emit_err(err);
                }
            });
        } else if const_val.is_none() {
            input.variable_names.iter().for_each(|var| {
                // Overwrite the TYC value with NONE.
                st.variables.insert(
                    var.identifier.name,
                    VariableSymbol {
                        type_: input.type_,
                        span: var.identifier.span,
                        variable_type: match &input.declaration_type {
                            DeclarationType::Const => VariableType::Const,
                            DeclarationType::Let => VariableType::Mut,
                        },
                        value: None,
                    },
                );
            });
        }

        Statement::Definition(DefinitionStatement {
            declaration_type: input.declaration_type,
            variable_names: input.variable_names.clone(),
            type_: input.type_.clone(),
            value: map_const((value, const_val)),
            span: input.span,
        })
    }

    fn reconstruct_assign(&mut self, input: AssignStatement) -> Statement {
        // Gets the target and its const value
        let (place_expr, place_const) = self.reconstruct_expression(input.place);

        let var_name = match place_expr {
            Expression::Identifier(var) => var.name,
            _ => unreachable!("The LHS of an assignment must be an identifier."),
        };

        // If the target has a constant value, asserts that the target wasn't declared as constant
        if place_const.is_some() {
            if let Some(var) = self.symbol_table.borrow().lookup_variable(&var_name) {
                match &var.variable_type {
                    VariableType::Const => self.handler.emit_err(TypeCheckerError::cannot_assign_to_const_var(
                        var_name,
                        place_expr.span(),
                    )),
                    VariableType::Input(ParamMode::Const) => self.handler.emit_err(
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
        // TODO: Set the variable, no need for deconstification.
        if let Some(c) = const_val.clone() {
            // Find the value in a parent scope and updates it
            st.set_variable(&var_name, c);
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

        // TODO: in future if symbol table is used for other passes.
        // We will have to remove these scopes instead of skipping over them.
        match const_value {
            // If branch const true
            Some(Value::Boolean(true)) => {
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
            Some(Value::Boolean(false)) if input.next.is_some() => {
                self.block_index += 1;
                self.reconstruct_statement(*input.next.unwrap())
            }
            // If branch const false and no branch follows it
            Some(Value::Boolean(false)) => {
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
        }
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> Statement {
        let (start, start_value) = self.reconstruct_expression(input.start);
        let (stop, stop_value) = self.reconstruct_expression(input.stop);

        // TODO: Assign const values to `start_value` and `stop_value`.

        // We match on start and stop cause loops require
        // bounds to be constants.

        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table
            .swap(prev_st.borrow().get_block_scope(self.block_index).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));

        let block = self.reconstruct_block(input.block);

        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table
            .swap(prev_st.get_block_scope(self.block_index).unwrap());
        self.symbol_table = RefCell::new(prev_st);
        self.block_index = self.block_index + 1;

        Statement::Iteration(Box::new(IterationStatement {
            variable: input.variable,
            type_: input.type_,
            start,
            start_value: RefCell::new(start_value),
            stop,
            stop_value: RefCell::new(stop_value),
            block,
            inclusive: input.inclusive,
            span: input.span,
        }))
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
        // Enter block scope.
        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table
            .swap(prev_st.borrow().get_block_scope(self.block_index).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));

        let block = Block {
            statements: input
                .statements
                .into_iter()
                .map(|s| self.reconstruct_statement(s))
                .collect(),
            span: input.span,
        };

        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        // TODO: Is this swap necessary?
        self.symbol_table
            .swap(prev_st.get_block_scope(self.block_index).unwrap());
        self.symbol_table = RefCell::new(prev_st);
        self.block_index = self.block_index + 1;

        block
    }
}
