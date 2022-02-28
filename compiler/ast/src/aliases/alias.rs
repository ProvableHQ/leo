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

use crate::{Identifier, Type};
use leo_span::Span;

use std::fmt;

use serde::{Deserialize, Serialize};

/// A type alias `type name = represents;`.
///
/// That is, `name` will become another name for `represents`.
/// This does not create a new type, that is, `name` is the same type as `represents`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Alias {
    /// The new name for `represents`.
    pub name: Identifier,
    /// A span for the entire `type name = represents;`.
    pub span: Span,
    /// The type that `name` will evaluate and is equal to.
    pub represents: Type,
}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.name.name, self.represents)
    }
}
