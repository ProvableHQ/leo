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
use leo_ast::SpreadOrExpression;
use leo_errors::{AsgError, Result, Span};

use std::cell::Cell;

#[derive(Clone)]
pub struct ArrayInlineExpression<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub elements: Vec<(Cell<&'a Expression<'a>>, bool)>, // bool = if spread
}

impl<'a> ArrayInlineExpression<'a> {
    pub fn expanded_length(&self) -> usize {
        self.elements
            .iter()
            .map(|(expr, is_spread)| {
                if *is_spread {
                    match expr.get().get_type() {
                        Some(Type::Array(_item, len)) => len,
                        _ => 0,
                    }
                } else {
                    1
                }
            })
            .sum()
    }
}

impl<'a> Node for ArrayInlineExpression<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for ArrayInlineExpression<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        self.elements.iter().for_each(|(element, _)| {
            element.get().set_parent(expr);
        })
    }

    fn get_type(&self) -> Option<Type<'a>> {
        let first = self.elements.first()?;
        let inner_type = match first.0.get().get_type()? {
            Type::Array(inner, _) if first.1 => *inner,
            _ => first.0.get().get_type()?,
        };
        Some(Type::Array(Box::new(inner_type), self.expanded_length()))
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue<'a>> {
        let mut const_values = vec![];
        for (expr, spread) in self.elements.iter() {
            if *spread {
                match expr.get().const_value()? {
                    ConstValue::Array(items) => const_values.extend(items),
                    _ => return None,
                }
            } else {
                const_values.push(expr.get().const_value()?);
            }
        }
        Some(ConstValue::Array(const_values))
    }

    fn is_consty(&self) -> bool {
        self.elements.iter().all(|x| x.0.get().is_consty())
    }
}

impl<'a> FromAst<'a, leo_ast::ArrayInlineExpression> for ArrayInlineExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::ArrayInlineExpression,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<ArrayInlineExpression<'a>> {
        let (mut expected_item, expected_len) = match expected_type {
            Some(PartialType::Array(item, dims)) => (item.map(|x| *x), dims),
            None => (None, None),
            Some(PartialType::Type(Type::UnsizedArray(item))) => (Some(item.clone().partial()), None),
            Some(type_) => {
                return Err(AsgError::unexpected_type(type_, "array", &value.span).into());
            }
        };

        // If we still don't know the type iterate through processing to get a type.
        // Once we encouter the type break the loop so we process as little as possible.
        if expected_item.is_none() {
            for expr in value.elements.iter() {
                expected_item = match expr {
                    SpreadOrExpression::Expression(e) => {
                        match <&Expression<'a>>::from_ast(scope, e, expected_item.clone()) {
                            Ok(expr) => expr.get_type().map(Type::partial),
                            Err(_) => continue,
                        }
                    }
                    _ => None,
                };

                if expected_item.is_some() {
                    break;
                }
            }
        }

        let mut len = 0;

        let output = ArrayInlineExpression {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            elements: value
                .elements
                .iter()
                .map(|e| match e {
                    SpreadOrExpression::Expression(e) => {
                        let expr = <&Expression<'a>>::from_ast(scope, e, expected_item.clone())?;
                        if expected_item.is_none() {
                            expected_item = expr.get_type().map(Type::partial);
                        }
                        len += 1;
                        Ok((Cell::new(expr), false))
                    }
                    SpreadOrExpression::Spread(e) => {
                        let expr = <&Expression<'a>>::from_ast(
                            scope,
                            e,
                            Some(PartialType::Array(expected_item.clone().map(Box::new), None)),
                        )?;

                        match expr.get_type() {
                            Some(Type::Array(item, spread_len)) => {
                                if expected_item.is_none() {
                                    expected_item = Some((*item).partial());
                                }

                                len += spread_len;
                            }
                            type_ => {
                                return Err(AsgError::unexpected_type(
                                    expected_item
                                        .as_ref()
                                        .map(|x| x.to_string())
                                        .as_deref()
                                        .unwrap_or("unknown"),
                                    type_.map(|x| x.to_string()).unwrap_or_else(|| "unknown".to_string()),
                                    &value.span,
                                )
                                .into());
                            }
                        }
                        Ok((Cell::new(expr), true))
                    }
                })
                .collect::<Result<Vec<_>>>()?,
        };
        if let Some(expected_len) = expected_len {
            if len != expected_len {
                return Err(AsgError::unexpected_type(
                    format!("array of length {}", expected_len),
                    format!("array of length {}", len),
                    &value.span,
                )
                .into());
            }
        }
        Ok(output)
    }
}

impl<'a> Into<leo_ast::ArrayInlineExpression> for &ArrayInlineExpression<'a> {
    fn into(self) -> leo_ast::ArrayInlineExpression {
        leo_ast::ArrayInlineExpression {
            elements: self
                .elements
                .iter()
                .map(|(element, spread)| {
                    let element = element.get().into();
                    if *spread {
                        SpreadOrExpression::Spread(element)
                    } else {
                        SpreadOrExpression::Expression(element)
                    }
                })
                .collect(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
