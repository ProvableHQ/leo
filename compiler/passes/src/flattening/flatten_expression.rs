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

use crate::Value;

impl<'a> ExpressionReconstructor for Flattener<'a> {
    type AdditionalOutput = Option<Value>;
    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Self::AdditionalOutput) {
        let st = self.symbol_table.borrow();
        let var = st.lookup_variable(&input.name).unwrap();

        let val = if let Declaration::Const(Some(c)) | Declaration::Mut(Some(c)) = &var.declaration {
            if self.negate {
                match c.clone().neg(input.span) {
                    Ok(c) => Some(c),
                    Err(err) => {
                        self.handler.emit_err(err);
                        Some(c.clone())
                    }
                }
            } else {
                Some(c.clone())
            }
        } else {
            None
        };

        (Expression::Identifier(input), val)
    }

    fn reconstruct_unary(&mut self, input: UnaryExpression) -> (Expression, Self::AdditionalOutput) {
        let (receiver, val) = if matches!(input.op, UnaryOperation::Negate) {
            let prior_negate_state = self.negate;
            self.negate = !self.negate;
            let ret = self.reconstruct_expression(*input.receiver.clone());
            self.negate = prior_negate_state;
            ret
        } else {
            self.reconstruct_expression(*input.receiver.clone())
        };

        let out = match (val, input.op) {
            (Some(v), UnaryOperation::Abs) if v.is_supported_const_fold_type() => Some(v.abs(input.span)).transpose(),
            (Some(v), UnaryOperation::AbsWrapped) if v.is_supported_const_fold_type() => {
                Some(v.abs_wrapped(input.span)).transpose()
            }
            (Some(v), UnaryOperation::Negate) if v.is_supported_const_fold_type() => Ok(Some(v)),
            (Some(v), UnaryOperation::Not) if v.is_supported_const_fold_type() => Some(v.not(input.span)).transpose(),
            _ => Ok(None),
        };

        match out {
            Ok(v) => (
                Expression::Unary(UnaryExpression {
                    receiver: Box::new(receiver),
                    op: input.op,
                    span: input.span,
                }),
                v,
            ),
            Err(e) => {
                self.handler.emit_err(e);
                (Expression::Unary(input), None)
            }
        }
    }

    fn reconstruct_literal(&mut self, input: LiteralExpression) -> (Expression, Self::AdditionalOutput) {
        let value = match input.clone() {
            LiteralExpression::Address(val, span) => Value::Address(val, span),
            LiteralExpression::Boolean(val, span) => Value::Boolean(val, span),
            LiteralExpression::Field(val, span) => Value::Field(val, span),
            LiteralExpression::Group(val) => Value::Group(val),
            LiteralExpression::Integer(itype, istr, span) => {
                let istr = if self.negate { format!("-{istr}") } else { istr };
                match itype {
                    IntegerType::U8 => Value::U8(istr.parse().unwrap(), span),
                    IntegerType::U16 => Value::U16(istr.parse().unwrap(), span),
                    IntegerType::U32 => Value::U32(istr.parse().unwrap(), span),
                    IntegerType::U64 => Value::U64(istr.parse().unwrap(), span),
                    IntegerType::U128 => Value::U128(istr.parse().unwrap(), span),
                    IntegerType::I8 => Value::I8(istr.parse().unwrap(), span),
                    IntegerType::I16 => Value::I16(istr.parse().unwrap(), span),
                    IntegerType::I32 => Value::I32(istr.parse().unwrap(), span),
                    IntegerType::I64 => Value::I64(istr.parse().unwrap(), span),
                    IntegerType::I128 => Value::I128(istr.parse().unwrap(), span),
                }
            }
            LiteralExpression::Scalar(val, span) => Value::Scalar(val, span),
            LiteralExpression::String(val, span) => Value::String(val, span),
        };

        (Expression::Literal(input), Some(value))
    }

    fn reconstruct_call(&mut self, _: CallExpression) -> (Expression, Self::AdditionalOutput) {
        unimplemented!("Flattening functions not yet implemented")
    }

    fn reconstruct_binary(&mut self, input: BinaryExpression) -> (Expression, Self::AdditionalOutput) {
        let (left_expr, left_const_value) = self.reconstruct_expression(*input.left.clone());
        let (right_expr, right_const_value) = self.reconstruct_expression(*input.right.clone());

        match (left_const_value, right_const_value) {
            (Some(left_value), Some(right_value))
                if !left_value.is_supported_const_fold_type() && !right_value.is_supported_const_fold_type() =>
            {
                (Expression::Binary(input), None)
            }
            (Some(const_value), None) => (
                Expression::Binary(BinaryExpression {
                    left: Box::new(Expression::Literal(const_value.into())),
                    right: Box::new(right_expr),
                    op: input.op,
                    span: input.span,
                }),
                None,
            ),
            (None, Some(const_value)) => (
                Expression::Binary(BinaryExpression {
                    left: Box::new(left_expr),
                    right: Box::new(Expression::Literal(const_value.into())),
                    op: input.op,
                    span: input.span,
                }),
                None,
            ),
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
                    BinaryOperation::Nand => {
                        let bitand = left_value.bitand(right_value, input.span);
                        if let Err(err) = bitand {
                            self.handler.emit_err(err);
                            return (Expression::Binary(input), None);
                        }
                        bitand.unwrap().not(input.span)
                    }
                    BinaryOperation::Neq => {
                        let eq = left_value.eq(right_value, input.span);
                        if let Err(err) = eq {
                            self.handler.emit_err(err);
                            return (Expression::Binary(input), None);
                        }
                        eq.unwrap().not(input.span)
                    }
                    BinaryOperation::Nor => {
                        let nor = left_value.bitand(right_value, input.span);
                        if let Err(err) = nor {
                            self.handler.emit_err(err);
                            return (Expression::Binary(input), None);
                        }
                        nor.unwrap().not(input.span)
                    }
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
                    self.handler.emit_err(err);
                    (
                        Expression::Binary(BinaryExpression {
                            left: Box::new(left_expr),
                            right: Box::new(right_expr),
                            op: input.op,
                            span: input.span,
                        }),
                        None,
                    )
                } else {
                    let value = value.unwrap();
                    (Expression::Literal(value.clone().into()), Some(value))
                }
            }
            _ => (
                Expression::Binary(BinaryExpression {
                    left: Box::new(left_expr),
                    right: Box::new(right_expr),
                    op: input.op,
                    span: input.span,
                }),
                None,
            ),
        }
    }

    fn reconstruct_ternary(&mut self, input: TernaryExpression) -> (Expression, Self::AdditionalOutput) {
        let (condition, const_cond) = self.reconstruct_expression(*input.condition);
        let (if_true, const_if_true) = self.reconstruct_expression(*input.if_true);
        let (if_false, const_if_false) = self.reconstruct_expression(*input.if_false);
        match const_cond {
            Some(Value::Boolean(true, _)) => (if_true, const_if_true),
            Some(Value::Boolean(false, _)) => (if_false, const_if_false),
            _ => (
                Expression::Ternary(TernaryExpression {
                    condition: Box::new(condition),
                    if_true: Box::new(if_true),
                    if_false: Box::new(if_false),
                    span: input.span,
                }),
                None,
            ),
        }
    }
}
