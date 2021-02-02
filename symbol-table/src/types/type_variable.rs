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
use leo_ast::Identifier;

use serde::{Deserialize, Serialize};
use std::fmt;

/// An unknown type in a Leo program.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypeVariable {
    identifier: Identifier,
}

impl fmt::Display for TypeVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl From<Identifier> for TypeVariable {
    fn from(identifier: Identifier) -> Self {
        Self { identifier }
    }
}

/// Compare the type variable `Identifier` and `Span`.
impl PartialEq for TypeVariable {
    fn eq(&self, other: &Self) -> bool {
        self.identifier.name.eq(&other.identifier.name) || self.identifier.span.eq(&other.identifier.span)
    }
}

impl Eq for TypeVariable {}
