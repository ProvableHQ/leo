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

use crate::{TypeChecker, VariableSymbol, VariableType};

use leo_ast::{
    Type::{Future, Tuple},
    *,
};
use leo_errors::TypeCheckerError;

impl StatementVisitor for TypeChecker<'_> {
    fn visit_statement(&mut self, input: &Statement) {
        // No statements can follow a return statement.
        if self.scope_state.has_return {
            self.emit_err(TypeCheckerError::unreachable_code_after_return(input.span()));
            return;
        }

        match input {
            Statement::Assert(stmt) => self.visit_assert(stmt),
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Console(stmt) => self.visit_console(stmt),
            Statement::Const(stmt) => self.visit_const(stmt),
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Expression(stmt) => self.visit_expression_statement(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Return(stmt) => self.visit_return(stmt),
        }
    }

    fn visit_assert(&mut self, input: &AssertStatement) {
        match &input.variant {
            AssertVariant::Assert(expr) => {
                let _type = self.visit_expression(expr, &Some(Type::Boolean));
            }
            AssertVariant::AssertEq(left, right) | AssertVariant::AssertNeq(left, right) => {
                let t1 = self.visit_expression(left, &None);
                let t2 = self.visit_expression(right, &None);

                if t1 != Type::Err && t2 != Type::Err && !self.eq_user(&t1, &t2) {
                    let op =
                        if matches!(input.variant, AssertVariant::AssertEq(..)) { "assert_eq" } else { "assert_neq" };
                    self.emit_err(TypeCheckerError::operation_types_mismatch(op, t1, t2, input.span()));
                }
            }
        }
    }

    fn visit_assign(&mut self, input: &AssignStatement) {
        let Expression::Identifier(var_name) = input.place else {
            self.emit_err(TypeCheckerError::invalid_assignment_target(input.place.span()));
            return;
        };

        // Lookup the variable in the symbol table and retrieve its type.
        let Some(var) = self.symbol_table.lookup_variable(self.scope_state.program_name.unwrap(), var_name.name) else {
            self.emit_err(TypeCheckerError::unknown_sym("variable", var_name.name, var_name.span));
            return;
        };

        // If the variable exists, then check that it is not a constant.
        match &var.declaration {
            VariableType::Const => self.emit_err(TypeCheckerError::cannot_assign_to_const_var(var_name, var.span)),
            VariableType::Input(Mode::Constant) => {
                self.emit_err(TypeCheckerError::cannot_assign_to_const_input(var_name, var.span))
            }
            VariableType::Mut | VariableType::Input(_) => {}
        }

        // If the variable exists and it's in an async function, then check that it is in the current conditional scope.
        if self.scope_state.variant.unwrap().is_async_function() && !self.symbol_in_conditional_scope(var_name.name) {
            self.emit_err(TypeCheckerError::async_cannot_assign_outside_conditional(var_name, var.span));
        }
        // Prohibit reassignment of futures.
        if let Type::Future(_) = var.type_ {
            self.emit_err(TypeCheckerError::cannot_reassign_future_variable(var_name, var.span));
        }

        self.visit_expression(&input.value, &Some(var.type_.clone()));
    }

    fn visit_block(&mut self, input: &Block) {
        self.in_scope(input.id, |slf| {
            input.statements.iter().for_each(|stmt| slf.visit_statement(stmt));
        });
    }

    fn visit_conditional(&mut self, input: &ConditionalStatement) {
        self.visit_expression(&input.condition, &Some(Type::Boolean));

        let mut then_block_has_return = false;
        let mut otherwise_block_has_return = false;

        // Set the `has_return` flag for the then-block.
        let previous_has_return = core::mem::replace(&mut self.scope_state.has_return, then_block_has_return);
        // Set the `is_conditional` flag.
        let previous_is_conditional = core::mem::replace(&mut self.scope_state.is_conditional, true);

        // Visit block.
        self.in_conditional_scope(|slf| slf.visit_block(&input.then));

        // Store the `has_return` flag for the then-block.
        then_block_has_return = self.scope_state.has_return;

        if let Some(otherwise) = &input.otherwise {
            // Set the `has_return` flag for the otherwise-block.
            self.scope_state.has_return = otherwise_block_has_return;

            match &**otherwise {
                Statement::Block(stmt) => {
                    // Visit the otherwise-block.
                    self.in_conditional_scope(|slf| slf.visit_block(stmt));
                }
                Statement::Conditional(stmt) => self.visit_conditional(stmt),
                _ => unreachable!("Else-case can only be a block or conditional statement."),
            }

            // Store the `has_return` flag for the otherwise-block.
            otherwise_block_has_return = self.scope_state.has_return;
        }

        // Restore the previous `has_return` flag.
        self.scope_state.has_return = previous_has_return || (then_block_has_return && otherwise_block_has_return);
        // Restore the previous `is_conditional` flag.
        self.scope_state.is_conditional = previous_is_conditional;
    }

    fn visit_console(&mut self, _: &ConsoleStatement) {
        unreachable!("Parsing guarantees that console statements are not present in the AST.");
    }

    fn visit_const(&mut self, input: &ConstDeclaration) {
        // Check that the type of the definition is not a unit type, singleton tuple type, or nested tuple type.
        match &input.type_ {
            // If the type is an empty tuple, return an error.
            Type::Unit => self.emit_err(TypeCheckerError::lhs_must_be_identifier_or_tuple(input.span)),
            // If the type is a singleton tuple, return an error.
            Type::Tuple(tuple) => match tuple.length() {
                0 | 1 => unreachable!("Parsing guarantees that tuple types have at least two elements."),
                _ => {
                    if tuple.elements().iter().any(|type_| matches!(type_, Type::Tuple(_))) {
                        self.emit_err(TypeCheckerError::nested_tuple_type(input.span))
                    }
                }
            },
            Type::Mapping(_) | Type::Err => unreachable!(
                "Parsing guarantees that `mapping` and `err` types are not present at this location in the AST."
            ),
            // Otherwise, the type is valid.
            _ => (), // Do nothing
        }

        // Check the expression on the right-hand side.
        self.visit_expression(&input.value, &Some(input.type_.clone()));

        // Add constants to symbol table so that any references to them in later statements will pass type checking.
        if let Err(err) = self.symbol_table.insert_variable(
            self.scope_state.program_name.unwrap(),
            input.place.name,
            VariableSymbol { type_: input.type_.clone(), span: input.place.span, declaration: VariableType::Const },
        ) {
            self.handler.emit_err(err);
        }
    }

    fn visit_definition(&mut self, input: &DefinitionStatement) {
        // Check that the type of the definition is defined.
        self.assert_type_is_valid(&input.type_, input.span);

        // Check that the type of the definition is not a unit type, singleton tuple type, or nested tuple type.
        match &input.type_ {
            // If the type is a singleton tuple, return an error.
            Type::Tuple(tuple) => match tuple.length() {
                0 | 1 => unreachable!("Parsing guarantees that tuple types have at least two elements."),
                _ => {
                    for type_ in tuple.elements() {
                        if matches!(type_, Type::Tuple(_)) {
                            self.emit_err(TypeCheckerError::nested_tuple_type(input.span))
                        }
                    }
                }
            },
            Type::Mapping(_) | Type::Err => unreachable!(
                "Parsing guarantees that `mapping` and `err` types are not present at this location in the AST."
            ),
            // Otherwise, the type is valid.
            _ => (), // Do nothing
        }

        // Check the expression on the right-hand side.
        let inferred_type = self.visit_expression(&input.value, &Some(input.type_.clone()));

        // Insert the variables into the symbol table.
        match &input.place {
            Expression::Identifier(identifier) => {
                self.insert_variable(Some(inferred_type.clone()), identifier, input.type_.clone(), identifier.span);
            }
            Expression::Tuple(tuple_expression) => {
                let tuple_type = match &input.type_ {
                    Type::Tuple(tuple_type) => tuple_type,
                    _ => unreachable!(
                        "Type checking guarantees that if the lhs is a tuple, its associated type is also a tuple."
                    ),
                };
                if tuple_expression.elements.len() != tuple_type.length() {
                    return self.emit_err(TypeCheckerError::incorrect_num_tuple_elements(
                        tuple_expression.elements.len(),
                        tuple_type.length(),
                        input.place.span(),
                    ));
                }

                for i in 0..tuple_expression.elements.len() {
                    let inferred = if let Type::Tuple(inferred_tuple) = &inferred_type {
                        inferred_tuple.elements().get(i).cloned().unwrap_or_default()
                    } else {
                        Type::Err
                    };
                    let expr = &tuple_expression.elements[i];
                    let identifier = match expr {
                        Expression::Identifier(identifier) => identifier,
                        _ => {
                            return self
                                .emit_err(TypeCheckerError::lhs_tuple_element_must_be_an_identifier(expr.span()));
                        }
                    };
                    self.insert_variable(Some(inferred), identifier, tuple_type.elements()[i].clone(), identifier.span);
                }
            }
            _ => self.emit_err(TypeCheckerError::lhs_must_be_identifier_or_tuple(input.place.span())),
        }
    }

    fn visit_expression_statement(&mut self, input: &ExpressionStatement) {
        // Expression statements can only be function calls.
        if !matches!(
            input.expression,
            Expression::Call(_) | Expression::Access(AccessExpression::AssociatedFunction(_))
        ) {
            self.emit_err(TypeCheckerError::expression_statement_must_be_function_call(input.span()));
        } else {
            // Check the expression.
            self.visit_expression(&input.expression, &None);
        }
    }

    fn visit_iteration(&mut self, input: &IterationStatement) {
        self.assert_int_type(&input.type_, input.variable.span);

        self.in_scope(input.id(), |slf| {
            // Add the loop variable to the scope of the loop body.
            if let Err(err) = slf.symbol_table.insert_variable(
                slf.scope_state.program_name.unwrap(),
                input.variable.name,
                VariableSymbol { type_: input.type_.clone(), span: input.span(), declaration: VariableType::Const },
            ) {
                slf.handler.emit_err(err);
            }

            let prior_has_return = core::mem::take(&mut slf.scope_state.has_return);
            let prior_has_finalize = core::mem::take(&mut slf.scope_state.has_called_finalize);

            slf.visit_block(&input.block);

            if slf.scope_state.has_return {
                slf.emit_err(TypeCheckerError::loop_body_contains_return(input.span()));
            }

            if slf.scope_state.has_called_finalize {
                slf.emit_err(TypeCheckerError::loop_body_contains_finalize(input.span()));
            }

            slf.scope_state.has_return = prior_has_return;
            slf.scope_state.has_called_finalize = prior_has_finalize;
        });

        self.visit_expression(&input.start, &Some(input.type_.clone()));

        self.visit_expression(&input.stop, &Some(input.type_.clone()));
    }

    fn visit_return(&mut self, input: &ReturnStatement) {
        let func_name = self.scope_state.function.unwrap();
        let func_symbol = self
            .symbol_table
            .lookup_function(Location::new(self.scope_state.program_name.unwrap(), func_name))
            .expect("The symbol table creator should already have visited all functions.");
        let mut return_type = func_symbol.function.output_type.clone();

        // Fully type the expected return value.
        if self.scope_state.variant == Some(Variant::AsyncTransition) && self.scope_state.has_called_finalize {
            let inferred_future_type = Future(FutureType::new(
                func_symbol.finalizer.as_ref().unwrap().inferred_inputs.clone(),
                Some(Location::new(self.scope_state.program_name.unwrap(), func_name)),
                true,
            ));

            // Need to modify return type since the function signature is just default future, but the actual return type is the fully inferred future of the finalize input type.
            let inferred = match return_type.clone() {
                Future(_) => inferred_future_type,
                Tuple(tuple) => Tuple(TupleType::new(
                    tuple
                        .elements()
                        .iter()
                        .map(|t| if matches!(t, Future(_)) { inferred_future_type.clone() } else { t.clone() })
                        .collect::<Vec<Type>>(),
                )),
                _ => {
                    return self.emit_err(TypeCheckerError::async_transition_missing_future_to_return(input.span()));
                }
            };

            // Check that the explicit type declared in the function output signature matches the inferred type.
            return_type = self.assert_and_return_type(inferred, &Some(return_type), input.span());
        }

        // Set the `has_return` flag.
        self.scope_state.has_return = true;

        self.visit_expression(&input.expression, &Some(return_type));
    }
}
