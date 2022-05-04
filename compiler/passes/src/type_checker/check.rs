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
use leo_errors::{Result, TypeCheckerError};
use leo_span::Span;

use crate::{Declaration, TypeChecker, VariableSymbol};

impl<'a> TypeChecker<'a> {
    fn compare_types(&self, t1: Result<Option<Type>>, t2: Result<Option<Type>>, span: &Span) {
        match (t1, t2) {
            (Ok(Some(t1)), Ok(Some(t2))) if t1 != t2 => self
                .handler
                .emit_err(TypeCheckerError::types_do_not_match(t1, t2, span).into()),
            (Ok(Some(t)), Ok(None)) | (Ok(None), Ok(Some(t))) => self
                .handler
                .emit_err(TypeCheckerError::type_expected_but_not_found(t, span).into()),
            (Err(err), Ok(None)) | (Ok(None), Err(err)) => self.handler.emit_err(err),
            (Err(err1), Err(err2)) => {
                self.handler.emit_err(err1);
                self.handler.emit_err(err2);
            }
            // Types match
            _ => {}
        }
    }

    fn assert_type(&self, type_: Result<Option<Type>>, expected: Type, span: &Span) {
        match type_ {
            Ok(Some(type_)) if type_ != expected => self
                .handler
                .emit_err(TypeCheckerError::type_should_be(type_, expected, span).into()),
            // Types match
            _ => {}
        }
    }
}

impl<'a> ExpressionVisitor<'a> for TypeChecker<'a> {}

// we can safely unwrap all self.parent instances because
// statements should always have some parent block
impl<'a> StatementVisitor<'a> for TypeChecker<'a> {
    fn visit_return(&mut self, input: &'a ReturnStatement) -> VisitResult {
        let parent = self.parent.unwrap();

        if let Some(func) = self.symbol_table.lookup_fn(&parent) {
            self.compare_types(func.get_type(), input.get_type(), input.span());
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

            self.compare_types(input.get_type(), input.get_type(), input.span());
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

            self.assert_type(input.value.get_type(), var.type_.clone(), input.span())
        } else {
            self.handler
                .emit_err(TypeCheckerError::unknown_assignee(&input.assignee.identifier.name, input.span()).into())
        }

        VisitResult::VisitChildren
    }

    fn visit_conditional(&mut self, input: &'a ConditionalStatement) -> VisitResult {
        self.assert_type(input.condition.get_type(), Type::Boolean, input.span());

        VisitResult::VisitChildren
    }

    fn visit_iteration(&mut self, input: &'a IterationStatement) -> VisitResult {
        // TODO: need to change symbol table to some other repr for variables.
        // self.symbol_table.insert_variable(input.variable.name, input);

        let iter_var_type = input.get_type();

        self.compare_types(iter_var_type.clone(), input.start.get_type(), input.span());
        self.compare_types(iter_var_type, input.stop.get_type(), input.span());

        VisitResult::VisitChildren
    }

    fn visit_console(&mut self, input: &'a ConsoleStatement) -> VisitResult {
        match &input.function {
            ConsoleFunction::Assert(expr) => {
                self.assert_type(expr.get_type(), Type::Boolean, expr.span());
            }
            ConsoleFunction::Error(_) | ConsoleFunction::Log(_) => {
                todo!("need to discuss this");
            }
        }

        VisitResult::VisitChildren
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

impl<'a> ProgramVisitor<'a> for TypeChecker<'a> {
    fn visit_function(&mut self, input: &'a Function) -> VisitResult {
        self.symbol_table.clear_variables();
        self.parent = Some(input.name());
        input.input.iter().for_each(|i| {
            let input_var = i.get_variable();

            if let Err(err) = self.symbol_table.insert_variable(
                input_var.identifier.name,
                VariableSymbol {
                    type_: &input_var.type_,
                    span: input_var.span(),
                    declaration: Declaration::Input(input_var.mode()),
                },
            ) {
                self.handler.emit_err(err);
            }
        });

        VisitResult::VisitChildren
    }
}
