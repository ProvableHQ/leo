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

use crate::{ConstValue, Expression, ExpressionNode, FromAst, IntegerType, Node, PartialType, Scope, Type};
pub use leo_ast::UnaryOperation;
use leo_errors::{Result, Span};

use std::cell::Cell;

#[derive(Clone)]
pub struct LengthOfExpression<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub inner: Cell<&'a Expression<'a>>,
}

impl<'a> Node for LengthOfExpression<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for LengthOfExpression<'a> {
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
        Some(Type::Integer(IntegerType::U32)) // For now we stick to U32 value type
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue> {
        None
    }

    fn is_consty(&self) -> bool {
        true
    }
}

impl<'a> FromAst<'a, leo_ast::LengthOfExpression> for LengthOfExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::LengthOfExpression,
        _expected_type: Option<PartialType<'a>>,
    ) -> Result<LengthOfExpression<'a>> {
        let inner = <&Expression<'a>>::from_ast(scope, &*value.inner, None)?;

        Ok(LengthOfExpression {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            inner: Cell::new(inner),
        })
    }
}

impl<'a> Into<leo_ast::LengthOfExpression> for &LengthOfExpression<'a> {
    fn into(self) -> leo_ast::LengthOfExpression {
        leo_ast::LengthOfExpression {
            inner: Box::new(self.inner.get().into()),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
