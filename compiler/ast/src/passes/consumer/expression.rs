// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::*;

/// A Consumer trait for expressions in the AST.
pub trait ExpressionConsumer {
    type Output;

    fn consume_expression(&mut self, input: Expression) -> Self::Output {
        match input {
            Expression::Access(access) => self.consume_access(access),
            Expression::Binary(binary) => self.consume_binary(binary),
            Expression::Call(call) => self.consume_call(call),
            Expression::Struct(struct_) => self.consume_struct_init(struct_),
            Expression::Err(err) => self.consume_err(err),
            Expression::Identifier(identifier) => self.consume_identifier(identifier),
            Expression::Literal(value) => self.consume_literal(value),
            Expression::Ternary(ternary) => self.consume_ternary(ternary),
            Expression::Tuple(tuple) => self.consume_tuple(tuple),
            Expression::Unary(unary) => self.consume_unary(unary),
            Expression::Unit(unit) => self.consume_unit(unit),
        }
    }

    fn consume_access(&mut self, _input: AccessExpression) -> Self::Output;

    fn consume_binary(&mut self, _input: BinaryExpression) -> Self::Output;

    fn consume_call(&mut self, _input: CallExpression) -> Self::Output;

    fn consume_struct_init(&mut self, _input: StructExpression) -> Self::Output;

    fn consume_err(&mut self, _input: ErrExpression) -> Self::Output {
        unreachable!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn consume_identifier(&mut self, _input: Identifier) -> Self::Output;

    fn consume_literal(&mut self, _input: Literal) -> Self::Output;

    fn consume_ternary(&mut self, _input: TernaryExpression) -> Self::Output;

    fn consume_tuple(&mut self, _input: TupleExpression) -> Self::Output;

    fn consume_unary(&mut self, _input: UnaryExpression) -> Self::Output;

    fn consume_unit(&mut self, _input: UnitExpression) -> Self::Output;
}
