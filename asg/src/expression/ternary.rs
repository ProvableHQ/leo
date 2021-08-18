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

use std::cell::Cell;

#[derive(Clone)]
pub struct TernaryExpression<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub condition: Cell<&'a Expression<'a>>,
    pub if_true: Cell<&'a Expression<'a>>,
    pub if_false: Cell<&'a Expression<'a>>,
}

impl<'a> Node for TernaryExpression<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for TernaryExpression<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        self.condition.get().set_parent(expr);
        self.if_true.get().set_parent(expr);
        self.if_false.get().set_parent(expr);
    }

    fn get_type(&self) -> Option<Type<'a>> {
        self.if_true.get().get_type()
    }

    fn is_mut_ref(&self) -> bool {
        self.if_true.get().is_mut_ref() && self.if_false.get().is_mut_ref()
    }

    fn const_value(&self) -> Option<ConstValue<'a>> {
        if let Some(ConstValue::Boolean(switch)) = self.condition.get().const_value() {
            if switch {
                self.if_true.get().const_value()
            } else {
                self.if_false.get().const_value()
            }
        } else {
            None
        }
    }

    fn is_consty(&self) -> bool {
        self.condition.get().is_consty() && self.if_true.get().is_consty() && self.if_false.get().is_consty()
    }
}

impl<'a> FromAst<'a, leo_ast::TernaryExpression> for TernaryExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::TernaryExpression,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<TernaryExpression<'a>, AsgConvertError> {
        let if_true = Cell::new(<&Expression<'a>>::from_ast(
            scope,
            &*value.if_true,
            expected_type.clone(),
        )?);
        let left: PartialType = if_true.get().get_type().unwrap().into();

        let if_false = if expected_type.is_none() {
            Cell::new(<&Expression<'a>>::from_ast(
                scope,
                &*value.if_false,
                Some(left.clone()),
            )?)
        } else {
            Cell::new(<&Expression<'a>>::from_ast(scope, &*value.if_false, expected_type)?)
        };
        let right = if_false.get().get_type().unwrap().into();

        if left != right {
            return Err(AsgConvertError::ternary_different_types(
                &left.to_string(),
                &right.to_string(),
                &value.span,
            ));
        }

        Ok(TernaryExpression {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            condition: Cell::new(<&Expression<'a>>::from_ast(
                scope,
                &*value.condition,
                Some(Type::Boolean.partial()),
            )?),
            if_true,
            if_false,
        })
    }
}

impl<'a> Into<leo_ast::TernaryExpression> for &TernaryExpression<'a> {
    fn into(self) -> leo_ast::TernaryExpression {
        leo_ast::TernaryExpression {
            condition: Box::new(self.condition.get().into()),
            if_true: Box::new(self.if_true.get().into()),
            if_false: Box::new(self.if_false.get().into()),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
