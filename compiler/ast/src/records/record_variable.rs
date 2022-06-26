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
use leo_span::Symbol;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A variable definition in a record;
/// For example: `owner: address;`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordVariable {
    /// The identifier of the constant.
    pub ident: Identifier,
    /// The type the constant has.
    pub type_: Type,
}

impl RecordVariable {
    pub fn new(ident: Identifier, type_: Type) -> Self {
        Self { ident, type_ }
    }

    pub fn name(&self) -> Symbol {
        return self.ident.name;
    }
}

impl fmt::Display for RecordVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.ident, self.type_)
    }
}
