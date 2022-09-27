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

use crate::{TypeChecker, VariableSymbol, VariableType};

use leo_ast::*;
use leo_errors::TypeCheckerError;

impl<'a> StatementVisitor<'a> for TypeChecker<'a> {
    fn visit_statement(&mut self, input: &'a Statement) {
        // No statements can follow a return statement.
        if self.has_return {
            self.emit_err(TypeCheckerError::unreachable_code_after_return(input.span()));
            return;
        }

        match input {
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Console(stmt) => self.visit_console(stmt),
            Statement::Decrement(stmt) => self.visit_decrement(stmt),
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Finalize(stmt) => self.visit_finalize(stmt),
            Statement::Increment(stmt) => self.visit_increment(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Return(stmt) => self.visit_return(stmt),
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

        let var_type = if let Some(var) = self.symbol_table.borrow_mut().lookup_variable(var_name.name) {
            match &var.declaration {
                VariableType::Const => self.emit_err(TypeCheckerError::cannot_assign_to_const_var(var_name, var.span)),
                VariableType::Input(Mode::Const) => {
                    self.emit_err(TypeCheckerError::cannot_assign_to_const_input(var_name, var.span))
                }
                _ => {}
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

        let mut then_block_has_finalize = false;
        let mut otherwise_block_has_finalize = false;

        // Set the `has_return` flag for the then-block.
        let previous_has_return = core::mem::replace(&mut self.has_return, then_block_has_return);
        // Set the `has_finalize` flag for the then-block.
        let previous_has_finalize = core::mem::replace(&mut self.has_finalize, then_block_has_finalize);

        self.visit_block(&input.then);

        // Store the `has_return` flag for the then-block.
        then_block_has_return = self.has_return;
        // Store the `has_finalize` flag for the then-block.
        then_block_has_finalize = self.has_finalize;

        if let Some(otherwise) = &input.otherwise {
            // Set the `has_return` flag for the otherwise-block.
            self.has_return = otherwise_block_has_return;
            // Set the `has_finalize` flag for the otherwise-block.
            self.has_finalize = otherwise_block_has_finalize;

            match &**otherwise {
                Statement::Block(stmt) => {
                    // Visit the otherwise-block.
                    self.visit_block(stmt);
                }
                Statement::Conditional(stmt) => self.visit_conditional(stmt),
                _ => unreachable!("Else-case can only be a block or conditional statement."),
            }

            // Store the `has_return` flag for the otherwise-block.
            otherwise_block_has_return = self.has_return;
            // Store the `has_finalize` flag for the otherwise-block.
            otherwise_block_has_finalize = self.has_finalize;
        }

        // Restore the previous `has_return` flag.
        self.has_return = previous_has_return || (then_block_has_return && otherwise_block_has_return);
        // Restore the previous `has_finalize` flag.
        self.has_finalize = previous_has_finalize || (then_block_has_finalize && otherwise_block_has_finalize);
    }

    fn visit_console(&mut self, input: &'a ConsoleStatement) {
        match &input.function {
            ConsoleFunction::Assert(expr) => {
                let type_ = self.visit_expression(expr, &Some(Type::Boolean));
                self.assert_bool_type(&type_, expr.span());
            }
            ConsoleFunction::AssertEq(left, right) | ConsoleFunction::AssertNeq(left, right) => {
                let t1 = self.visit_expression(left, &None);
                let t2 = self.visit_expression(right, &None);

                // Check that the types are equal.
                self.check_eq_types(&t1, &t2, input.span());
            }
        }
    }

    fn visit_decrement(&mut self, input: &'a DecrementStatement) {
        if !self.is_finalize {
            self.emit_err(TypeCheckerError::increment_or_decrement_outside_finalize(input.span()));
        }

        // Assert that the first operand is a mapping.
        let mapping_type = self.visit_identifier(&input.mapping, &None);
        self.assert_mapping_type(&mapping_type, input.span());

        match mapping_type {
            None => self.emit_err(TypeCheckerError::could_not_determine_type(
                input.mapping,
                input.mapping.span,
            )),
            Some(Type::Mapping(mapping_type)) => {
                // Check that the index matches the key type of the mapping.
                let index_type = self.visit_expression(&input.index, &None);
                self.assert_type(&index_type, &mapping_type.key, input.index.span());

                // Check that the amount matches the value type of the mapping.
                let amount_type = self.visit_expression(&input.amount, &None);
                self.assert_type(&amount_type, &mapping_type.value, input.amount.span());

                // Check that the amount type is incrementable.
                self.assert_field_group_scalar_int_type(&amount_type, input.amount.span());
            }
            Some(mapping_type) => self.emit_err(TypeCheckerError::expected_one_type_of(
                "mapping",
                mapping_type,
                input.mapping.span,
            )),
        }
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) {
        let declaration = if input.declaration_type == DeclarationType::Const {
            VariableType::Const
        } else {
            VariableType::Mut
        };

        // Check that the type of the definition is valid.
        self.assert_type_is_valid(input.span, &input.type_);

        self.visit_expression(&input.value, &Some(input.type_.clone()));

        if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
            input.variable_name.name,
            VariableSymbol {
                type_: input.type_.clone(),
                span: input.span(),
                declaration,
            },
        ) {
            self.handler.emit_err(err);
        }
    }

    fn visit_finalize(&mut self, input: &'a FinalizeStatement) {
        if self.is_finalize {
            self.emit_err(TypeCheckerError::finalize_in_finalize(input.span()));
        }

        // Set the `has_finalize` flag.
        self.has_finalize = true;

        // Check that the function has a finalize block.
        // Note that `self.function.unwrap()` is safe since every `self.function` is set for every function.
        // Note that `(self.function.unwrap()).unwrap()` is safe since all functions have been checked to exist.
        let finalize = self
            .symbol_table
            .borrow()
            .lookup_fn_symbol(self.function.unwrap())
            .unwrap()
            .finalize
            .clone();
        match finalize {
            None => self.emit_err(TypeCheckerError::finalize_without_finalize_block(input.span())),
            Some(finalize) => {
                // Check number of function arguments.
                if finalize.input.len() != input.arguments.len() {
                    self.emit_err(TypeCheckerError::incorrect_num_args_to_finalize(
                        finalize.input.len(),
                        input.arguments.len(),
                        input.span(),
                    ));
                }

                // Check function argument types.
                finalize
                    .input
                    .iter()
                    .zip(input.arguments.iter())
                    .for_each(|(expected, argument)| {
                        self.visit_expression(argument, &Some(expected.type_()));
                    });
            }
        }
    }

    fn visit_increment(&mut self, input: &'a IncrementStatement) {
        if !self.is_finalize {
            self.emit_err(TypeCheckerError::increment_or_decrement_outside_finalize(input.span()));
        }

        // Assert that the first operand is a mapping.
        let mapping_type = self.visit_identifier(&input.mapping, &None);
        self.assert_mapping_type(&mapping_type, input.span());

        match mapping_type {
            None => self.emit_err(TypeCheckerError::could_not_determine_type(
                input.mapping,
                input.mapping.span,
            )),
            Some(Type::Mapping(mapping_type)) => {
                // Check that the index matches the key type of the mapping.
                let index_type = self.visit_expression(&input.index, &None);
                self.assert_type(&index_type, &mapping_type.key, input.index.span());

                // Check that the amount matches the value type of the mapping.
                let amount_type = self.visit_expression(&input.amount, &None);
                self.assert_type(&amount_type, &mapping_type.value, input.amount.span());

                // Check that the amount type is incrementable.
                self.assert_field_group_scalar_int_type(&amount_type, input.amount.span());
            }
            Some(mapping_type) => self.emit_err(TypeCheckerError::expected_one_type_of(
                "mapping",
                mapping_type,
                input.mapping.span,
            )),
        }
    }

    fn visit_iteration(&mut self, input: &'a IterationStatement) {
        let iter_type = &Some(input.type_.clone());
        self.assert_int_type(iter_type, input.variable.span);

        // Create a new scope for the loop body.
        let scope_index = self.create_child_scope();

        // Add the loop variable to the scope of the loop body.
        if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
            input.variable.name,
            VariableSymbol {
                type_: input.type_.clone(),
                span: input.span(),
                declaration: VariableType::Const,
            },
        ) {
            self.handler.emit_err(err);
        }

        let prior_has_return = core::mem::take(&mut self.has_return);
        let prior_has_finalize = core::mem::take(&mut self.has_finalize);

        self.visit_block(&input.block);

        if self.has_return {
            self.emit_err(TypeCheckerError::loop_body_contains_return(input.span()));
        }

        if self.has_finalize {
            self.emit_err(TypeCheckerError::loop_body_contains_finalize(input.span()));
        }

        self.has_return = prior_has_return;
        self.has_finalize = prior_has_finalize;

        // Exit the scope.
        self.exit_scope(scope_index);

        self.visit_expression(&input.start, iter_type);

        // If `input.start` is a literal, instantiate it as a value.
        if let Expression::Literal(literal) = &input.start {
            input.start_value.replace(Some(Value::from(literal)));
        }

        self.visit_expression(&input.stop, iter_type);

        // If `input.stop` is a literal, instantiate it as a value.
        if let Expression::Literal(literal) = &input.stop {
            input.stop_value.replace(Some(Value::from(literal)));
        }
    }

    fn visit_return(&mut self, input: &'a ReturnStatement) {
        // we can safely unwrap all self.parent instances because
        // statements should always have some parent block
        let parent = self.function.unwrap();
        let return_type = &self
            .symbol_table
            .borrow()
            .lookup_fn_symbol(parent)
            .map(|f| match self.is_finalize {
                // TODO: Check this.
                // Note that this `unwrap()` is safe since we checked that the function has a finalize block.
                true => f.finalize.as_ref().unwrap().output_type.clone(),
                false => f.output_type.clone(),
            });

        self.has_return = true;

        self.visit_expression(&input.expression, return_type);
    }
}
