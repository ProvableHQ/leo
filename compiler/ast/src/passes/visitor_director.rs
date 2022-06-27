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

use crate::*;

pub trait VisitorDirector<'a> {
    type Visitor: ExpressionVisitor<'a> + ProgramVisitor<'a> + StatementVisitor<'a>;

    fn visitor(self) -> Self::Visitor;

    fn visitor_ref(&mut self) -> &mut Self::Visitor;
}

pub trait ExpressionVisitorDirector<'a>: VisitorDirector<'a> {
    type AdditionalInput: Default;
    type Output;

    fn visit_expression(&mut self, input: &'a Expression, additional: &Self::AdditionalInput) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_expression(input) {
            match input {
                Expression::Access(expr) => self.visit_access(expr, additional),
                Expression::Identifier(expr) => self.visit_identifier(expr, additional),
                Expression::Literal(expr) => self.visit_literal(expr, additional),
                Expression::Binary(expr) => self.visit_binary(expr, additional),
                Expression::Call(expr) => self.visit_call(expr, additional),
                Expression::CircuitInit(expr) => self.visit_circuit_init(expr, additional),
                Expression::Err(expr) => self.visit_err(expr, additional),
                Expression::Ternary(expr) => self.visit_ternary(expr, additional),
                Expression::Unary(expr) => self.visit_unary(expr, additional),
            };
        }

        None
    }

    fn visit_identifier(&mut self, input: &'a Identifier, _additional: &Self::AdditionalInput) -> Option<Self::Output> {
        self.visitor_ref().visit_identifier(input);
        None
    }

    fn visit_literal(
        &mut self,
        input: &'a LiteralExpression,
        _additional: &Self::AdditionalInput,
    ) -> Option<Self::Output> {
        self.visitor_ref().visit_literal(input);
        None
    }

    fn visit_access(
        &mut self,
        input: &'a AccessExpression,
        additional: &Self::AdditionalInput,
    ) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_access(input) {
            match input {
                AccessExpression::Member(member) => return self.visit_expression(&member.inner, additional),
                AccessExpression::AssociatedConstant(_member) => {}
                AccessExpression::AssociatedFunction(member) => {
                    member.args.iter().for_each(|expr| {
                        self.visit_expression(expr, additional);
                    });
                }
            };
        }
        None
    }

    fn visit_binary(
        &mut self,
        input: &'a BinaryExpression,
        additional: &Self::AdditionalInput,
    ) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_binary(input) {
            self.visit_expression(&input.left, additional);
            self.visit_expression(&input.right, additional);
        }
        None
    }

    fn visit_unary(&mut self, input: &'a UnaryExpression, additional: &Self::AdditionalInput) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_unary(input) {
            self.visit_expression(&input.receiver, additional);
        }
        None
    }

    fn visit_ternary(
        &mut self,
        input: &'a TernaryExpression,
        additional: &Self::AdditionalInput,
    ) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_ternary(input) {
            self.visit_expression(&input.condition, additional);
            self.visit_expression(&input.if_true, additional);
            self.visit_expression(&input.if_false, additional);
        }
        None
    }

    fn visit_call(&mut self, input: &'a CallExpression, additional: &Self::AdditionalInput) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_call(input) {
            input.arguments.iter().for_each(|expr| {
                self.visit_expression(expr, additional);
            });
        }
        None
    }

    fn visit_circuit_init(
        &mut self,
        input: &'a CircuitInitExpression,
        additional: &Self::AdditionalInput,
    ) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_circuit_init(input) {
            input.members.iter().for_each(|member| {
                if let Some(expr) = &member.expression {
                    self.visit_expression(expr, additional);
                }
            });
        }
        None
    }

    fn visit_err(&mut self, input: &'a ErrExpression, _additional: &Self::AdditionalInput) -> Option<Self::Output> {
        self.visitor_ref().visit_err(input);
        None
    }
}

pub trait StatementVisitorDirector<'a>: VisitorDirector<'a> + ExpressionVisitorDirector<'a> {
    fn visit_statement(&mut self, input: &'a Statement) {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_statement(input) {
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
    }

    fn visit_return(&mut self, input: &'a ReturnStatement) {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_return(input) {
            self.visit_expression(&input.expression, &Default::default());
        }
    }

    fn visit_definition(&mut self, input: &'a DefinitionStatement) {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_definition(input) {
            self.visit_expression(&input.value, &Default::default());
        }
    }

    fn visit_assign(&mut self, input: &'a AssignStatement) {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_assign(input) {
            self.visit_expression(&input.value, &Default::default());
        }
    }

    fn visit_conditional(&mut self, input: &'a ConditionalStatement) {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_conditional(input) {
            self.visit_expression(&input.condition, &Default::default());
            self.visit_block(&input.block);
            if let Some(stmt) = input.next.as_ref() {
                self.visit_statement(stmt);
            }
        }
    }

    fn visit_iteration(&mut self, input: &'a IterationStatement) {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_iteration(input) {
            self.visit_expression(&input.start, &Default::default());
            self.visit_expression(&input.stop, &Default::default());
            self.visit_block(&input.block);
        }
    }

    fn visit_console(&mut self, input: &'a ConsoleStatement) {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_console(input) {
            match &input.function {
                ConsoleFunction::Assert(expr) => self.visit_expression(expr, &Default::default()),
                ConsoleFunction::Error(fmt) | ConsoleFunction::Log(fmt) => {
                    fmt.parameters.iter().for_each(|expr| {
                        self.visit_expression(expr, &Default::default());
                    });
                    None
                }
            };
        }
    }

    fn visit_block(&mut self, input: &'a Block) {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_block(input) {
            input.statements.iter().for_each(|stmt| self.visit_statement(stmt));
        }
    }
}

pub trait ProgramVisitorDirector<'a>: VisitorDirector<'a> + StatementVisitorDirector<'a> {
    fn visit_program(&mut self, input: &'a Program) {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_program(input) {
            input
                .functions
                .values()
                .for_each(|function| self.visit_function(function));
            input.circuits.values().for_each(|circuit| self.visit_circuit(circuit));
        }
    }

    fn visit_function(&mut self, input: &'a Function) {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_function(input) {
            self.visit_block(&input.block);
        }
    }

    fn visit_circuit(&mut self, input: &'a Circuit) {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_circuit(input) {
            input.members.iter().for_each(|member| {
                match member {
                    CircuitMember::CircuitVariable(_, _) => {}
                };
            })
        }
    }
}
