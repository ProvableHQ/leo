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

use crate::Value;

impl<'a> ExpressionReconstructor for Flattener<'a> {
    type AdditionalOutput = Option<Value>;
    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Self::AdditionalOutput) {
        // let x: u32 = 10u32;
        // let y: u32 = x;

        // return y == 10u32;

        /*
        function main(b: bool) {
            let x = 0;
            if b {
                x = 1;
            } else {
                x = 2;
            }
            x == 1
        }

        function main() {
            let x = 0;
            if true {
                x = 1;
            } else {
                x = 2;
            }
            x == 1
        }
        */
        match &self
            .symbol_table
            .borrow()
            .lookup_variable(input.name)
            .unwrap()
            .declaration
        {
            Declaration::Const(Some(c)) | Declaration::Mut(Some(c)) => {
                (Expression::Literal(c.clone().into()), Some(c.clone()))
            }
            _ => (Expression::Identifier(input), None),
        }
    }

    fn reconstruct_call(&mut self, input: CallExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Call(CallExpression {
                function: input.function,
                arguments: input
                    .arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg).0)
                    .collect(),
                span: input.span,
            }),
            None,
        )
    }

    fn reconstruct_binary(&mut self, input: BinaryExpression) -> (Expression, Self::AdditionalOutput) {
        let (_, left_const_value) = self.reconstruct_expression(*input.left.clone());
        let (_, right_const_value) = self.reconstruct_expression(*input.right.clone());

        match (left_const_value, right_const_value) {
            (Some(left_value), Some(right_value))
                if !left_value.is_supported_const_fold_type() && !right_value.is_supported_const_fold_type() =>
            {
                (Expression::Binary(input), None)
            }
            (Some(left_value), Some(right_value)) => {
                let value = match &input.op {
                    BinaryOperation::Add => left_value.add(right_value, input.span),
                    BinaryOperation::AddWrapped => left_value.add_wrapped(right_value, input.span),
                    BinaryOperation::And | BinaryOperation::BitwiseAnd => left_value.bitand(right_value, input.span),
                    BinaryOperation::Div => left_value.div(right_value, input.span),
                    BinaryOperation::DivWrapped => left_value.div_wrapped(right_value, input.span),
                    BinaryOperation::Eq => left_value.eq(right_value, input.span),
                    BinaryOperation::Ge => left_value.ge(right_value, input.span),
                    BinaryOperation::Gt => left_value.gt(right_value, input.span),
                    BinaryOperation::Le => left_value.le(right_value, input.span),
                    BinaryOperation::Lt => left_value.lt(right_value, input.span),
                    BinaryOperation::Mul => left_value.mul(right_value, input.span),
                    BinaryOperation::MulWrapped => left_value.mul_wrapped(right_value, input.span),
                    BinaryOperation::Nand => left_value.bitand(right_value, input.span).map(|v| !v),
                    BinaryOperation::Neq => left_value.eq(right_value, input.span).map(|v| !v),
                    BinaryOperation::Nor => left_value.bitor(right_value, input.span).map(|v| !v),
                    BinaryOperation::Or | BinaryOperation::BitwiseOr => left_value.bitor(right_value, input.span),
                    BinaryOperation::Pow => left_value.pow(right_value, input.span),
                    BinaryOperation::PowWrapped => left_value.pow_wrapped(right_value, input.span),
                    BinaryOperation::Shl => left_value.shl(right_value, input.span),
                    BinaryOperation::ShlWrapped => left_value.shl_wrapped(right_value, input.span),
                    BinaryOperation::Shr => left_value.shr(right_value, input.span),
                    BinaryOperation::ShrWrapped => left_value.shr_wrapped(right_value, input.span),
                    BinaryOperation::Sub => left_value.sub(right_value, input.span),
                    BinaryOperation::SubWrapped => left_value.sub_wrapped(right_value, input.span),
                    BinaryOperation::Xor => left_value.xor(right_value, input.span),
                };

                if let Err(err) = value {
                    self._handler.emit_err(err);
                    (Expression::Binary(input), None)
                } else {
                    let value = value.unwrap();
                    (Expression::Literal(value.clone().into()), Some(value))
                }
            }
            _ => (Expression::Binary(input), None),
        }
    }
}
