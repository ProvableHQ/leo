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

use crate::{ConstantFolder, VariableType};

impl<'a> ExpressionReconstructor for ConstantFolder<'a> {
    // This is the possible constant value of an expression.
    type AdditionalOutput = Option<Value>;
    fn reconstruct_binary(&mut self, input: BinaryExpression) -> (Expression, Self::AdditionalOutput) {
        let (left_expr, left_const_value) = self.reconstruct_expression(*input.left.clone());
        let (right_expr, right_const_value) = self.reconstruct_expression(*input.right.clone());

        // We check if both sides are constant values
        // That are a currently supported type for constant folding
        // Which are bools and ints.
        match (left_const_value, right_const_value) {
            (Some(left_value), Some(right_value))
                // if its an unsupported type we just return
                // saying it has no constant value.
                if !left_value.is_supported_const_fold_type() && !right_value.is_supported_const_fold_type() =>
            {
                (Expression::Binary(input), None)
            }
            // The following cases are if only one side is constant.
            // That does not fold and does not return a constant value.
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
            // If both sides are constant we call the appropriate operations and fold into a literal.
            (Some(left_value), Some(right_value)) => {
                let value = match &input.op {
                    BinaryOperation::Add => left_value.add(right_value),
                    BinaryOperation::AddWrapped => left_value.add_wrapped(right_value),
                    BinaryOperation::And | BinaryOperation::BitwiseAnd => left_value.bitand(right_value),
                    BinaryOperation::Div => left_value.div(right_value),
                    BinaryOperation::DivWrapped => left_value.div_wrapped(right_value),
                    BinaryOperation::Eq => left_value.eq(right_value),
                    BinaryOperation::Gte => left_value.ge(right_value),
                    BinaryOperation::Gt => left_value.gt(right_value),
                    BinaryOperation::Lte => left_value.le(right_value),
                    BinaryOperation::Lt => left_value.lt(right_value),
                    BinaryOperation::Mul => left_value.mul(right_value),
                    BinaryOperation::MulWrapped => left_value.mul_wrapped(right_value),
                    BinaryOperation::Nand => {
                        let bitand = left_value.bitand(right_value);
                        if let Err(err) = bitand {
                            self.handler.emit_err(err);
                            return (Expression::Binary(input), None);
                        }
                        bitand.unwrap().not()
                    }
                    BinaryOperation::Neq => {
                        let eq = left_value.eq(right_value);
                        if let Err(err) = eq {
                            self.handler.emit_err(err);
                            return (Expression::Binary(input), None);
                        }
                        eq.unwrap().not()
                    }
                    BinaryOperation::Nor => {
                        let nor = left_value.bitand(right_value);
                        if let Err(err) = nor {
                            self.handler.emit_err(err);
                            return (Expression::Binary(input), None);
                        }
                        nor.unwrap().not()
                    }
                    BinaryOperation::Or | BinaryOperation::BitwiseOr => left_value.bitor(right_value),
                    BinaryOperation::Pow => left_value.pow(right_value),
                    BinaryOperation::PowWrapped => left_value.pow_wrapped(right_value),
                    BinaryOperation::Shl => left_value.shl(right_value),
                    BinaryOperation::ShlWrapped => left_value.shl_wrapped(right_value),
                    BinaryOperation::Shr => left_value.shr(right_value),
                    BinaryOperation::ShrWrapped => left_value.shr_wrapped(right_value),
                    BinaryOperation::Sub => left_value.sub(right_value),
                    BinaryOperation::SubWrapped => left_value.sub_wrapped(right_value),
                    BinaryOperation::Xor => left_value.xor(right_value),
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

    fn reconstruct_call(&mut self, _: CallExpression) -> (Expression, Self::AdditionalOutput) {
        // We only support the main function for now in flattening.
        unimplemented!("Flattening functions not yet implemented")
    }

    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Self::AdditionalOutput) {
        let st = self.symbol_table.borrow();
        let var = st.lookup_variable(input.name).unwrap();

        // We grab the constant value of a variable if it exists.
        let val = match &var.variable_type {
            VariableType::Const | VariableType::Mut => match var.value.clone() {
                Some(value) => {
                    if self.negate {
                        match value.clone().neg() {
                            Ok(c) => Some(c),
                            Err(err) => {
                                self.handler.emit_err(err);
                                Some(value)
                            }
                        }
                    } else {
                        Some(value)
                    }
                }
                None => None,
            },
            VariableType::Input(..) => None,
        };

        (Expression::Identifier(input), val)
    }

    fn reconstruct_literal(&mut self, input: Literal) -> (Expression, Self::AdditionalOutput) {
        // We parse the literal value as a constant.
        // TODO: we should have these parsed at parsing time.
        let value = match input.clone() {
            Literal::Address(val, _) => Value::Address(val),
            Literal::Boolean(val, _) => Value::Boolean(val),
            Literal::Field(val, _) => Value::Field(val),
            Literal::Group(val) => Value::Group(val),
            Literal::I8(istr, _) => {
                let istr = if self.negate { format!("-{}", istr) } else { istr };
                Value::I8(istr.parse().unwrap())
            }
            Literal::I16(istr, _) => {
                let istr = if self.negate { format!("-{}", istr) } else { istr };
                Value::I16(istr.parse().unwrap())
            }
            Literal::I32(istr, _) => {
                let istr = if self.negate { format!("-{}", istr) } else { istr };
                Value::I32(istr.parse().unwrap())
            }
            Literal::I64(istr, _) => {
                let istr = if self.negate { format!("-{}", istr) } else { istr };
                Value::I64(istr.parse().unwrap())
            }
            Literal::I128(istr, _) => {
                let istr = if self.negate { format!("-{}", istr) } else { istr };
                Value::I128(istr.parse().unwrap())
            }
            Literal::U8(ustr, _) => {
                let ustr = if self.negate { format!("-{}", ustr) } else { ustr };
                Value::U8(ustr.parse().unwrap())
            }
            Literal::U16(ustr, _) => {
                let ustr = if self.negate { format!("-{}", ustr) } else { ustr };
                Value::U16(ustr.parse().unwrap())
            }
            Literal::U32(ustr, _) => {
                let ustr = if self.negate { format!("-{}", ustr) } else { ustr };
                Value::U32(ustr.parse().unwrap())
            }
            Literal::U64(ustr, _) => {
                let ustr = if self.negate { format!("-{}", ustr) } else { ustr };
                Value::U64(ustr.parse().unwrap())
            }
            Literal::U128(ustr, _) => {
                let ustr = if self.negate { format!("-{}", ustr) } else { ustr };
                Value::U128(ustr.parse().unwrap())
            }
            Literal::Scalar(val, _) => Value::Scalar(val),
            Literal::String(val, _) => Value::String(val),
        };

        (Expression::Literal(input), Some(value))
    }

    fn reconstruct_ternary(&mut self, input: TernaryExpression) -> (Expression, Self::AdditionalOutput) {
        let (condition, const_cond) = self.reconstruct_expression(*input.condition);
        let (if_true, const_if_true) = self.reconstruct_expression(*input.if_true);
        let (if_false, const_if_false) = self.reconstruct_expression(*input.if_false);
        // If the ternary condition is constant, we just return the appropriate ternary branch.
        match const_cond {
            Some(Value::Boolean(true)) => (if_true, const_if_true),
            Some(Value::Boolean(false)) => (if_false, const_if_false),
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

    fn reconstruct_unary(&mut self, input: UnaryExpression) -> (Expression, Self::AdditionalOutput) {
        // If we are doing a negation operation we set appropriate flags.
        let (receiver, val) = if matches!(input.op, UnaryOperation::Negate) {
            let prior_negate_state = self.negate;
            self.negate = !self.negate;
            let ret = self.reconstruct_expression(*input.receiver.clone());
            self.negate = prior_negate_state;
            ret
        } else {
            self.reconstruct_expression(*input.receiver.clone())
        };

        // We handle the following constant folding operations.
        // The rest don't support non int/bool types.
        // The only types we constant fold are int and bool types.
        let out = match (val, input.op) {
            (Some(v), UnaryOperation::Abs) if v.is_supported_const_fold_type() => Some(v.abs()).transpose(),
            (Some(v), UnaryOperation::AbsWrapped) if v.is_supported_const_fold_type() => {
                Some(v.abs_wrapped()).transpose()
            }
            (Some(v), UnaryOperation::Negate) if v.is_supported_const_fold_type() => Ok(Some(v)),
            (Some(v), UnaryOperation::Not) if v.is_supported_const_fold_type() => Some(v.not()).transpose(),
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
}
