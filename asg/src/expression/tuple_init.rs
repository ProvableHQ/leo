// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{AsgId, ConstValue, Expression, ExpressionNode, FromAst, Node, PartialType, Scope, Type};
use leo_errors::{AsgError, Result, Span};

use std::cell::Cell;

#[derive(Clone)]
pub struct TupleInitExpression<'a> {
    pub id: AsgId,
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub elements: Vec<Cell<&'a Expression<'a>>>,
}

impl<'a> Node for TupleInitExpression<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    fn asg_id(&self) -> AsgId {
        self.id
    }
}

impl<'a> ExpressionNode<'a> for TupleInitExpression<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        self.elements.iter().for_each(|element| {
            element.get().set_parent(expr);
        })
    }

    fn get_type(&self) -> Option<Type<'a>> {
        let mut output = vec![];
        for element in self.elements.iter() {
            output.push(element.get().get_type()?);
        }
        Some(Type::Tuple(output))
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue<'a>> {
        let mut consts = vec![];
        for element in self.elements.iter() {
            if let Some(const_value) = element.get().const_value() {
                consts.push(const_value);
            } else {
                return None;
            }
        }
        Some(ConstValue::Tuple(consts))
    }

    fn is_consty(&self) -> bool {
        self.elements.iter().all(|x| x.get().is_consty())
    }
}

impl<'a> FromAst<'a, leo_ast::TupleInitExpression> for TupleInitExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::TupleInitExpression,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<TupleInitExpression<'a>> {
        let tuple_types = match expected_type {
            Some(PartialType::Tuple(sub_types)) => Some(sub_types),
            None => None,
            x => {
                return Err(AsgError::unexpected_type(
                    "tuple",
                    x.map(|x| x.to_string()).unwrap_or_else(|| "unknown".to_string()),
                    &value.span,
                )
                .into());
            }
        };

        if let Some(tuple_types) = tuple_types.as_ref() {
            // Expected type can be equal or less than actual size of a tuple.
            // Size of expected tuple can be based on accessed index.
            if tuple_types.len() > value.elements.len() {
                return Err(AsgError::unexpected_type(
                    format!("tuple of length {}", tuple_types.len()),
                    format!("tuple of length {}", value.elements.len()),
                    &value.span,
                )
                .into());
            }
        }

        let elements = value
            .elements
            .iter()
            .enumerate()
            .map(|(i, e)| {
                <&Expression<'a>>::from_ast(
                    scope,
                    e,
                    tuple_types.as_ref().map(|x| x.get(i)).flatten().cloned().flatten(),
                )
                .map(Cell::new)
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(TupleInitExpression {
            id: scope.context.get_id(),
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            elements,
        })
    }
}

impl<'a> Into<leo_ast::TupleInitExpression> for &TupleInitExpression<'a> {
    fn into(self) -> leo_ast::TupleInitExpression {
        leo_ast::TupleInitExpression {
            elements: self.elements.iter().map(|e| e.get().into()).collect(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
