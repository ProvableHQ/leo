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

use crate::{AsgConvertError, ConstValue, Expression, ExpressionNode, FromAst, Node, PartialType, Scope, Span, Type};
pub use leo_ast::UnaryOperation;

use std::cell::Cell;

#[derive(Clone)]
pub struct UnaryExpression<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub operation: UnaryOperation,
    pub inner: Cell<&'a Expression<'a>>,
}

impl<'a> Node for UnaryExpression<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for UnaryExpression<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        self.inner.get().set_parent(expr);
    }

    fn get_type(&self) -> Option<Type<'a>> {
        self.inner.get().get_type()
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue> {
        if let Some(inner) = self.inner.get().const_value() {
            match self.operation {
                UnaryOperation::Not => match inner {
                    ConstValue::Boolean(value) => Some(ConstValue::Boolean(!value)),
                    _ => None,
                },
                UnaryOperation::Negate => {
                    match inner {
                        ConstValue::Int(value) => Some(ConstValue::Int(value.value_negate()?)),
                        // ConstValue::Group(value) => Some(ConstValue::Group(value)), TODO: groups
                        // ConstValue::Field(value) => Some(ConstValue::Field(-value)),
                        _ => None,
                    }
                }
            }
        } else {
            None
        }
    }

    fn is_consty(&self) -> bool {
        self.inner.get().is_consty()
    }
}

impl<'a> FromAst<'a, leo_ast::UnaryExpression> for UnaryExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::UnaryExpression,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<UnaryExpression<'a>, AsgConvertError> {
        let expected_type = match value.op {
            UnaryOperation::Not => match expected_type.map(|x| x.full()).flatten() {
                Some(Type::Boolean) | None => Some(Type::Boolean),
                Some(type_) => {
                    return Err(AsgConvertError::unexpected_type(
                        &type_.to_string(),
                        Some(&*Type::Boolean.to_string()),
                        &value.span,
                    ));
                }
            },
            UnaryOperation::Negate => match expected_type.map(|x| x.full()).flatten() {
                Some(type_ @ Type::Integer(_)) => Some(type_),
                Some(Type::Group) => Some(Type::Group),
                Some(Type::Field) => Some(Type::Field),
                None => None,
                Some(type_) => {
                    return Err(AsgConvertError::unexpected_type(
                        &type_.to_string(),
                        Some("integer, group, field"),
                        &value.span,
                    ));
                }
            },
        };
        Ok(UnaryExpression {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            operation: value.op.clone(),
            inner: Cell::new(<&Expression<'a>>::from_ast(
                scope,
                &*value.inner,
                expected_type.map(Into::into),
            )?),
        })
    }
}

impl<'a> Into<leo_ast::UnaryExpression> for &UnaryExpression<'a> {
    fn into(self) -> leo_ast::UnaryExpression {
        leo_ast::UnaryExpression {
            op: self.operation.clone(),
            inner: Box::new(self.inner.get().into()),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
