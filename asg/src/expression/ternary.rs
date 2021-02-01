// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

pub struct TernaryExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub condition: Arc<Expression>,
    pub if_true: Arc<Expression>,
    pub if_false: Arc<Expression>,
}

impl Node for TernaryExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for TernaryExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.condition.set_parent(Arc::downgrade(expr));
        self.if_true.set_parent(Arc::downgrade(expr));
        self.if_false.set_parent(Arc::downgrade(expr));
    }

    fn get_type(&self) -> Option<Type> {
        self.if_true.get_type()
    }

    fn is_mut_ref(&self) -> bool {
        self.if_true.is_mut_ref() && self.if_false.is_mut_ref()
    }

    fn const_value(&self) -> Option<ConstValue> {
        if let Some(ConstValue::Boolean(switch)) = self.condition.const_value() {
            if switch {
                self.if_true.const_value()
            } else {
                self.if_false.const_value()
            }
        } else {
            None
        }
    }

    fn is_consty(&self) -> bool {
        self.condition.is_consty() && self.if_true.is_consty() && self.if_false.is_consty()
    }
}

impl FromAst<leo_ast::TernaryExpression> for TernaryExpression {
    fn from_ast(
        scope: &Scope,
        value: &leo_ast::TernaryExpression,
        expected_type: Option<PartialType>,
    ) -> Result<TernaryExpression, AsgConvertError> {
        Ok(TernaryExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            condition: Arc::<Expression>::from_ast(scope, &*value.condition, Some(Type::Boolean.partial()))?,
            if_true: Arc::<Expression>::from_ast(scope, &*value.if_true, expected_type.clone())?,
            if_false: Arc::<Expression>::from_ast(scope, &*value.if_false, expected_type)?,
        })
    }
}

impl Into<leo_ast::TernaryExpression> for &TernaryExpression {
    fn into(self) -> leo_ast::TernaryExpression {
        leo_ast::TernaryExpression {
            condition: Box::new(self.condition.as_ref().into()),
            if_true: Box::new(self.if_true.as_ref().into()),
            if_false: Box::new(self.if_false.as_ref().into()),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
