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
use leo_ast::IntegerType;

use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

pub struct ArrayAccessExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub array: Arc<Expression>,
    pub index: Arc<Expression>,
}

impl Node for ArrayAccessExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for ArrayAccessExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.array.set_parent(Arc::downgrade(expr));
        self.index.set_parent(Arc::downgrade(expr));
    }

    fn get_type(&self) -> Option<Type> {
        match self.array.get_type() {
            Some(Type::Array(element, _)) => Some(*element),
            _ => None,
        }
    }

    fn is_mut_ref(&self) -> bool {
        self.array.is_mut_ref()
    }

    fn const_value(&self) -> Option<ConstValue> {
        let mut array = match self.array.const_value()? {
            ConstValue::Array(values) => values,
            _ => return None,
        };
        let const_index = match self.index.const_value()? {
            ConstValue::Int(x) => x.to_usize()?,
            _ => return None,
        };
        if const_index >= array.len() {
            return None;
        }
        Some(array.remove(const_index))
    }

    fn is_consty(&self) -> bool {
        self.array.is_consty()
    }
}

impl FromAst<leo_ast::ArrayAccessExpression> for ArrayAccessExpression {
    fn from_ast(
        scope: &Scope,
        value: &leo_ast::ArrayAccessExpression,
        expected_type: Option<PartialType>,
    ) -> Result<ArrayAccessExpression, AsgConvertError> {
        let array = Arc::<Expression>::from_ast(
            scope,
            &*value.array,
            Some(PartialType::Array(expected_type.map(Box::new), None)),
        )?;
        match array.get_type() {
            Some(Type::Array(..)) => (),
            type_ => {
                return Err(AsgConvertError::unexpected_type(
                    "array",
                    type_.map(|x| x.to_string()).as_deref(),
                    &value.span,
                ));
            }
        }

        let index = Arc::<Expression>::from_ast(
            scope,
            &*value.index,
            Some(PartialType::Integer(None, Some(IntegerType::U32))),
        )?;

        if !index.is_consty() {
            return Err(AsgConvertError::unexpected_nonconst(
                &index.span().cloned().unwrap_or_default(),
            ));
        }

        Ok(ArrayAccessExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            array,
            index,
        })
    }
}

impl Into<leo_ast::ArrayAccessExpression> for &ArrayAccessExpression {
    fn into(self) -> leo_ast::ArrayAccessExpression {
        leo_ast::ArrayAccessExpression {
            array: Box::new(self.array.as_ref().into()),
            index: Box::new(self.index.as_ref().into()),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
