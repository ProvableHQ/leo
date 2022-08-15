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

use std::cell::RefCell;

impl<'a> StatementVisitor<'a> for TypeChecker<'a> {
    fn visit_statement(&mut self, input: &'a Statement) {
        if self.has_return {
            self.emit_err(TypeCheckerError::unreachable_code_after_return(input.span()));
            return;
        }

        match input {
            Statement::Return(stmt) => self.visit_return(stmt),
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Console(stmt) => self.visit_console(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
        }
    }

    fn visit_return(&mut self, input: &'a ReturnStatement) {
        // we can safely unwrap all self.parent instances because
        // statements should always have some parent block
        let parent = self.parent.unwrap();
        let return_type = &self
            .symbol_table
            .borrow()
            .lookup_fn_symbol(parent)
            .map(|f| f.output.clone());

        self.has_return = true;

        self.visit_expression(&input.expression, return_type);
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
                VariableType::Input(ParamMode::Const) => {
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

    fn visit_conditional(&mut self, input: &'a ConditionalStatement) {
        self.visit_expression(&input.condition, &Some(Type::Boolean));

        let mut then_block_has_return = false;
        let mut otherwise_block_has_return = false;

        // Set the `has_return` flag for the then-block.
        let previous_has_return = core::mem::replace(&mut self.has_return, then_block_has_return);

        self.visit_block(&input.then);

        // Store the `has_return` flag for the then-block.
        then_block_has_return = self.has_return;

        if let Some(s) = input.otherwise.as_ref() {
            // Set the `has_return` flag for the otherwise-block.
            self.has_return = otherwise_block_has_return;

            self.visit_statement(s);

            // Store the `has_return` flag for the otherwise-block.
            otherwise_block_has_return = self.has_return;
        }

        // Restore the previous `has_return` flag.
        self.has_return = previous_has_return || (then_block_has_return && otherwise_block_has_return);
    }

    fn visit_iteration(&mut self, input: &'a IterationStatement) {
        let iter_type = &Some(input.type_.clone());
        self.assert_int_type(iter_type, input.variable.span);

        // Create a new scope for the loop body.
        let scope_index = self.symbol_table.borrow_mut().insert_block();
        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table
            .swap(prev_st.borrow().lookup_scope_by_index(scope_index).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));

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

        input.block.statements.iter().for_each(|s| self.visit_statement(s));

        if self.has_return {
            self.emit_err(TypeCheckerError::loop_body_contains_return(input.span()));
        }

        self.has_return = prior_has_return;

        // Restore the previous scope.
        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table
            .swap(prev_st.lookup_scope_by_index(scope_index).unwrap());
        self.symbol_table = RefCell::new(prev_st);

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

    fn visit_console(&mut self, input: &'a ConsoleStatement) {
        match &input.function {
            ConsoleFunction::Assert(expr) => {
                self.visit_expression(expr, &Some(Type::Boolean));
            }
            ConsoleFunction::Error(_) | ConsoleFunction::Log(_) => {
                // TODO: undetermined
            }
        }
    }

    fn visit_block(&mut self, input: &'a Block) {
        // Creates a new sub-scope since we are in a block.
        let scope_index = self.symbol_table.borrow_mut().insert_block();
        let previous_symbol_table = std::mem::take(&mut self.symbol_table);
        self.symbol_table.swap(
            previous_symbol_table
                .borrow()
                .lookup_scope_by_index(scope_index)
                .unwrap(),
        );
        self.symbol_table.borrow_mut().parent = Some(Box::new(previous_symbol_table.into_inner()));

        input.statements.iter().for_each(|stmt| self.visit_statement(stmt));

        let previous_symbol_table = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table
            .swap(previous_symbol_table.lookup_scope_by_index(scope_index).unwrap());
        self.symbol_table = RefCell::new(previous_symbol_table);
    }
}
