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
pub use leo_ast::UnaryOperation;
use leo_errors::{AsgError, Result, Span};

use serde::Serialize;
use std::cell::Cell;

#[derive(Clone, Serialize)]
pub struct CastExpression<'a> {
    pub id: u32,
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub inner: Cell<&'a Expression<'a>>,
    pub target_type: Type<'a>,
}

impl<'a> Node for CastExpression<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for CastExpression<'a> {
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
        Some(self.target_type.clone())
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue> {
        let value = self.inner.get().const_value()?;
        match value {
            ConstValue::Int(int) => match &self.target_type {
                Type::Integer(target) => Some(ConstValue::Int(int.cast_to(target))),
                _ => None,
            },
            _ => None,
        }
    }

    fn is_consty(&self) -> bool {
        self.inner.get().is_consty()
    }
}

impl<'a> FromAst<'a, leo_ast::CastExpression> for CastExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::CastExpression,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<CastExpression<'a>> {
        let target_type = scope.resolve_ast_type(&value.target_type, &value.span)?;
        if let Some(expected_type) = &expected_type {
            if !expected_type.matches(&target_type) {
                return Err(AsgError::unexpected_type(expected_type, target_type, &value.span).into());
            }
        }

        let inner = <&Expression<'a>>::from_ast(scope, &*value.inner, None)?;

        Ok(CastExpression {
            id: scope.context.get_id(),
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            inner: Cell::new(inner),
            target_type,
        })
    }
}

impl<'a> Into<leo_ast::CastExpression> for &CastExpression<'a> {
    fn into(self) -> leo_ast::CastExpression {
        leo_ast::CastExpression {
            target_type: (&self.target_type).into(),
            inner: Box::new(self.inner.get().into()),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
