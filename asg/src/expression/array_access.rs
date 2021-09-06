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
use leo_ast::IntegerType;
use leo_errors::{AsgError, Result, Span};

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

    fn const_value(&self) -> Option<ConstValue<'a>> {
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
    ) -> Result<ArrayAccessExpression<'a>> {
        let array = <&Expression<'a>>::from_ast(
            scope,
            &*value.array,
            Some(PartialType::Array(expected_type.map(Box::new), None)),
        )?;
        let array_len = match array.get_type() {
            Some(Type::Array(_, len)) => Some(len),
            Some(Type::ArrayWithoutSize(_)) => None,
            type_ => {
                return Err(AsgError::unexpected_type(
                    "array",
                    type_.map(|x| x.to_string()).unwrap_or_else(|| "unknown".to_string()),
                    &value.span,
                )
                .into());
            }
        };

        let index = <&Expression<'a>>::from_ast(
            scope,
            &*value.index,
            Some(PartialType::Integer(None, Some(IntegerType::U32))),
        )?;

        if let Some(index) = index
            .const_value()
            .map(|x| x.int().map(|x| x.to_usize()).flatten())
            .flatten()
        {
            // Only check index if array size is known.
            // Array out of bounds will be caught later if it really happens.
            if let Some(array_len) = array_len {
                if index >= array_len {
                    return Err(
                        AsgError::array_index_out_of_bounds(index, &array.span().cloned().unwrap_or_default()).into(),
                    );
                }
            }
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
