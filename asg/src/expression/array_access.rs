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

use std::cell::Cell;

#[derive(Clone)]
pub struct ArrayAccessExpression<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub array: Cell<&'a Expression<'a>>,
    pub index: Cell<&'a Expression<'a>>,
}

impl<'a> Node for ArrayAccessExpression<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for ArrayAccessExpression<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        self.array.get().set_parent(expr);
        self.index.get().set_parent(expr);
    }

    fn get_type(&self) -> Option<Type<'a>> {
        match self.array.get().get_type() {
            Some(Type::Array(element, _)) => Some(*element),
            _ => None,
        }
    }

    fn is_mut_ref(&self) -> bool {
        self.array.get().is_mut_ref()
    }

    fn const_value(&self) -> Option<ConstValue> {
        let mut array = match self.array.get().const_value()? {
            ConstValue::Array(values) => values,
            _ => return None,
        };
        let const_index = match self.index.get().const_value()? {
            ConstValue::Int(x) => x.to_usize()?,
            _ => return None,
        };
        if const_index >= array.len() {
            return None;
        }
        Some(array.remove(const_index))
    }

    fn is_consty(&self) -> bool {
        self.array.get().is_consty()
    }
}

impl<'a> FromAst<'a, leo_ast::ArrayAccessExpression> for ArrayAccessExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::ArrayAccessExpression,
        expected_type: Option<PartialType<'a>>,
        circuit_name: Option<&leo_ast::Identifier>,
    ) -> Result<ArrayAccessExpression<'a>, AsgConvertError> {
        let array = <&Expression<'a>>::from_ast(
            scope,
            &*value.array,
            Some(PartialType::Array(expected_type.map(Box::new), None)),
            circuit_name,
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

        let index = <&Expression<'a>>::from_ast(
            scope,
            &*value.index,
            Some(PartialType::Integer(None, Some(IntegerType::U32))),
            circuit_name,
        )?;

        if !index.is_consty() {
            return Err(AsgConvertError::unexpected_nonconst(
                &index.span().cloned().unwrap_or_default(),
            ));
        }

        Ok(ArrayAccessExpression {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            array: Cell::new(array),
            index: Cell::new(index),
        })
    }
}

impl<'a> Into<leo_ast::ArrayAccessExpression> for &ArrayAccessExpression<'a> {
    fn into(self) -> leo_ast::ArrayAccessExpression {
        leo_ast::ArrayAccessExpression {
            array: Box::new(self.array.get().into()),
            index: Box::new(self.index.get().into()),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
