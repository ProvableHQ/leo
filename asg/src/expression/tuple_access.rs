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

pub struct TupleAccessExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub tuple_ref: Arc<Expression>,
    pub index: usize,
}

impl Node for TupleAccessExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for TupleAccessExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.tuple_ref.set_parent(Arc::downgrade(expr));
    }

    fn get_type(&self) -> Option<Type> {
        match self.tuple_ref.get_type()? {
            Type::Tuple(subtypes) => subtypes.get(self.index).cloned(),
            _ => None,
        }
    }

    fn is_mut_ref(&self) -> bool {
        self.tuple_ref.is_mut_ref()
    }

    fn const_value(&self) -> Option<ConstValue> {
        let tuple_const = self.tuple_ref.const_value()?;
        match tuple_const {
            ConstValue::Tuple(sub_consts) => sub_consts.get(self.index).cloned(),
            _ => None,
        }
    }

    fn is_consty(&self) -> bool {
        self.tuple_ref.is_consty()
    }
}

impl FromAst<leo_ast::TupleAccessExpression> for TupleAccessExpression {
    fn from_ast(
        scope: &Scope,
        value: &leo_ast::TupleAccessExpression,
        expected_type: Option<PartialType>,
    ) -> Result<TupleAccessExpression, AsgConvertError> {
        let index = value
            .index
            .value
            .parse::<usize>()
            .map_err(|_| AsgConvertError::parse_index_error())?;

        let mut expected_tuple = vec![None; index + 1];
        expected_tuple[index] = expected_type;

        let tuple = Arc::<Expression>::from_ast(scope, &*value.tuple, Some(PartialType::Tuple(expected_tuple)))?;
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
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            tuple_ref: tuple,
            index,
        })
    }
}

impl Into<leo_ast::TupleAccessExpression> for &TupleAccessExpression {
    fn into(self) -> leo_ast::TupleAccessExpression {
        leo_ast::TupleAccessExpression {
            tuple: Box::new(self.tuple_ref.as_ref().into()),
            index: leo_ast::PositiveNumber {
                value: self.index.to_string(),
            },
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
