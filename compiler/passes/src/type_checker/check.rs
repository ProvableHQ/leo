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
use leo_errors::Result;

use crate::TypeChecker;

impl<'a> TypeChecker<'a> {
    fn compare_types(&self, t1: Result<Option<Type>>, t2: Result<Option<Type>>) {
        match (t1, t2) {
            (Ok(Some(t1)), Ok(Some(t2))) if t1 != t2 => self.handler.emit_err(todo!()),
            (Ok(Some(t)), Ok(None)) | (Ok(None), Ok(Some(t))) => self.handler.emit_err(todo!()),
            (Err(err), Ok(None)) | (Ok(None), Err(err)) => self.handler.emit_err(err),
            (Err(err1), Err(err2)) => {
                self.handler.emit_err(err1);
                self.handler.emit_err(err2);
            }
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
            self.compare_types(func.get_type(), input.get_type());
        }

        VisitResult::VisitChildren
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) -> VisitResult {
        input.variable_names.iter().for_each(|v| {
            if let Err(err) = self.symbol_table.insert_variable(v.identifier.name, input) {
                self.handler.emit_err(err);
            }
        });

        VisitResult::VisitChildren
    }

    fn visit_assign(&mut self, input: &'a AssignStatement) -> VisitResult {
        let input_var = self.symbol_table.lookup_fn_input(&input.assignee.identifier.name);
        let var = self.symbol_table.lookup_var(&input.assignee.identifier.name);

        match (input_var, var) {
            (Some(var), None) => {
                self.compare_types(var.get_type(), input.value.get_type());
            }
            (None, Some(var)) => {
                self.compare_types(var.get_type(), input.value.get_type());
            }
            (None, None) => self.handler.emit_err(todo!()),
            // Don't have to error here as shadowing checks are done during insertions.
            _ => {}
        }

        Default::default()
    }

    fn visit_conditional(&mut self, _input: &'a ConditionalStatement) -> VisitResult {
        Default::default()
    }

    fn visit_iteration(&mut self, _input: &'a IterationStatement) -> VisitResult {
        Default::default()
    }

    fn visit_console(&mut self, _input: &'a ConsoleStatement) -> VisitResult {
        Default::default()
    }

    fn visit_expression_statement(&mut self, _input: &'a ExpressionStatement) -> VisitResult {
        Default::default()
    }

    fn visit_block(&mut self, _input: &'a Block) -> VisitResult {
        Default::default()
    }
}

impl<'a> ProgramVisitor<'a> for TypeChecker<'a> {
    fn visit_function(&mut self, input: &'a Function) -> VisitResult {
        self.symbol_table.clear_variables();
        self.parent = Some(input.name());
        input.input.iter().for_each(|i| {
            let symbol = i.get_variable().identifier.name;
            if let Err(err) = self.symbol_table.insert_fn_input(symbol, i) {
                self.handler.emit_err(err);
            }
        });

        VisitResult::VisitChildren
    }
}
