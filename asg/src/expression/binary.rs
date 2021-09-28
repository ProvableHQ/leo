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

use crate::{ConstValue, Expression, ExpressionNode, FromAst, Node, PartialType, Scope, Type};
pub use leo_ast::{BinaryOperation, BinaryOperationClass};
use leo_errors::{AsgError, Result, Span};

use serde::Serialize;
use std::cell::Cell;

#[derive(Clone, Serialize)]
pub struct BinaryExpression<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub operation: BinaryOperation,
    pub left: Cell<&'a Expression<'a>>,
    pub right: Cell<&'a Expression<'a>>,
}

impl<'a> Node for BinaryExpression<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for BinaryExpression<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        self.left.get().set_parent(expr);
        self.right.get().set_parent(expr);
    }

    fn get_type(&self) -> Option<Type<'a>> {
        match self.operation.class() {
            BinaryOperationClass::Boolean => Some(Type::Boolean),
            BinaryOperationClass::Numeric => self.left.get().get_type(),
        }
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue> {
        use BinaryOperation::*;
        let left = self.left.get().const_value()?;
        let right = self.right.get().const_value()?;

        match (left, right) {
            (ConstValue::Int(left), ConstValue::Int(right)) => Some(match self.operation {
                Add => ConstValue::Int(left.value_add(&right)?),
                Sub => ConstValue::Int(left.value_sub(&right)?),
                Mul => ConstValue::Int(left.value_mul(&right)?),
                Div => ConstValue::Int(left.value_div(&right)?),
                Pow => ConstValue::Int(left.value_pow(&right)?),
                Eq => ConstValue::Boolean(left == right),
                Ne => ConstValue::Boolean(left != right),
                Ge => ConstValue::Boolean(left.value_ge(&right)?),
                Gt => ConstValue::Boolean(left.value_gt(&right)?),
                Le => ConstValue::Boolean(left.value_le(&right)?),
                Lt => ConstValue::Boolean(left.value_lt(&right)?),
                _ => return None,
            }),
            // (ConstValue::Field(left), ConstValue::Field(right)) => {
            //     Some(match self.operation {
            //         Add => ConstValue::Field(left.checked_add(&right)?),
            //         Sub => ConstValue::Field(left.checked_sub(&right)?),
            //         Mul => ConstValue::Field(left.checked_mul(&right)?),
            //         Div => ConstValue::Field(left.checked_div(&right)?),
            //         Eq => ConstValue::Boolean(left == right),
            //         Ne => ConstValue::Boolean(left != right),
            //         _ => return None,
            //     })
            // },
            (ConstValue::Boolean(left), ConstValue::Boolean(right)) => Some(match self.operation {
                Eq => ConstValue::Boolean(left == right),
                Ne => ConstValue::Boolean(left != right),
                And => ConstValue::Boolean(left && right),
                Or => ConstValue::Boolean(left || right),
                _ => return None,
            }),
            //todo: group?
            (left, right) => Some(match self.operation {
                Eq => ConstValue::Boolean(left == right),
                Ne => ConstValue::Boolean(left != right),
                _ => return None,
            }),
        }
    }

    fn is_consty(&self) -> bool {
        self.left.get().is_consty() && self.right.get().is_consty()
    }
}

impl<'a> FromAst<'a, leo_ast::BinaryExpression> for BinaryExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::BinaryExpression,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<BinaryExpression<'a>> {
        let class = value.op.class();
        let expected_type = match class {
            BinaryOperationClass::Boolean => match expected_type {
                Some(PartialType::Type(Type::Boolean)) | None => None,
                Some(x) => {
                    return Err(AsgError::unexpected_type(Type::Boolean, x, &value.span).into());
                }
            },
            BinaryOperationClass::Numeric => match expected_type {
                Some(x @ PartialType::Integer(_, _)) => Some(x),
                Some(x @ PartialType::Type(Type::Field)) => Some(x),
                Some(x @ PartialType::Type(Type::Group)) => Some(x),
                Some(x) => {
                    return Err(AsgError::unexpected_type("integer, field, or group", x, &value.span).into());
                }
                None => None,
            },
        };

        // left
        let (left, right) = match <&Expression<'a>>::from_ast(scope, &*value.left, expected_type.clone()) {
            Ok(left) => {
                if let Some(left_type) = left.get_type() {
                    let right = <&Expression<'a>>::from_ast(scope, &*value.right, Some(left_type.partial()))?;
                    (left, right)
                } else {
                    let right = <&Expression<'a>>::from_ast(scope, &*value.right, expected_type)?;
                    if let Some(right_type) = right.get_type() {
                        (
                            <&Expression<'a>>::from_ast(scope, &*value.left, Some(right_type.partial()))?,
                            right,
                        )
                    } else {
                        (left, right)
                    }
                }
            }
            Err(e) => {
                let right = <&Expression<'a>>::from_ast(scope, &*value.right, expected_type)?;
                if let Some(right_type) = right.get_type() {
                    (
                        <&Expression<'a>>::from_ast(scope, &*value.left, Some(right_type.partial()))?,
                        right,
                    )
                } else {
                    return Err(e);
                }
            }
        };

        let left_type = left.get_type();
        #[allow(clippy::unused_unit)]
        match class {
            BinaryOperationClass::Numeric => match left_type {
                Some(Type::Integer(_)) => (),
                Some(Type::Group) | Some(Type::Field)
                    if value.op == BinaryOperation::Add || value.op == BinaryOperation::Sub =>
                {
                    ()
                }
                Some(Type::Field) if value.op == BinaryOperation::Mul || value.op == BinaryOperation::Div => (),
                type_ => {
                    return Err(AsgError::unexpected_type(
                        "integer",
                        type_.map(|x| x.to_string()).unwrap_or_else(|| "unknown".to_string()),
                        &value.span,
                    )
                    .into());
                }
            },
            BinaryOperationClass::Boolean => match &value.op {
                BinaryOperation::And | BinaryOperation::Or => match left_type {
                    Some(Type::Boolean) | None => (),
                    Some(x) => {
                        return Err(AsgError::unexpected_type(Type::Boolean, x, &value.span).into());
                    }
                },
                BinaryOperation::Eq | BinaryOperation::Ne => (), // all types allowed
                op => match left_type {
                    Some(Type::Integer(_)) | None => (),
                    Some(x) => {
                        return Err(
                            AsgError::operator_allowed_only_for_type(op.as_ref(), "integer", x, &value.span).into(),
                        );
                    }
                },
            },
        }

        let right_type = right.get_type();

        match (left_type, right_type) {
            (Some(left_type), Some(right_type)) => {
                if !left_type.is_assignable_from(&right_type) {
                    return Err(AsgError::unexpected_type(left_type, right_type, &value.span).into());
                }
            }
            (None, None) => {
                return Err(AsgError::unexpected_type("any type", "unknown type", &value.span).into());
            }
            (_, _) => (),
        }
        Ok(BinaryExpression {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            operation: value.op,
            left: Cell::new(left),
            right: Cell::new(right),
        })
    }
}

impl<'a> Into<leo_ast::BinaryExpression> for &BinaryExpression<'a> {
    fn into(self) -> leo_ast::BinaryExpression {
        leo_ast::BinaryExpression {
            op: self.operation,
            left: Box::new(self.left.get().into()),
            right: Box::new(self.right.get().into()),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
