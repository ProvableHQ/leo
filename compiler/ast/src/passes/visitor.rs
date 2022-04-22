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

//! This module contains Visitor traits for the AST.

use crate::*;

pub enum VisitResult {
    VisitChildren,
    SkipChildren,
}

impl Default for VisitResult {
    fn default() -> Self {
        VisitResult::VisitChildren
    }
}

pub trait ExpressionVisitor {
    fn visit_expression(&mut self, _input: &Expression) -> VisitResult {
        Default::default()
    }

    fn visit_identifer(&mut self, _input: &Identifier) -> VisitResult {
        Default::default()
    }

    fn visit_value(&mut self, _input: &ValueExpression) -> VisitResult {
        Default::default()
    }

    fn visit_binary(&mut self, _input: &BinaryExpression) -> VisitResult {
        Default::default()
    }

    fn visit_unary(&mut self, _input: &UnaryExpression) -> VisitResult {
        Default::default()
    }

    fn visit_ternary(&mut self, _input: &TernaryExpression) -> VisitResult {
        Default::default()
    }

    fn visit_call(&mut self, _input: &CallExpression) -> VisitResult {
        Default::default()
    }

    fn visit_err(&mut self, _input: &ErrExpression) -> VisitResult {
        Default::default()
    }
}

pub trait StatementVisitor {
    fn visit_statement(&mut self, _input: &Statement) -> VisitResult {
        Default::default()
    }

    fn visit_return(&mut self, _input: &ReturnStatement) -> VisitResult {
        Default::default()
    }

    fn visit_definition(&mut self, _input: &DefinitionStatement) -> VisitResult {
        Default::default()
    }

    fn visit_assign(&mut self, _input: &AssignStatement) -> VisitResult {
        Default::default()
    }

    fn visit_conditional(&mut self, _input: &ConditionalStatement) -> VisitResult {
        Default::default()
    }

    fn visit_iteration(&mut self, _input: &IterationStatement) -> VisitResult {
        Default::default()
    }

    fn visit_console(&mut self, _input: &ConsoleStatement) -> VisitResult {
        Default::default()
    }

    fn visit_expression_statement(&mut self, _input: &ExpressionStatement) -> VisitResult {
        Default::default()
    }

    fn visit_block(&mut self, _input: &Block) -> VisitResult {
        Default::default()
    }
}

pub trait ProgramVisitor {
    fn visit_program(&mut self, _input: &Program) -> VisitResult {
        Default::default()
    }

    fn visit_function(&mut self, _input: &Function) -> VisitResult {
        Default::default()
    }
}
