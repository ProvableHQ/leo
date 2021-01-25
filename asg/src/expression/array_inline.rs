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
use leo_ast::SpreadOrExpression;
use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

pub struct ArrayInlineExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub elements: Vec<(Arc<Expression>, bool)>, // bool = if spread
}

impl ArrayInlineExpression {
    pub fn expanded_length(&self) -> usize {
        self.elements
            .iter()
            .map(|(expr, is_spread)| {
                if *is_spread {
                    match expr.get_type() {
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

impl Node for ArrayInlineExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for ArrayInlineExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.elements.iter().for_each(|(element, _)| {
            element.set_parent(Arc::downgrade(expr));
        })
    }

    fn get_type(&self) -> Option<Type> {
        Some(Type::Array(
            Box::new(self.elements.first()?.0.get_type()?),
            self.expanded_length(),
        ))
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue> {
        let mut const_values = vec![];
        for (expr, spread) in self.elements.iter() {
            if *spread {
                match expr.const_value()? {
                    ConstValue::Array(items) => const_values.extend(items),
                    _ => return None,
                }
            } else {
                const_values.push(expr.const_value()?);
            }
        }
        Some(ConstValue::Array(const_values))
    }
}

impl FromAst<leo_ast::ArrayInlineExpression> for ArrayInlineExpression {
    fn from_ast(
        scope: &Scope,
        value: &leo_ast::ArrayInlineExpression,
        expected_type: Option<PartialType>,
    ) -> Result<ArrayInlineExpression, AsgConvertError> {
        let (mut expected_item, expected_len) = match expected_type {
            Some(PartialType::Array(item, dims)) => (item.map(|x| *x), dims),
            None => (None, None),
            Some(type_) => {
                return Err(AsgConvertError::unexpected_type(
                    &type_.to_string(),
                    Some("array"),
                    &value.span,
                ));
            }
        };

        let mut len = 0;
        let output = ArrayInlineExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            elements: value
                .elements
                .iter()
                .map(|e| match e {
                    SpreadOrExpression::Expression(e) => {
                        let expr = Arc::<Expression>::from_ast(scope, e, expected_item.clone())?;
                        if expected_item.is_none() {
                            expected_item = expr.get_type().map(Type::partial);
                        }
                        len += 1;
                        Ok((expr, false))
                    }
                    SpreadOrExpression::Spread(e) => {
                        let expr = Arc::<Expression>::from_ast(
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
                                return Err(AsgConvertError::unexpected_type(
                                    expected_item
                                        .as_ref()
                                        .map(|x| x.to_string())
                                        .as_deref()
                                        .unwrap_or("unknown"),
                                    type_.map(|x| x.to_string()).as_deref(),
                                    &value.span,
                                ));
                            }
                        }
                        Ok((expr, true))
                    }
                })
                .collect::<Result<Vec<_>, AsgConvertError>>()?,
        };
        if let Some(expected_len) = expected_len {
            if len != expected_len {
                return Err(AsgConvertError::unexpected_type(
                    &*format!("array of length {}", expected_len),
                    Some(&*format!("array of length {}", len)),
                    &value.span,
                ));
            }
        }
        Ok(output)
    }
}

impl Into<leo_ast::ArrayInlineExpression> for &ArrayInlineExpression {
    fn into(self) -> leo_ast::ArrayInlineExpression {
        leo_ast::ArrayInlineExpression {
            elements: self
                .elements
                .iter()
                .map(|(element, spread)| {
                    let element = element.as_ref().into();
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
