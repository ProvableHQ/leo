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

use crate::{Declaration, Flattener};

impl<'a> ExpressionReconstructor for Flattener<'a> {
    fn reconstruct_expression(&mut self, input: Expression) -> Expression {
        match input {
            Expression::Identifier(identifier) => self.reconstruct_identifier(identifier),
            Expression::Value(value) => self.reconstruct_value(value),
            Expression::Binary(binary) => self.reconstruct_binary(binary),
            Expression::Unary(unary) => self.reconstruct_unary(unary),
            Expression::Ternary(ternary) => self.reconstruct_ternary(ternary),
            Expression::Call(call) => self.reconstruct_call(call),
            Expression::Err(err) => self.reconstruct_err(err),
        }
    }

    fn reconstruct_identifier(&mut self, input: Identifier) -> Expression {
        match &self
            .symbol_table
            .borrow()
            .lookup_variable(input.name)
            .unwrap()
            .declaration
        {
            Declaration::Const(Some(c)) => Expression::Value(c.clone().into()),
            _ => Expression::Identifier(input),
        }
    }

    fn reconstruct_value(&mut self, input: ValueExpression) -> Expression {
        Expression::Value(input)
    }

    fn reconstruct_binary(&mut self, input: BinaryExpression) -> Expression {
        Expression::Binary(BinaryExpression {
            left: Box::new(self.reconstruct_expression(*input.left)),
            right: Box::new(self.reconstruct_expression(*input.right)),
            op: input.op,
            span: input.span,
        })
    }

    fn reconstruct_unary(&mut self, input: UnaryExpression) -> Expression {
        Expression::Unary(UnaryExpression {
            inner: Box::new(self.reconstruct_expression(*input.inner)),
            op: input.op,
            span: input.span,
        })
    }

    fn reconstruct_ternary(&mut self, input: TernaryExpression) -> Expression {
        Expression::Ternary(TernaryExpression {
            condition: Box::new(self.reconstruct_expression(*input.condition)),
            if_true: Box::new(self.reconstruct_expression(*input.if_true)),
            if_false: Box::new(self.reconstruct_expression(*input.if_false)),
            span: input.span,
        })
    }

    fn reconstruct_call(&mut self, input: CallExpression) -> Expression {
        Expression::Call(CallExpression {
            function: Box::new(self.reconstruct_expression(*input.function)),
            arguments: input
                .arguments
                .into_iter()
                .map(|arg| self.reconstruct_expression(arg))
                .collect(),
            span: input.span,
        })
    }

    fn reconstruct_err(&mut self, input: ErrExpression) -> Expression {
        Expression::Err(input)
    }
}
