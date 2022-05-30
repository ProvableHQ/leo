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

use leo_ast::*;
use leo_errors::TypeCheckerError;

use crate::{Declaration, TypeChecker, VariableSymbol};

use super::director::Director;

impl<'a> StatementVisitor<'a> for TypeChecker<'a> {}

impl<'a> StatementVisitorDirector<'a> for Director<'a> {
    fn visit_return(&mut self, input: &'a ReturnStatement) {
        // we can safely unwrap all self.parent instances because
        // statements should always have some parent block
        let parent = self.visitor.parent.unwrap();

        self.visit_expression(
            &input.expression,
            &self.visitor.symbol_table.lookup_fn(&parent).map(|f| f.output),
        );
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) {
        let declaration = if input.declaration_type == Declare::Const {
            Declaration::Const
        } else {
            Declaration::Mut
        };

        input.variable_names.iter().for_each(|v| {
            if let Err(err) = self.visitor.symbol_table.insert_variable(
                v.identifier.name,
                VariableSymbol {
                    type_: &input.type_,
                    span: input.span(),
                    declaration: declaration.clone(),
                },
            ) {
                self.visitor.handler.emit_err(err);
            }

            self.visit_expression(&input.value, &Some(input.type_));
        });
    }

    fn visit_assign(&mut self, input: &'a AssignStatement) {
        let var_name = &input.assignee.identifier.name;
        let var_type = if let Some(var) = self.visitor.symbol_table.lookup_variable(var_name) {
            match &var.declaration {
                Declaration::Const => self
                    .visitor
                    .handler
                    .emit_err(TypeCheckerError::cannont_assign_to_const_var(var_name, var.span).into()),
                Declaration::Input(ParamMode::Constant) => self
                    .visitor
                    .handler
                    .emit_err(TypeCheckerError::cannont_assign_to_const_input(var_name, var.span).into()),
                _ => {}
            }

            Some(*var.type_)
        } else {
            self.visitor.handler.emit_err(
                TypeCheckerError::unknown_sym("variable", &input.assignee.identifier.name, input.assignee.span).into(),
            );

            None
        };

        if var_type.is_some() {
            self.visit_expression(&input.value, &var_type);
        }
    }

    fn visit_conditional(&mut self, input: &'a ConditionalStatement) {
        self.visit_expression(&input.condition, &Some(Type::Boolean));
    }

    fn visit_iteration(&mut self, input: &'a IterationStatement) {
        if let Err(err) = self.visitor.symbol_table.insert_variable(
            input.variable.name,
            VariableSymbol {
                type_: &input.type_,
                span: input.span(),
                declaration: Declaration::Const,
            },
        ) {
            self.visitor.handler.emit_err(err);
        }

        self.visit_expression(&input.start, &Some(input.type_));
        self.visit_expression(&input.stop, &Some(input.type_));
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
        self.visitor.symbol_table.push_variable_scope();
        // have to redo the logic here so we have scoping
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
        self.visitor.symbol_table.pop_variable_scope();
    }
}
