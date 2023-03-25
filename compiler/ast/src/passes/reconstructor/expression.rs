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

/// A Reconstructor trait for expressions in the AST.
pub trait ExpressionReconstructor {
    type AdditionalOutput: Default;

    fn reconstruct_expression(&mut self, input: Expression) -> (Expression, Self::AdditionalOutput) {
        match input {
            Expression::Access(access) => self.reconstruct_access(access),
            Expression::Binary(binary) => self.reconstruct_binary(binary),
            Expression::Call(call) => self.reconstruct_call(call),
            Expression::Struct(struct_) => self.reconstruct_struct_init(struct_),
            Expression::Err(err) => self.reconstruct_err(err),
            Expression::Identifier(identifier) => self.reconstruct_identifier(identifier),
            Expression::Literal(value) => self.reconstruct_literal(value),
            Expression::Ternary(ternary) => self.reconstruct_ternary(ternary),
            Expression::Tuple(tuple) => self.reconstruct_tuple(tuple),
            Expression::Unary(unary) => self.reconstruct_unary(unary),
            Expression::Unit(unit) => self.reconstruct_unit(unit),
        }
    }

    fn reconstruct_access(&mut self, input: AccessExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Access(match input {
                AccessExpression::AssociatedFunction(function) => {
                    AccessExpression::AssociatedFunction(AssociatedFunction {
                        ty: function.ty,
                        name: function.name,
                        args: function
                            .args
                            .into_iter()
                            .map(|arg| self.reconstruct_expression(arg).0)
                            .collect(),
                        span: function.span,
                    })
                }
                AccessExpression::Member(member) => AccessExpression::Member(MemberAccess {
                    inner: Box::new(self.reconstruct_expression(*member.inner).0),
                    name: member.name,
                    span: member.span,
                }),
                AccessExpression::Tuple(tuple) => AccessExpression::Tuple(TupleAccess {
                    tuple: Box::new(self.reconstruct_expression(*tuple.tuple).0),
                    index: tuple.index,
                    span: tuple.span,
                }),
                expr => expr,
            }),
            Default::default(),
        )
    }

    fn reconstruct_binary(&mut self, input: BinaryExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Binary(BinaryExpression {
                left: Box::new(self.reconstruct_expression(*input.left).0),
                right: Box::new(self.reconstruct_expression(*input.right).0),
                op: input.op,
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_call(&mut self, input: CallExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Call(CallExpression {
                function: Box::new(self.reconstruct_expression(*input.function).0),
                arguments: input
                    .arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg).0)
                    .collect(),
                external: input.external,
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_struct_init(&mut self, input: StructExpression) -> (Expression, Self::AdditionalOutput) {
        (Expression::Struct(input), Default::default())
    }

    fn reconstruct_err(&mut self, _input: ErrExpression) -> (Expression, Self::AdditionalOutput) {
        unreachable!("`ErrExpression`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Self::AdditionalOutput) {
        (Expression::Identifier(input), Default::default())
    }

    fn reconstruct_literal(&mut self, input: Literal) -> (Expression, Self::AdditionalOutput) {
        (Expression::Literal(input), Default::default())
    }

    fn reconstruct_ternary(&mut self, input: TernaryExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Ternary(TernaryExpression {
                condition: Box::new(self.reconstruct_expression(*input.condition).0),
                if_true: Box::new(self.reconstruct_expression(*input.if_true).0),
                if_false: Box::new(self.reconstruct_expression(*input.if_false).0),
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_tuple(&mut self, input: TupleExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Tuple(TupleExpression {
                elements: input
                    .elements
                    .into_iter()
                    .map(|element| self.reconstruct_expression(element).0)
                    .collect(),
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_unary(&mut self, input: UnaryExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Unary(UnaryExpression {
                receiver: Box::new(self.reconstruct_expression(*input.receiver).0),
                op: input.op,
                span: input.span,
            }),
            Default::default(),
        )
    }

    fn reconstruct_unit(&mut self, input: UnitExpression) -> (Expression, Self::AdditionalOutput) {
        (Expression::Unit(input), Default::default())
    }
}
