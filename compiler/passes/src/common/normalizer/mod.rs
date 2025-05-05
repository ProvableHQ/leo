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

use leo_ast::{Expression, ExpressionReconstructor, Node, ProgramReconstructor, Statement, StatementReconstructor};

/// A `Normalizer` normalizes all NodeID and Span values in the AST.
pub struct Normalizer;

impl ExpressionReconstructor for Normalizer {
    type AdditionalOutput = ();

    fn reconstruct_expression(&mut self, mut input: Expression) -> (Expression, Self::AdditionalOutput) {
        // Set the ID and span to 0.
        input.set_id(0);
        input.set_span(Default::default());
        // Reconstructo the sub-expressions.
        match input {
            Expression::AssociatedConstant(constant) => self.reconstruct_associated_constant(constant),
            Expression::AssociatedFunction(function) => self.reconstruct_associated_function(function),
            Expression::Array(array) => self.reconstruct_array(array),
            Expression::ArrayAccess(access) => self.reconstruct_array_access(*access),
            Expression::Binary(binary) => self.reconstruct_binary(*binary),
            Expression::Call(call) => self.reconstruct_call(*call),
            Expression::Cast(cast) => self.reconstruct_cast(*cast),
            Expression::Struct(struct_) => self.reconstruct_struct_init(struct_),
            Expression::Err(err) => self.reconstruct_err(err),
            Expression::Identifier(identifier) => self.reconstruct_identifier(identifier),
            Expression::Literal(value) => self.reconstruct_literal(value),
            Expression::Locator(locator) => self.reconstruct_locator(locator),
            Expression::MemberAccess(access) => self.reconstruct_member_access(*access),
            Expression::Ternary(ternary) => self.reconstruct_ternary(*ternary),
            Expression::Tuple(tuple) => self.reconstruct_tuple(tuple),
            Expression::TupleAccess(access) => self.reconstruct_tuple_access(*access),
            Expression::Unary(unary) => self.reconstruct_unary(*unary),
            Expression::Unit(unit) => self.reconstruct_unit(unit),
        }
    }
}

impl StatementReconstructor for Normalizer {
    fn reconstruct_statement(&mut self, mut input: leo_ast::Statement) -> (leo_ast::Statement, Self::AdditionalOutput) {
        // Set the ID and span to 0.
        input.set_id(0);
        input.set_span(Default::default());
        // Reconstruct the sub-components.
        match input {
            Statement::Assert(assert) => self.reconstruct_assert(assert),
            Statement::Assign(stmt) => self.reconstruct_assign(*stmt),
            Statement::Block(stmt) => {
                let (stmt, output) = self.reconstruct_block(stmt);
                (stmt.into(), output)
            }
            Statement::Conditional(stmt) => self.reconstruct_conditional(stmt),
            Statement::Const(stmt) => self.reconstruct_const(stmt),
            Statement::Definition(stmt) => self.reconstruct_definition(stmt),
            Statement::Expression(stmt) => self.reconstruct_expression_statement(stmt),
            Statement::Iteration(stmt) => self.reconstruct_iteration(*stmt),
            Statement::Return(stmt) => self.reconstruct_return(stmt),
        }
    }
}

impl ProgramReconstructor for Normalizer {}
