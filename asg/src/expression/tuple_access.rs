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
pub struct TupleAccessExpression<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub tuple_ref: Cell<&'a Expression<'a>>,
    pub index: usize,
}

impl<'a> Node for TupleAccessExpression<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for TupleAccessExpression<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        self.tuple_ref.get().set_parent(expr);
    }

    fn get_type(&self) -> Option<Type<'a>> {
        match self.tuple_ref.get().get_type()? {
            Type::Tuple(subtypes) => subtypes.get(self.index).cloned(),
            _ => None,
        }
    }

    fn is_mut_ref(&self) -> bool {
        self.tuple_ref.get().is_mut_ref()
    }

    fn const_value(&self) -> Option<ConstValue> {
        let tuple_const = self.tuple_ref.get().const_value()?;
        match tuple_const {
            ConstValue::Tuple(sub_consts) => sub_consts.get(self.index).cloned(),
            _ => None,
        }
    }

    fn is_consty(&self) -> bool {
        self.tuple_ref.get().is_consty()
    }
}

impl<'a> FromAst<'a, leo_ast::TupleAccessExpression> for TupleAccessExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::TupleAccessExpression,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<TupleAccessExpression<'a>, AsgConvertError> {
        let index = value
            .index
            .value
            .parse::<usize>()
            .map_err(|_| AsgConvertError::parse_index_error())?;

        let mut expected_tuple = vec![None; index + 1];
        expected_tuple[index] = expected_type;

        let tuple = <&Expression<'a>>::from_ast(scope, &*value.tuple, Some(PartialType::Tuple(expected_tuple)))?;
        let tuple_type = tuple.get_type();
        if let Some(Type::Tuple(_items)) = tuple_type {
        } else {
            return Err(AsgConvertError::unexpected_type(
                "a tuple",
                tuple_type.map(|x| x.to_string()).as_deref(),
                &value.span,
            ));
        }

        Ok(TupleAccessExpression {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            tuple_ref: Cell::new(tuple),
            index,
        })
    }
}

impl<'a> Into<leo_ast::TupleAccessExpression> for &TupleAccessExpression<'a> {
    fn into(self) -> leo_ast::TupleAccessExpression {
        leo_ast::TupleAccessExpression {
            tuple: Box::new(self.tuple_ref.get().into()),
            index: leo_ast::PositiveNumber {
                value: self.index.to_string().into(),
            },
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
