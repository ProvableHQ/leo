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

//! This module contains Visitor trait implementations for the AST.
//! It implements default methods for each node to be made
//! given the type of node its visiting.

use std::marker::PhantomData;

use crate::*;

pub struct VisitorDirector<'a, V: ExpressionVisitor<'a>> {
    visitor: V,
    lifetime: PhantomData<&'a ()>,
}

impl<'a, V: ExpressionVisitor<'a>> VisitorDirector<'a, V> {
    pub fn new(visitor: V) -> Self {
        Self {
            visitor,
            lifetime: PhantomData,
        }
    }

    pub fn visitor(self) -> V {
        self.visitor
    }

    pub fn visit_expression(&mut self, input: &'a Expression) {
        if let VisitResult::VisitChildren = self.visitor.visit_expression(input) {
            match input {
                Expression::Identifier(_) => {}
                Expression::Value(_) => {}
                Expression::Binary(expr) => self.visit_binary(expr),
                Expression::Unary(expr) => self.visit_unary(expr),
                Expression::Ternary(expr) => self.visit_ternary(expr),
                Expression::Call(expr) => self.visit_call(expr),
                Expression::Err(_) => {}
            }
        }
    }

    pub fn visit_binary(&mut self, input: &'a BinaryExpression) {
        if let VisitResult::VisitChildren = self.visitor.visit_binary(input) {
            self.visit_expression(&input.left);
            self.visit_expression(&input.right);
        }
    }

    pub fn visit_unary(&mut self, input: &'a UnaryExpression) {
        if let VisitResult::VisitChildren = self.visitor.visit_unary(input) {
            self.visit_expression(&input.inner);
        }
    }

    pub fn visit_ternary(&mut self, input: &'a TernaryExpression) {
        if let VisitResult::VisitChildren = self.visitor.visit_ternary(input) {
            self.visit_expression(&input.condition);
            self.visit_expression(&input.if_true);
            self.visit_expression(&input.if_false);
        }
    }

    pub fn visit_call(&mut self, input: &'a CallExpression) {
        if let VisitResult::VisitChildren = self.visitor.visit_call(input) {
            input.arguments.iter().for_each(|expr| self.visit_expression(expr));
        }
    }
}

impl<'a, V: ExpressionVisitor<'a> + StatementVisitor<'a>> VisitorDirector<'a, V> {
    pub fn visit_statement(&mut self, input: &'a Statement) {
        if let VisitResult::VisitChildren = self.visitor.visit_statement(input) {
            match input {
                Statement::Return(stmt) => self.visit_return(stmt),
                Statement::Definition(stmt) => self.visit_definition(stmt),
                Statement::Assign(stmt) => self.visit_assign(stmt),
                Statement::Conditional(stmt) => self.visit_conditional(stmt),
                Statement::Iteration(stmt) => self.visit_iteration(stmt),
                Statement::Console(stmt) => self.visit_console(stmt),
                Statement::Expression(stmt) => self.visit_expression_statement(stmt),
                Statement::Block(stmt) => self.visit_block(stmt),
            }
        }
    }

    pub fn visit_return(&mut self, input: &'a ReturnStatement) {
        if let VisitResult::VisitChildren = self.visitor.visit_return(input) {
            self.visit_expression(&input.expression);
        }
    }

    pub fn visit_definition(&mut self, input: &'a DefinitionStatement) {
        if let VisitResult::VisitChildren = self.visitor.visit_definition(input) {
            self.visit_expression(&input.value);
        }
    }

    pub fn visit_assign(&mut self, input: &'a AssignStatement) {
        if let VisitResult::VisitChildren = self.visitor.visit_assign(input) {
            self.visit_expression(&input.value);
        }
    }

    pub fn visit_conditional(&mut self, input: &'a ConditionalStatement) {
        if let VisitResult::VisitChildren = self.visitor.visit_conditional(input) {
            self.visit_expression(&input.condition);
            self.visit_block(&input.block);
            if let Some(stmt) = input.next.as_ref() {
                self.visit_statement(stmt);
            }
        }
    }

    pub fn visit_iteration(&mut self, input: &'a IterationStatement) {
        if let VisitResult::VisitChildren = self.visitor.visit_iteration(input) {
            self.visit_expression(&input.start);
            self.visit_expression(&input.stop);
            self.visit_block(&input.block);
        }
    }

    pub fn visit_console(&mut self, input: &'a ConsoleStatement) {
        if let VisitResult::VisitChildren = self.visitor.visit_console(input) {
            match &input.function {
                ConsoleFunction::Assert(expr) => self.visit_expression(expr),
                ConsoleFunction::Error(fmt) | ConsoleFunction::Log(fmt) => {
                    fmt.parameters.iter().for_each(|expr| self.visit_expression(expr));
                }
            }
        }
    }

    pub fn visit_expression_statement(&mut self, input: &'a ExpressionStatement) {
        if let VisitResult::VisitChildren = self.visitor.visit_expression_statement(input) {
            self.visit_expression(&input.expression);
        }
    }

    pub fn visit_block(&mut self, input: &'a Block) {
        if let VisitResult::VisitChildren = self.visitor.visit_block(input) {
            input.statements.iter().for_each(|stmt| self.visit_statement(stmt));
        }
    }
}

impl<'a, V: ExpressionVisitor<'a> + ProgramVisitor<'a> + StatementVisitor<'a>> VisitorDirector<'a, V> {
    pub fn visit_program(&mut self, input: &'a Program) {
        if let VisitResult::VisitChildren = self.visitor.visit_program(input) {
            input
                .functions
                .values()
                .for_each(|function| self.visit_function(function));
        }
    }

    pub fn visit_function(&mut self, input: &'a Function) {
        if let VisitResult::VisitChildren = self.visitor.visit_function(input) {
            self.visit_block(&input.block);
        }
    }
}
