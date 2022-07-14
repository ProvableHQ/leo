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
        let (value, const_val) = self.reconstruct_expression(input.value.clone());
        let mut st = self.symbol_table.borrow_mut();

        if const_val.is_none() {
            // Check that the variable's declaration type is not constant.
            match input.declaration_type {
                DeclarationType::Let => {}
                DeclarationType::Const => {
                    self.handler.emit_err(TypeCheckerError::cannot_assign_to_const_var(
                        input.value,
                        input.variable_name,
                        input.span,
                    ));
                }
            }
        }

        st.set_value(input.variable_name.name, const_val.clone());


        Statement::Definition(DefinitionStatement {
            declaration_type: input.declaration_type,
            variable_name: input.variable_name,
            type_: input.type_,
            value,
            span: input.span,
        })
    }

    fn reconstruct_assign(&mut self, input: AssignStatement) -> Statement {
        // Gets the target and its const value
        let (place_expr, _) = self.reconstruct_expression(input.place);

        let variable_name = match place_expr {
            Expression::Identifier(var) => var.name,
            _ => unreachable!("The LHS of an assignment must be an identifier."),
        };

        // Reconstruct `input.value` and optionally compute its value.
        let (value, const_val) = self.reconstruct_expression(input.value);

        let mut st = self.symbol_table.borrow_mut();
        st.set_value(variable_name, const_val);

        if self.in_conditional {
            self.non_constant_variables.push(variable_name);
        }

        Statement::Assign(Box::new(AssignStatement {
            operation: input.operation,
            place: place_expr,
            value,
            span: input.span,
        }))
    }

    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> Statement {
        // Flattens the condition and gets its expression and possible const value
        let (condition, const_value) = self.reconstruct_expression(input.condition);

        // TODO: Consider zeroing out the scopes that are never traversed, instead of removing them.
        match const_value {
            // If `input.condition` is `true`, the conditional statement is replaced with the `if` branch.
            Some(Value::Boolean(true)) => {
                let block = Statement::Block(self.reconstruct_block(input.block));

                // Remove `input.next` from the symbol table.
                let mut st = self.symbol_table.borrow_mut();
                let mut next = input.next;
                while next.is_some() {
                    st.remove_scope(self.scope_index);
                    match *next.unwrap() {
                        // If `input.next` is a `ConditionalStatement`, we need to remove both the `if` and `else` blocks.
                        // Since the AST may contain a chain of ConditionalStatements, we must iteratively remove blocks until no further `ConditionalStatement`s are found.
                        Statement::Conditional(c) => next = c.next,
                        Statement::Block(..) => next = None,
                        _ => unreachable!(
                            "The next statement of a conditional statement must be a conditional statement or a block."
                        ),
                    }
                }

                block
            }
            Some(Value::Boolean(false)) => {
                match input.next.is_some() {
                    // If `input.condition` is `false` and there is `next` branch, the conditional statement is replaced with a reconstructed `input.next`.
                    true => {
                        // Remove the scope associated with `input.block` since it is never traversed.
                        let mut st = self.symbol_table.borrow_mut();
                        st.remove_scope(self.scope_index);

                        // Drop the mutable reference to the symbol table as it is no longer needed.
                        // We do this to avoid a borrow error (E0502).
                        drop(st);

                        match *input.next.unwrap() {
                            Statement::Block(block) => self.reconstruct_statement(Statement::Block(block)),
                            Statement::Conditional(conditional) => {
                                // Store the prior state of `ConstantFolder`.
                                let prior_in_conditional = std::mem::replace(&mut self.in_conditional, true);
                                let prior_non_constant_variables = std::mem::take(&mut self.non_constant_variables);

                                // Reconstruct the `else` branch.
                                let result = self.reconstruct_statement(Statement::Conditional(conditional));

                                // Restore the prior state of `ConstantFolder`.
                                self.in_conditional = prior_in_conditional;
                                let variables_to_be_cleared = std::mem::replace(&mut self.non_constant_variables, prior_non_constant_variables);

                                // Unset all values that were written to during the conditional.
                                let mut st = self.symbol_table.borrow_mut();
                                variables_to_be_cleared.iter().for_each(|v| st.unset_value(v));

                                result
                            }
                            _ => unreachable!("The next statement of a conditional statement must be a block or a conditional statement."),
                        }
                    }

                    // If `input.condition` is `false` and there is no `next` branch, the conditional statement is replaced with an empty block statement.
                    false => {
                        // Clear the scope associated with `input.block` since it is never traversed.
                        match self.symbol_table.borrow_mut().lookup_scope_by_index(self.scope_index) {
                            Some(scope) => {
                                scope.borrow_mut().clear();
                                self.scope_index += 1
                            }
                            _ => unreachable!("The scope associated with `input.block` must exist."),
                        }

                        Statement::Block(Block {
                            statements: Vec::new(),
                            span: input.span,
                        })
                    }
                }
            }
            // If `input.condition` is not a condition, then visit both branches.
            _ => {
                // Store the prior state of `ConstantFolder`.
                let prior_in_conditional = std::mem::replace(&mut self.in_conditional, true);
                let prior_non_constant_variables = std::mem::take(&mut self.non_constant_variables);

                let block = self.reconstruct_block(input.block);
                let next = input.next.map(|n| Box::new(self.reconstruct_statement(*n)));

                // Restore the prior state of `ConstantFolder`.
                self.in_conditional = prior_in_conditional;

                // Unset all values that were written to during the conditional.
                let variables_to_be_cleared =
                    std::mem::replace(&mut self.non_constant_variables, prior_non_constant_variables);
                let mut st = self.symbol_table.borrow_mut();
                variables_to_be_cleared.iter().for_each(|v| st.unset_value(v));

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

        let block = self.reconstruct_block(input.block);

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
        let current_scope = self.scope_index;

        // Enter block scope.
        self.enter_block_scope(current_scope);
        self.scope_index = 0;

        let block = Block {
            statements: input
                .statements
                .into_iter()
                .map(|s| self.reconstruct_statement(s))
                .collect(),
            span: input.span,
        };

        // Exit block scope.
        self.exit_block_scope(current_scope);
        self.scope_index = current_scope + 1;

        block
    }
}
