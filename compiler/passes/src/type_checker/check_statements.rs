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

use crate::{Declaration, TypeChecker, VariableSymbol};

use super::type_output::TypeOutput;

impl<'a> StatementVisitor<'a> for TypeChecker<'a> {
    fn visit_return(&mut self, input: &'a ReturnStatement) {
        // we can safely unwrap all self.parent instances because
        // statements should always have some parent block
        let parent = self.parent.unwrap();

        let return_type = &self.symbol_table.borrow().lookup_fn(&parent).map(|f| f.type_);
        self.validate_ident_type(return_type);
        self.has_return = true;

        self.visit_expression(&input.expression, return_type);
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) {
        input.variable_names.iter().for_each(|v| {
            self.validate_ident_type(&Some(input.type_));

            let output = self.visit_expression(&input.value, &Some(input.type_));

            let var = VariableSymbol {
                type_: input.type_,
                span: input.span(),
                declaration: match output {
                    TypeOutput::Const(c) | TypeOutput::Lit(c) if input.declaration_type.is_const() => {
                        Declaration::Const(Some(c))
                    }
                    TypeOutput::MutType(_) if input.declaration_type.is_const() => {
                        self.handler
                            .emit_err(TypeCheckerError::cannot_define_const_with_non_const(
                                v.identifier.name,
                                input.span,
                            ));
                        Declaration::Const(None)
                    }
                    TypeOutput::Const(c) | TypeOutput::Lit(c) | TypeOutput::Mut(c) => Declaration::Mut(Some(c)),
                    TypeOutput::ConstType(_) | TypeOutput::LitType(_) | TypeOutput::MutType(_)
                        if input.declaration_type.is_const() =>
                    {
                        Declaration::Const(None)
                    }
                    _ => Declaration::Mut(None),
                },
            };

            if let Err(err) = self.symbol_table.borrow_mut().insert_variable(v.identifier.name, var) {
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

        let var_type = if let Some(var) = self.symbol_table.borrow_mut().lookup_variable_mut(&var_name.name) {
            // TODO: would be good to update variable value here. that way were not left with out-of-date variable info used in expressions during tyc
            Some(var.type_)
        } else {
            self.handler
                .emit_err(TypeCheckerError::unknown_sym("variable", var_name.name, var_name.span));

            None
        };

        if var_type.is_some() {
            self.validate_ident_type(&var_type);
        }
    }

    fn visit_conditional(&mut self, input: &'a ConditionalStatement) {
        let output = self.visit_expression(&input.condition, &Some(Type::Boolean));
        let prev_block_non_const_flag = self.next_block_non_const;
        self.next_block_non_const = !output.is_const();

        self.visit_block(&input.block);
        if let Some(s) = input.next.as_ref() {
            self.visit_statement(s)
        }
        self.next_block_non_const = prev_block_non_const_flag;
    }

    fn visit_iteration(&mut self, input: &'a IterationStatement) {
        let iter_type = &Some(input.type_);
        self.assert_int_type(iter_type, input.variable.span);

        self.validate_ident_type(iter_type);
        let inserted = self.symbol_table.borrow_mut().insert_variable(
            input.variable.name,
            VariableSymbol {
                type_: input.type_,
                span: input.span(),
                declaration: Declaration::Const(None),
            },
        );

        let scope_index = self.symbol_table.borrow_mut().insert_block();
        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table
            .swap(prev_st.borrow().get_block_scope(scope_index).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));
        let prev_block_non_const_flag = self.next_block_non_const;
        self.next_block_non_const = false;

        input.block.statements.iter().for_each(|s| self.visit_statement(s));
        // Keep the iteration scope but empty it .
        // As it will just become a block of block scopes later on.
        // Makes it easier to avoid conflicts.
        self.symbol_table.borrow_mut().variables.clear();
        self.symbol_table.borrow_mut().scopes.clear();
        self.symbol_table.borrow_mut().scope_index = 0;
        self.next_block_non_const = prev_block_non_const_flag;

        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table.swap(prev_st.get_block_scope(scope_index).unwrap());
        self.symbol_table = RefCell::new(prev_st);

        if let Err(err) = inserted {
            self.handler.emit_err(err);
        } else {
            self.symbol_table.borrow_mut().variables.remove(&input.variable.name);
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
        // creates a new sub-scope since we are in a block.
        let scope_index = self.symbol_table.borrow_mut().insert_block();
        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table
            .swap(prev_st.borrow().get_block_scope(scope_index).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));
        // sets the local const status of the current block based on the flag passed from the above block
        let prev_block_const_flag = self.next_block_non_const;
        self.symbol_table.borrow_mut().is_locally_non_const = prev_block_const_flag;
        self.next_block_non_const = false;

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
        self.next_block_non_const = prev_block_const_flag;
    }
}
