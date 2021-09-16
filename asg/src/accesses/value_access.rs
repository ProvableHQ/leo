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
use leo_errors::{AsgError, Result, Span};

use std::cell::Cell;

#[derive(Clone)]
pub struct ValueAccess<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub target: Cell<&'a Expression<'a>>,
    pub access: Cell<&'a Expression<'a>>,
}

impl<'a> Node for ValueAccess<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for ValueAccess<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        self.target.get().set_parent(expr);
    }

    fn get_type(&self) -> Option<Type<'a>> {
        self.target.get().get_type()
    }

    fn is_mut_ref(&self) -> bool {
        self.target.get().is_mut_ref()
    }

    fn const_value(&self) -> Option<ConstValue<'a>> {
        self.target.get().const_value()
    }

    fn is_consty(&self) -> bool {
        true
    }
}

impl<'a> FromAst<'a, leo_ast::ValueAccess> for ValueAccess<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::ValueAccess,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<ValueAccess<'a>> {
        let target = <&'a Expression<'a>>::from_ast(scope, &*value.value, expected_type)?;
        // TODO make expected type for this an whatever to_bits/bytes should return.
        let access = <&'a Expression<'a>>::from_ast(scope, &*value.access, None)?;

        match target.get_type() {
            Some(Type::Array(_, _)) | Some(Type::ArrayWithoutSize(_)) => {
                return Err(AsgError::unexpected_type("scalar type", "array", &value.span).into())
            }
            Some(Type::Tuple(_)) => return Err(AsgError::unexpected_type("scalar type", "tuple", &value.span).into()),
            Some(Type::Circuit(circuit)) => {
                return Err(AsgError::unexpected_type("scalar type ", circuit.name.borrow(), &value.span).into())
            }
            _ => {}
        };

        Ok(ValueAccess {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            target: Cell::new(target),
            access: Cell::new(access),
        })
    }
}

impl<'a> Into<leo_ast::ValueAccess> for &ValueAccess<'a> {
    fn into(self) -> leo_ast::ValueAccess {
        leo_ast::ValueAccess {
            value: Box::new(self.target.get().into()),
            access: Box::new(self.access.get().into()),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
