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

        // If the rhs of the DefinitionStatement is a constant value, store it in the symbol table.
        if let Some(const_val) = const_val.clone() {
            match st.variable_in_local_scope(input.variable_name.name) {
                // If the variable is in the current scope, update it's value.
                true => {
                    st.set_value_in_local_scope(input.variable_name.name, Some(const_val));
                }
                // If the variable is not in the current scope, create a new entry with the appropriate value.
                false => {
                    // Note that we do not need to check for shadowing since type checking has already taken place.
                    st.insert_variable_unchecked(
                        input.variable_name.name,
                        VariableSymbol {
                            type_: Type::from(&const_val),
                            span: input.variable_name.span,
                            variable_type: match input.declaration_type {
                                DeclarationType::Const => VariableType::Const,
                                DeclarationType::Let => VariableType::Mut,
                            },
                            value: Some(const_val),
                        },
                    )
                }
            }
        } else {
            // TODO: Uncomment when constant folding is implemented for all types.
            // // Check that the variable's declaration type is not const.
            // match input.declaration_type {
            //     DeclarationType::Let => {},
            //     DeclarationType::Const => {
            //         self.handler.emit_err(TypeCheckerError::cannot_assign_to_const_var(var, var.span()));
            //     }
            // }
        }

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

        let var_name = match place_expr {
            Expression::Identifier(var) => var.name,
            _ => unreachable!("The LHS of an assignment must be an identifier."),
        };

        // Reconstruct `input.value` and optionally compute its value.
        let (value, const_val) = self.reconstruct_expression(input.value);

        let mut st = self.symbol_table.borrow_mut();

        // Update the value associated with `input.place`.
        // Note that this may turn a previously constant value into a non-constant value.
        match st.variable_in_local_scope(var_name) {
            // If the variable is in the current scope, update it's value.
            true => {
                st.set_value_in_local_scope(var_name, const_val);
            }
            // If the variable is not in the current scope, create a new entry with the appropriate value.
            // Note that we create a new entry even if the variable is defined in the parent scope. This is
            // necessary to ensure that symbol table contains the correct constant value at this point in the program.
            false => {
                // Lookup the variable in the parent scope.
                let (type_, variable_type) = {
                    let variable_symbol = st
                        .lookup_variable(var_name)
                        .expect("Variable should exist in parent scope.");
                    (variable_symbol.type_.clone(), variable_symbol.variable_type.clone())
                };
                // Note that we do not need to obey shadowing rules since type checking has already taken place.
                st.insert_variable_unchecked(
                    var_name,
                    VariableSymbol {
                        type_,
                        span: place_expr.span(),
                        variable_type,
                        value: const_val,
                    },
                )
            }
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

        // TODO: in future if symbol table is used for other passes.
        // We will have to remove these scopes instead of skipping over them.
        match const_value {
            // If branch const true
            Some(Value::Boolean(true)) => {
                let block = Statement::Block(self.reconstruct_block(input.block));
                if input.next.is_some() {
                    self.scope_index += 1;
                }
                let mut next = input.next;
                while let Some(Statement::Conditional(c)) = next.as_deref() {
                    if c.next.is_some() {
                        self.scope_index += 1;
                    }
                    next = c.next.clone();
                }

                block
            }
            // If branch const false and another branch follows this one
            Some(Value::Boolean(false)) if input.next.is_some() => {
                self.scope_index += 1;
                self.reconstruct_statement(*input.next.unwrap())
            }
            // If branch const false and no branch follows it
            Some(Value::Boolean(false)) => {
                self.scope_index += 1;
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
