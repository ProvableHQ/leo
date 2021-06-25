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
pub struct ArrayRangeAccessExpression<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub array: Cell<&'a Expression<'a>>,
    pub left: Cell<Option<&'a Expression<'a>>>,
    pub right: Cell<Option<&'a Expression<'a>>>,
    // this is either const(right) - const(left) OR the length inferred by type checking
    // special attention must be made to update this if semantic-altering changes are made to left or right.
    pub length: u32,
}

impl<'a> Node for ArrayRangeAccessExpression<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for ArrayRangeAccessExpression<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        self.array.get().set_parent(expr);
        self.array.get().enforce_parents(self.array.get());
        if let Some(left) = self.left.get() {
            left.set_parent(expr);
        }
        if let Some(right) = self.right.get() {
            right.set_parent(expr);
        }
    }

    fn get_type(&self) -> Option<Type<'a>> {
        let element = match self.array.get().get_type() {
            Some(Type::Array(element, _)) => element,
            _ => return None,
        };

        Some(Type::Array(element, self.length))
    }

    fn is_mut_ref(&self) -> bool {
        self.array.get().is_mut_ref()
    }

    fn const_value(&self) -> Option<ConstValue> {
        let mut array = match self.array.get().const_value()? {
            ConstValue::Array(values) => values,
            _ => return None,
        };
        let const_left = match self.left.get().map(|x| x.const_value()) {
            Some(Some(ConstValue::Int(x))) => x.to_usize()?,
            None => 0,
            _ => return None,
        };
        let const_right = match self.right.get().map(|x| x.const_value()) {
            Some(Some(ConstValue::Int(x))) => x.to_usize()?,
            None => array.len(),
            _ => return None,
        };
        if const_left > const_right || const_right as usize > array.len() {
            return None;
        }

        Some(ConstValue::Array(array.drain(const_left..const_right).collect()))
    }

    fn is_consty(&self) -> bool {
        self.array.get().is_consty()
    }
}

impl<'a> FromAst<'a, leo_ast::ArrayRangeAccessExpression> for ArrayRangeAccessExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::ArrayRangeAccessExpression,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<ArrayRangeAccessExpression<'a>, AsgConvertError> {
        let (expected_array, expected_len) = match expected_type.clone() {
            Some(PartialType::Array(element, len)) => (Some(PartialType::Array(element, None)), len),
            None => (None, None),
            Some(x) => {
                return Err(AsgConvertError::unexpected_type(
                    &x.to_string(),
                    Some("array"),
                    &value.span,
                ));
            }
        };
        let array = <&Expression<'a>>::from_ast(scope, &*value.array, expected_array)?;
        let array_type = array.get_type();
        let (parent_element, parent_size) = match array_type {
            Some(Type::Array(inner, size)) => (inner, size),
            type_ => {
                return Err(AsgConvertError::unexpected_type(
                    "array",
                    type_.map(|x| x.to_string()).as_deref(),
                    &value.span,
                ));
            }
        };

        let left = value
            .left
            .as_deref()
            .map(|left| {
                <&Expression<'a>>::from_ast(scope, left, Some(PartialType::Integer(None, Some(IntegerType::U32))))
            })
            .transpose()?;
        let right = value
            .right
            .as_deref()
            .map(|right| {
                <&Expression<'a>>::from_ast(scope, right, Some(PartialType::Integer(None, Some(IntegerType::U32))))
            })
            .transpose()?;

        let const_left = match left.map(|x| x.const_value()) {
            Some(Some(ConstValue::Int(x))) => x.to_usize().map(|x| x as u32),
            None => Some(0),
            _ => None,
        };
        let const_right = match right.map(|x| x.const_value()) {
            Some(Some(ConstValue::Int(value))) => {
                let value = value.to_usize().map(|x| x as u32);
                if let Some(value) = value {
                    if value > parent_size {
                        return Err(AsgConvertError::array_index_out_of_bounds(
                            value,
                            &right.unwrap().span().cloned().unwrap_or_default(),
                        ));
                    } else if let Some(left) = const_left {
                        if left > value {
                            return Err(AsgConvertError::array_index_out_of_bounds(
                                value,
                                &right.unwrap().span().cloned().unwrap_or_default(),
                            ));
                        }
                    }
                }
                value
            }
            None => Some(parent_size),
            _ => None,
        };

        let mut length = if let (Some(left), Some(right)) = (const_left, const_right) {
            Some(right - left)
        } else {
            None
        };
        if let Some(expected_len) = expected_len {
            if let Some(length) = length {
                if length != expected_len {
                    let concrete_type = Type::Array(parent_element, length);
                    return Err(AsgConvertError::unexpected_type(
                        &expected_type.as_ref().unwrap().to_string(),
                        Some(&concrete_type.to_string()),
                        &value.span,
                    ));
                }
            }
            if let Some(value) = const_left {
                if value + expected_len > parent_size {
                    return Err(AsgConvertError::array_index_out_of_bounds(
                        value,
                        &left.unwrap().span().cloned().unwrap_or_default(),
                    ));
                }
            }
            length = Some(expected_len);
        }
        if length.is_none() {
            return Err(AsgConvertError::unknown_array_size(&value.span));
        }

        Ok(ArrayRangeAccessExpression {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            array: Cell::new(array),
            left: Cell::new(left),
            right: Cell::new(right),
            length: length.unwrap(),
        })
    }
}

impl<'a> Into<leo_ast::ArrayRangeAccessExpression> for &ArrayRangeAccessExpression<'a> {
    fn into(self) -> leo_ast::ArrayRangeAccessExpression {
        leo_ast::ArrayRangeAccessExpression {
            array: Box::new(self.array.get().into()),
            left: self.left.get().map(|left| Box::new(left.into())),
            right: self.right.get().map(|right| Box::new(right.into())),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
