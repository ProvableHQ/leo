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

use crate::{AsgId, Identifier, Node, Scope, Type};
use leo_errors::{Result, Span};

use std::cell::RefCell;

#[derive(Clone)]
pub struct Alias<'a> {
    pub id: AsgId,
    pub name: RefCell<Identifier>,
    pub span: Option<Span>,
    pub represents: Type<'a>,
}

impl<'a> PartialEq for Alias<'a> {
    fn eq(&self, other: &Alias) -> bool {
        if self.name != other.name {
            return false;
        }
        self.id == other.id
    }
}

impl<'a> Eq for Alias<'a> {}

impl<'a> Node for Alias<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    fn asg_id(&self) -> AsgId {
        self.id
    }
}

impl<'a> Alias<'a> {
    pub(super) fn init(scope: &'a Scope<'a>, value: &leo_ast::Alias) -> Result<&'a Alias<'a>> {
        let alias = scope.context.alloc_alias(Alias {
            id: scope.context.get_id(),
            name: RefCell::new(value.name.clone()),
            span: Some(value.span.clone()),
            represents: scope.resolve_ast_type(&value.represents, &value.span)?,
        });

        Ok(alias)
    }
}
