// Copyright (C) 2019-2025 Provable Inc.
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

use leo_ast::{AstVisitor, Block, Expression, Statement};

use super::visitor::LateLintingVisitor;

impl AstVisitor for LateLintingVisitor<'_> {
    type AdditionalInput = ();
    type Output = ();

    fn visit_expression(&mut self, input: &Expression, additional: &Self::AdditionalInput) -> Self::Output {
        for lint in &mut self.lints {
            lint.check_expression(input);
        }

        match input {
            Expression::Array(array) => self.visit_array(array, additional),
            Expression::ArrayAccess(access) => self.visit_array_access(access, additional),
            Expression::AssociatedConstant(constant) => self.visit_associated_constant(constant, additional),
            Expression::AssociatedFunction(function) => self.visit_associated_function(function, additional),
            Expression::Async(async_) => self.visit_async(async_, additional),
            Expression::Binary(binary) => self.visit_binary(binary, additional),
            Expression::Call(call) => self.visit_call(call, additional),
            Expression::Cast(cast) => self.visit_cast(cast, additional),
            Expression::Struct(struct_) => self.visit_struct_init(struct_, additional),
            Expression::Err(err) => self.visit_err(err, additional),
            Expression::Path(path) => self.visit_path(path, additional),
            Expression::Literal(literal) => self.visit_literal(literal, additional),
            Expression::Locator(locator) => self.visit_locator(locator, additional),
            Expression::MemberAccess(access) => self.visit_member_access(access, additional),
            Expression::Repeat(repeat) => self.visit_repeat(repeat, additional),
            Expression::Ternary(ternary) => self.visit_ternary(ternary, additional),
            Expression::Tuple(tuple) => self.visit_tuple(tuple, additional),
            Expression::TupleAccess(access) => self.visit_tuple_access(access, additional),
            Expression::Unary(unary) => self.visit_unary(unary, additional),
            Expression::Unit(unit) => self.visit_unit(unit, additional),
        }
    }

    fn visit_statement(&mut self, input: &Statement) {
        for lint in &mut self.lints {
            lint.check_statement(input);
        }

        match input {
            Statement::Assert(stmt) => self.visit_assert(stmt),
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Const(stmt) => self.visit_const(stmt),
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Expression(stmt) => self.visit_expression_statement(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Return(stmt) => self.visit_return(stmt),
        }
    }

    fn visit_block(&mut self, input: &Block) {
        for lint in &mut self.lints {
            lint.check_block(input);
        }

        input.statements.iter().for_each(|stmt| self.visit_statement(stmt));

        for lint in &mut self.lints {
            lint.check_block_post(input);
        }
    }
}
