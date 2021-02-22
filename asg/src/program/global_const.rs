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

use crate::{AsgConvertError, ExpressionStatement, Identifier, Node, Scope, Span, Type};

use uuid::Uuid;

#[derive(Clone)]
pub struct Define<'a> {
    pub id: Uuid,
    pub name: Identifier,
    pub expression: ExpressionStatement<'a>,
}

impl<'a> PartialEq for Define<'a> {
    fn eq(&self, other: &Define) -> bool {
        if self.name != other.name {
            return false;
        }
        self.id == other.id
    }
}

impl<'a> Eq for Define<'a> {}
