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

use super::*;
use crate::{ConditionalTreeNode, TypeChecker, VariableSymbol, VariableType};

use leo_ast::{
    Type::{Future, Tuple},
    *,
};
use leo_errors::TypeCheckerError;

use itertools::Itertools;

impl<'a, N: Network> StatementVisitor<'a> for TypeChecker<'a, N> {
    fn visit_statement(&mut self, input: &'a Statement) {
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

    fn visit_assert(&mut self, input: &'a AssertStatement) {
        match &input.variant {
            AssertVariant::Assert(expr) => {
                let type_ = self.visit_expression(expr, &Some(Type::Boolean));
                self.assert_bool_type(&type_, expr.span());
            }
            AssertVariant::AssertEq(left, right) | AssertVariant::AssertNeq(left, right) => {
                let t1 = self.visit_expression(left, &None);
                let t2 = self.visit_expression(right, &None);

                // Check that the types are equal.
                self.check_eq_types(&t1, &t2, input.span());
            }
        }
    }

    fn visit_assign(&mut self, input: &'a AssignStatement) {
        let var_name = match input.place {
            Expression::Identifier(id) => id,
            _ => {
                self.emit_err(TypeCheckerError::invalid_assignment_target(input.place.span()));
                return;
            }
        };

        // Lookup the variable in the symbol table and retrieve its type.
        let var_type = if let Some(var) = self.symbol_table.borrow().lookup_variable(Location::new(None, var_name.name))
        {
            // If the variable exists, then check that it is not a constant.
            match &var.declaration {
                VariableType::Const => self.emit_err(TypeCheckerError::cannot_assign_to_const_var(var_name, var.span)),
                VariableType::Input(Mode::Constant) => {
                    self.emit_err(TypeCheckerError::cannot_assign_to_const_input(var_name, var.span))
                }
                VariableType::Mut | VariableType::Input(_) => {}
            }

            // If the variable exists and its in an async function, then check that it is in the current scope.
            // Note that this unwrap is safe because the scope state is initalized before traversing the function.
            if self.scope_state.variant.unwrap().is_async_function()
                && self.scope_state.is_conditional
                && self
                    .symbol_table
                    .borrow()
                    .lookup_variable_in_current_scope(Location::new(None, var_name.name))
                    .is_none()
            {
                self.emit_err(TypeCheckerError::async_cannot_assign_outside_conditional(var_name, var.span));
            }
            // Prohibit reassignment of futures.
            if let Type::Future(_) = var.type_ {
                self.emit_err(TypeCheckerError::cannot_reassign_future_variable(var_name, var.span));
            }

            Some(var.type_.clone())
        } else {
            self.emit_err(TypeCheckerError::unknown_sym("variable", var_name.name, var_name.span));
            None
        };

        if var_type.is_some() {
            self.visit_expression(&input.value, &var_type);
        }
    }

    fn visit_block(&mut self, input: &'a Block) {
        // Create a new scope for the then-block.
        let scope_index = self.create_child_scope();

        input.statements.iter().for_each(|stmt| self.visit_statement(stmt));

        // Exit the scope for the then-block.
        self.exit_scope(scope_index);
    }

    fn visit_conditional(&mut self, input: &'a ConditionalStatement) {
        self.visit_expression(&input.condition, &Some(Type::Boolean));

        let mut then_block_has_return = false;
        let mut otherwise_block_has_return = false;

        // Set the `has_return` flag for the then-block.
        let previous_has_return = core::mem::replace(&mut self.scope_state.has_return, then_block_has_return);
        // Set the `is_conditional` flag.
        let previous_is_conditional = core::mem::replace(&mut self.scope_state.is_conditional, true);

        // Create scope for checking awaits in `then` branch of conditional.
        let current_bst_nodes: Vec<ConditionalTreeNode> = match self
            .await_checker
            .create_then_scope(self.scope_state.variant == Some(Variant::AsyncFunction), input.span)
        {
            Ok(nodes) => nodes,
            Err(warn) => return self.emit_warning(warn),
        };

        // Visit block.
        self.visit_block(&input.then);

        // Store the `has_return` flag for the then-block.
        then_block_has_return = self.scope_state.has_return;

        // Exit scope for checking awaits in `then` branch of conditional.
        let saved_paths = self
            .await_checker
            .exit_then_scope(self.scope_state.variant == Some(Variant::AsyncFunction), current_bst_nodes);

        if let Some(otherwise) = &input.otherwise {
            // Set the `has_return` flag for the otherwise-block.
            self.scope_state.has_return = otherwise_block_has_return;

            match &**otherwise {
                Statement::Block(stmt) => {
                    // Visit the otherwise-block.
                    self.visit_block(stmt);
                }
                Statement::Conditional(stmt) => self.visit_conditional(stmt),
                _ => unreachable!("Else-case can only be a block or conditional statement."),
            }

            // Store the `has_return` flag for the otherwise-block.
            otherwise_block_has_return = self.scope_state.has_return;
        }

        // Update the set of all possible BST paths.
        self.await_checker.exit_statement_scope(self.scope_state.variant == Some(Variant::AsyncFunction), saved_paths);

        // Restore the previous `has_return` flag.
        self.scope_state.has_return = previous_has_return || (then_block_has_return && otherwise_block_has_return);
        // Restore the previous `is_conditional` flag.
        self.scope_state.is_conditional = previous_is_conditional;
    }

    fn visit_console(&mut self, _: &'a ConsoleStatement) {
        unreachable!("Parsing guarantees that console statements are not present in the AST.");
    }

    fn visit_const(&mut self, input: &'a ConstDeclaration) {
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

        // Enforce that Constant variables have literal expressions on right-hand side
        match &input.value {
            Expression::Literal(_) => (),
            Expression::Tuple(tuple_expression) => match tuple_expression.elements.len() {
                0 | 1 => unreachable!("Parsing guarantees that tuple types have at least two elements."),
                _ => {
                    if tuple_expression.elements.iter().any(|expr| !matches!(expr, Expression::Literal(_))) {
                        self.emit_err(TypeCheckerError::const_declaration_must_be_literal_or_tuple_of_literals(
                            input.span,
                        ))
                    }
                }
            },
            _ => self.emit_err(TypeCheckerError::const_declaration_must_be_literal_or_tuple_of_literals(input.span())),
        }

        // Check the expression on the right-hand side.
        self.visit_expression(&input.value, &Some(input.type_.clone()));

        // Add constants to symbol table so that any references to them in later statements will pass TC
        if let Err(err) =
            self.symbol_table.borrow_mut().insert_variable(Location::new(None, input.place.name), VariableSymbol {
                type_: input.type_.clone(),
                span: input.place.span,
                declaration: VariableType::Const,
            })
        {
            self.handler.emit_err(err);
        }
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) {
        // Check that the type of the definition is defined.
        self.assert_type_is_valid(&input.type_, input.span);

        // Check that the type of the definition is not a unit type, singleton tuple type, or nested tuple type.
        match &input.type_ {
            // If the type is an empty tuple, return an error.
            Type::Unit => self.emit_err(TypeCheckerError::lhs_must_be_identifier_or_tuple(input.span)),
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
                self.insert_variable(inferred_type.clone(), identifier, input.type_.clone(), 0, identifier.span)
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

                for ((index, expr), type_) in
                    tuple_expression.elements.iter().enumerate().zip_eq(tuple_type.elements().iter())
                {
                    let identifier = match expr {
                        Expression::Identifier(identifier) => identifier,
                        _ => {
                            return self
                                .emit_err(TypeCheckerError::lhs_tuple_element_must_be_an_identifier(expr.span()));
                        }
                    };
                    self.insert_variable(inferred_type.clone(), identifier, type_.clone(), index, identifier.span);
                }
            }
            _ => self.emit_err(TypeCheckerError::lhs_must_be_identifier_or_tuple(input.place.span())),
        }
    }

    fn visit_expression_statement(&mut self, input: &'a ExpressionStatement) {
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

    fn visit_iteration(&mut self, input: &'a IterationStatement) {
        let iter_type = &Some(input.type_.clone());
        self.assert_int_type(iter_type, input.variable.span);

        // Create a new scope for the loop body.
        let scope_index = self.create_child_scope();

        // Add the loop variable to the scope of the loop body.
        if let Err(err) =
            self.symbol_table.borrow_mut().insert_variable(Location::new(None, input.variable.name), VariableSymbol {
                type_: input.type_.clone(),
                span: input.span(),
                declaration: VariableType::Const,
            })
        {
            self.handler.emit_err(err);
        }

        let prior_has_return = core::mem::take(&mut self.scope_state.has_return);
        let prior_has_finalize = core::mem::take(&mut self.scope_state.has_called_finalize);

        self.visit_block(&input.block);

        if self.scope_state.has_return {
            self.emit_err(TypeCheckerError::loop_body_contains_return(input.span()));
        }

        if self.scope_state.has_called_finalize {
            self.emit_err(TypeCheckerError::loop_body_contains_finalize(input.span()));
        }

        self.scope_state.has_return = prior_has_return;
        self.scope_state.has_called_finalize = prior_has_finalize;

        // Exit the scope.
        self.exit_scope(scope_index);

        // Check that the literal is valid.
        self.visit_expression(&input.start, iter_type);

        // If `input.start` is a valid literal, instantiate it as a value.
        match &input.start {
            Expression::Literal(literal) => {
                // Note that this check is needed because the pass attempts to make progress, even though the literal may be invalid.
                if let Ok(value) = Value::try_from(literal) {
                    input.start_value.replace(Some(value));
                }
            }
            Expression::Identifier(id) => {
                if let Some(var) = self.symbol_table.borrow().lookup_variable(Location::new(None, id.name)) {
                    if VariableType::Const != var.declaration {
                        self.emit_err(TypeCheckerError::loop_bound_must_be_literal_or_const(id.span));
                    }
                }
            }
            _ => self.emit_err(TypeCheckerError::loop_bound_must_be_literal_or_const(input.start.span())),
        }

        self.visit_expression(&input.stop, iter_type);

        // If `input.stop` is a valid literal, instantiate it as a value.
        match &input.stop {
            Expression::Literal(literal) => {
                // Note that this check is needed because the pass attempts to make progress, even though the literal may be invalid.
                if let Ok(value) = Value::try_from(literal) {
                    input.stop_value.replace(Some(value));
                }
            }
            Expression::Identifier(id) => {
                if let Some(var) = self.symbol_table.borrow().lookup_variable(Location::new(None, id.name)) {
                    if VariableType::Const != var.declaration {
                        self.emit_err(TypeCheckerError::loop_bound_must_be_literal_or_const(id.span));
                    }
                }
            }
            _ => self.emit_err(TypeCheckerError::loop_bound_must_be_literal_or_const(input.stop.span())),
        }
    }

    fn visit_return(&mut self, input: &'a ReturnStatement) {
        // We can safely unwrap all self.parent instances because
        // statements should always have some parent block
        let parent = self.scope_state.function.unwrap();
        let func =
            self.symbol_table.borrow().lookup_fn_symbol(Location::new(self.scope_state.program_name, parent)).cloned();
        let mut return_type = func.clone().map(|f| f.output_type.clone());

        // Fully type the expected return value.
        if self.scope_state.variant == Some(Variant::AsyncTransition) && self.scope_state.has_called_finalize {
            let inferred_future_type =
                match self.async_function_input_types.get(&func.unwrap().finalize.clone().unwrap()) {
                    Some(types) => Future(FutureType::new(
                        types.clone(),
                        Some(Location::new(self.scope_state.program_name, parent)),
                        true,
                    )),
                    None => {
                        return self
                            .emit_err(TypeCheckerError::async_transition_missing_future_to_return(input.span()));
                    }
                };
            // Need to modify return type since the function signature is just default future, but the actual return type is the fully inferred future of the finalize input type.
            let inferred = match return_type.clone() {
                Some(Future(_)) => Some(inferred_future_type),
                Some(Tuple(tuple)) => Some(Tuple(TupleType::new(
                    tuple
                        .elements()
                        .iter()
                        .map(|t| if matches!(t, Future(_)) { inferred_future_type.clone() } else { t.clone() })
                        .collect::<Vec<Type>>(),
                ))),
                _ => {
                    return self.emit_err(TypeCheckerError::async_transition_missing_future_to_return(input.span()));
                }
            };

            // Check that the explicit type declared in the function output signature matches the inferred type.
            if let Some(ty) = inferred {
                return_type = Some(self.assert_and_return_type(ty, &return_type, input.span()));
            }
        }

        // Set the `has_return` flag.
        self.scope_state.has_return = true;

        // Check that the return expression is not a nested tuple.
        if let Expression::Tuple(TupleExpression { elements, .. }) = &input.expression {
            for element in elements {
                if matches!(element, Expression::Tuple(_)) {
                    self.emit_err(TypeCheckerError::nested_tuple_expression(element.span()));
                }
            }
        }

        // Set the `is_return` flag. This is necessary to allow unit expressions in the return statement.
        self.scope_state.is_return = true;
        // Type check the associated expression.
        self.visit_expression(&input.expression, &return_type);
        // Unset the `is_return` flag.
        self.scope_state.is_return = false;
    }
}
