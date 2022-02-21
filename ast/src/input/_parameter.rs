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

use super::*;
use crate::{Identifier, Type};

/// A set of properties for a single definition in an input file.
/// Used as a key in [`ProgramInput`] and [`ProgramState`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Parameter {
    pub variable: Identifier,
    pub type_: Type,
    pub span: Span,
}

impl From<Definition> for Parameter {
    fn from(definition: Definition) -> Self {
        Self {
            variable: definition.name,
            type_: definition.type_,
            span: definition.span,
        }
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.variable, self.type_)
    }
}

/// Parameter is a key, so for allowing its JSON representation,
/// we need to make a string.
impl Serialize for Parameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
