// Copyright (C) 2019-2026 Provable Inc.
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

use crate::{Expression, Path, Type};
use itertools::Itertools as _;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A composite type of a identifier and external program name.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompositeType {
    /// The path to the composite definition.
    pub path: Path,
    /// Expressions for the const arguments passed to the composite's const parameters.
    pub const_arguments: Vec<Expression>,
}

impl fmt::Display for CompositeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path)?;

        if !self.const_arguments.is_empty() {
            write!(f, "::[{}]", self.const_arguments.iter().format(", "))?;
        }

        Ok(())
    }
}

impl From<CompositeType> for Type {
    fn from(value: CompositeType) -> Self {
        Type::Composite(value)
    }
}
