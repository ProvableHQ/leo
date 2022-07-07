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

use crate::{Declaration, TypeChecker, VariableSymbol};

use leo_ast::*;
use leo_errors::TypeCheckerError;

use std::cell::RefCell;

impl<'a> StatementVisitor<'a> for TypeChecker<'a> {
    fn visit_return(&mut self, input: &'a ReturnStatement) {
        // We can safely unwrap all self.parent instances because
        // `ReturnStatement`s should always have some parent block.
        let parent = self.parent.unwrap();

        let return_type = &self.symbol_table.borrow().lookup_fn(&parent).map(|f| f.output);
        self.check_core_type_conflict(return_type);
        self.has_return = true;

        self.visit_expression(&input.expression, return_type);
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) {
        let declaration = match input.declaration_type {
            Declare::Const => Declaration::Const,
            _ => Declaration::Mut,
        };

        input.variable_names.iter().for_each(|v| {
            self.check_core_type_conflict(&Some(input.type_));

            self.visit_expression(&input.value, &Some(input.type_));

            if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
                v.identifier.name,
                VariableSymbol {
                    type_: input.type_,
                    span: input.span(),
                    declaration: declaration.clone(),
                },
            ) {
                self.handler.emit_err(err);
            }
        });
    }

    fn visit_assign(&mut self, input: &'a AssignStatement) {
        let var_name = match input.place {
            Expression::Identifier(id) => id,
            _ => {
                self.handler
                    .emit_err(TypeCheckerError::invalid_assignment_target(input.place.span()));
                return;
            }
        };

        let var_type = if let Some(var) = self.symbol_table.borrow_mut().lookup_variable(&var_name.name) {
            // TODO: Check where this check is moved to in `improved-flattening`.
            match &var.declaration {
                Declaration::Const => self
                    .handler
                    .emit_err(TypeCheckerError::cannont_assign_to_const_var(var_name, var.span)),
                Declaration::Input(ParamMode::Const) => self
                    .handler
                    .emit_err(TypeCheckerError::cannont_assign_to_const_input(var_name, var.span)),
                _ => {}
            }

            Some(var.type_)
        } else {
            self.handler
                .emit_err(TypeCheckerError::unknown_sym("variable", var_name.name, var_name.span));

            None
        };

        if var_type.is_some() {
            self.check_core_type_conflict(&var_type);
            self.visit_expression(&input.value, &var_type);
        }
    }

    fn visit_conditional(&mut self, input: &'a ConditionalStatement) {
        self.visit_expression(&input.condition, &Some(Type::Boolean));
        self.visit_block(&input.block);
        if let Some(s) = input.next.as_ref() {
            self.visit_statement(s)
        }
    }

    fn visit_iteration(&mut self, input: &'a IterationStatement) {
        let iter_type = &Some(input.type_);
        self.check_core_type_conflict(iter_type);

        if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
            input.variable.name,
            VariableSymbol {
                type_: input.type_,
                span: input.span(),
                declaration: Declaration::Const,
            },
        ) {
            self.handler.emit_err(err);
        }

        self.visit_expression(&input.start, iter_type);
        self.visit_expression(&input.stop, iter_type);
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
        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table
            .swap(prev_st.borrow().get_block_scope(scope_index).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));

        input.statements.iter().for_each(|stmt| {
            match stmt {
                Statement::Return(stmt) => self.visit_return(stmt),
                Statement::Definition(stmt) => self.visit_definition(stmt),
                Statement::Assign(stmt) => self.visit_assign(stmt),
                Statement::Conditional(stmt) => self.visit_conditional(stmt),
                Statement::Iteration(stmt) => self.visit_iteration(stmt),
                Statement::Console(stmt) => self.visit_console(stmt),
                Statement::Block(stmt) => self.visit_block(stmt),
            };
        });

        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table.swap(prev_st.get_block_scope(scope_index).unwrap());
        self.symbol_table = RefCell::new(prev_st);
    }
}
