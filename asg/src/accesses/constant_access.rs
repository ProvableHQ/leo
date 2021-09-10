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

use crate::{Constant, ConstValue, Expression, ExpressionNode, FromAst, Identifier, Node, PartialType, Scope, Type};
use leo_errors::{AsgError, Result, Span};

use std::cell::Cell;

#[derive(Clone)]
pub struct ConstantAccess<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub value: Cell<&'a Constant<'a>>,
    pub target: Cell<Option<&'a Expression<'a>>>,
    pub member: Identifier,
}

impl<'a> Node for ConstantAccess<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for ConstantAccess<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        if let Some(target) = self.target.get() {
            target.set_parent(expr);
        }
    }

    fn get_type(&self) -> Option<Type<'a>> {
        None
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue<'a>> {
        match self.target.get()?.const_value()? {
            ConstValue::Circuit(_, members) => {
                let (_, const_value) = members.get(&self.member.name.to_string())?.clone();
                Some(const_value)
            }
            _ => None,
        }
    }

    fn is_consty(&self) -> bool {
        true
    }
}

impl<'a> FromAst<'a, leo_ast::ValueAccess> for ConstantAccess<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::ValueAccess,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<ConstantAccess<'a>> {
        let target = <&'a Expression<'a>>::from_ast(scope, &*value.value, None)?;
    }
}

impl<'a> Into<leo_ast::ValueAccess> for &ConstantAccess<'a> {
    fn into(self) -> leo_ast::ValueAccess {
        leo_ast::ValueAccess {
            value: Box::new(leo_ast::Expression::Value(self.value.into())),
            name: self.member.clone(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
