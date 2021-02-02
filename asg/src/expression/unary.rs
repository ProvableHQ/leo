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

use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

pub struct UnaryExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub operation: UnaryOperation,
    pub inner: Arc<Expression>,
}

impl Node for UnaryExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for UnaryExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.inner.set_parent(Arc::downgrade(expr));
    }

    fn get_type(&self) -> Option<Type> {
        self.inner.get_type()
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue> {
        if let Some(inner) = self.inner.const_value() {
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
        self.inner.is_consty()
    }
}

impl FromAst<leo_ast::UnaryExpression> for UnaryExpression {
    fn from_ast(
        scope: &Scope,
        value: &leo_ast::UnaryExpression,
        expected_type: Option<PartialType>,
    ) -> Result<UnaryExpression, AsgConvertError> {
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
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            operation: value.op.clone(),
            inner: Arc::<Expression>::from_ast(scope, &*value.inner, expected_type.map(Into::into))?,
        })
    }
}

impl Into<leo_ast::UnaryExpression> for &UnaryExpression {
    fn into(self) -> leo_ast::UnaryExpression {
        leo_ast::UnaryExpression {
            op: self.operation.clone(),
            inner: Box::new(self.inner.as_ref().into()),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
