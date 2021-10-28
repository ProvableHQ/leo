// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use std::cell::Cell;

use crate::{accesses::*, expression::*, program::*, statement::*};

pub enum VisitResult {
    VisitChildren,
    SkipChildren,
    Exit,
}

impl Default for VisitResult {
    fn default() -> Self {
        VisitResult::VisitChildren
    }
}

#[allow(unused_variables)]
pub trait ExpressionVisitor<'a> {
    fn visit_expression(&mut self, input: &Cell<&'a Expression<'a>>) -> VisitResult {
        Default::default()
    }

    fn visit_array_init(&mut self, input: &ArrayInitExpression<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_array_inline(&mut self, input: &ArrayInlineExpression<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_binary(&mut self, input: &BinaryExpression<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_call(&mut self, input: &CallExpression<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_circuit_init(&mut self, input: &CircuitInitExpression<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_ternary_expression(&mut self, input: &TernaryExpression<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_cast_expression(&mut self, input: &CastExpression<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_array_access(&mut self, input: &ArrayAccess<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_array_range_access(&mut self, input: &ArrayRangeAccess<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_circuit_access(&mut self, input: &CircuitAccess<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_tuple_access(&mut self, input: &TupleAccess<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_constant(&mut self, input: &Constant<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_access_expression(&mut self, input: &AccessExpression<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_tuple_init(&mut self, input: &TupleInitExpression<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_unary(&mut self, input: &UnaryExpression<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_variable_ref(&mut self, input: &VariableRef<'a>) -> VisitResult {
        Default::default()
    }
}

#[allow(unused_variables)]
pub trait StatementVisitor<'a>: ExpressionVisitor<'a> {
    fn visit_statement(&mut self, input: &Cell<&'a Statement<'a>>) -> VisitResult {
        Default::default()
    }

    // left = Some(ArrayIndex.0) always if AssignAccess::ArrayIndex. if member/tuple, always None
    fn visit_assign_access(&mut self, input: &AssignAccess<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_assign(&mut self, input: &AssignStatement<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_block(&mut self, input: &BlockStatement<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_conditional_statement(&mut self, input: &ConditionalStatement<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_formatted_string(&mut self, input: &ConsoleArgs<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_console(&mut self, input: &ConsoleStatement<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_definition(&mut self, input: &DefinitionStatement<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_expression_statement(&mut self, input: &ExpressionStatement<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_iteration(&mut self, input: &IterationStatement<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_return(&mut self, input: &ReturnStatement<'a>) -> VisitResult {
        Default::default()
    }
}

#[allow(unused_variables)]
pub trait ProgramVisitor<'a>: StatementVisitor<'a> {
    fn visit_function(&mut self, input: &'a Function<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_circuit_member(&mut self, input: &CircuitMember<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_circuit(&mut self, input: &'a Circuit<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_global_const(&mut self, input: &'a DefinitionStatement<'a>) -> VisitResult {
        Default::default()
    }

    fn visit_program(&mut self, input: &Program<'a>) -> VisitResult {
        Default::default()
    }
}
