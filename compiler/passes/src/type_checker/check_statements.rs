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

impl<'a> StatementVisitor<'a> for TypeChecker<'a> {
    fn visit_return(&mut self, input: &'a ReturnStatement) -> VisitResult {
        // we can safely unwrap all self.parent instances because
        // statements should always have some parent block
        let parent = self.parent.unwrap();

        if let Some(func) = self.symbol_table.lookup_fn(&parent) {
            self.compare_expr_type(&input.expression, Some(func.output.clone()), input.expression.span());
        }

        VisitResult::VisitChildren
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) -> VisitResult {
        let declaration = if input.declaration_type == Declare::Const {
            Declaration::Const
        } else {
            Declaration::Mut
        };

        input.variable_names.iter().for_each(|v| {
            if let Err(err) = self.symbol_table.insert_variable(
                v.identifier.name,
                VariableSymbol {
                    type_: &input.type_,
                    span: input.span(),
                    declaration: declaration.clone(),
                },
            ) {
                self.handler.emit_err(err);
            }

            self.compare_expr_type(&input.value, Some(input.type_.clone()), input.value.span());
        });

        VisitResult::VisitChildren
    }

    fn visit_assign(&mut self, input: &'a AssignStatement) -> VisitResult {
        let var_name = &input.assignee.identifier.name;
        if let Some(var) = self.symbol_table.lookup_variable(var_name) {
            match &var.declaration {
                Declaration::Const => self
                    .handler
                    .emit_err(TypeCheckerError::cannont_assign_to_const_var(var_name, var.span).into()),
                Declaration::Input(ParamMode::Constant) => self
                    .handler
                    .emit_err(TypeCheckerError::cannont_assign_to_const_input(var_name, var.span).into()),
                _ => {}
            }

            self.compare_expr_type(&input.value, Some(var.type_.clone()), input.value.span());
        } else {
            self.handler.emit_err(
                TypeCheckerError::unknown_sym("variable", &input.assignee.identifier.name, &input.assignee.span).into(),
            )
        }

        VisitResult::VisitChildren
    }

    fn visit_conditional(&mut self, input: &'a ConditionalStatement) -> VisitResult {
        self.compare_expr_type(&input.condition, Some(Type::Boolean), input.condition.span());

        VisitResult::VisitChildren
    }

    fn visit_iteration(&mut self, input: &'a IterationStatement) -> VisitResult {
        if let Err(err) = self.symbol_table.insert_variable(
            input.variable.name,
            VariableSymbol {
                type_: &input.type_,
                span: input.span(),
                declaration: Declaration::Const,
            },
        ) {
            self.handler.emit_err(err);
        }

        self.compare_expr_type(&input.start, Some(input.type_.clone()), input.start.span());
        self.compare_expr_type(&input.stop, Some(input.type_.clone()), input.stop.span());

        VisitResult::VisitChildren
    }

    fn visit_console(&mut self, input: &'a ConsoleStatement) -> VisitResult {
        match &input.function {
            ConsoleFunction::Assert(expr) => {
                self.compare_expr_type(expr, Some(Type::Boolean), expr.span());
            }
            ConsoleFunction::Error(_) | ConsoleFunction::Log(_) => {
                // TODO: undetermined
            }
        }

        VisitResult::VisitChildren
    }

    fn visit_expression_statement(&mut self, _input: &'a ExpressionStatement) -> VisitResult {
        VisitResult::SkipChildren
    }

    fn visit_block(&mut self, input: &'a Block) -> VisitResult {
        self.symbol_table.push_variable_scope();
        // have to redo the logic here so we have scoping
        input.statements.iter().for_each(|stmt| {
            match stmt {
                Statement::Return(stmt) => self.visit_return(stmt),
                Statement::Definition(stmt) => self.visit_definition(stmt),
                Statement::Assign(stmt) => self.visit_assign(stmt),
                Statement::Conditional(stmt) => self.visit_conditional(stmt),
                Statement::Iteration(stmt) => self.visit_iteration(stmt),
                Statement::Console(stmt) => self.visit_console(stmt),
                Statement::Expression(stmt) => self.visit_expression_statement(stmt),
                Statement::Block(stmt) => self.visit_block(stmt),
            };
        });
        self.symbol_table.pop_variable_scope();

        VisitResult::SkipChildren
    }
}
