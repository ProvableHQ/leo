// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::{Identifier};

use leo_span::Symbol;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A struct type of a identifier and optional external program name.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Copy)]
pub struct StructType {
    // The identifier of the struct definition.
    pub id: Identifier,
    // The external program that this struct is defined in.
    pub external: Option<Symbol>,
}

impl fmt::Display for StructType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.external {
            Some(external) => write!(f, "{external}/{id}", id = self.id),
            None => write!(f, "{id}", id = self.id),
        }
    }
}
