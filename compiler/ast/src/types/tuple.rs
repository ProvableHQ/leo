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

use crate::Type;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A type list of at least two types.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TupleType {
    elements: Vec<Type>,
}

impl TupleType {
    /// Creates a new tuple type.
    pub fn new(elements: Vec<Type>) -> Self {
        Self { elements }
    }

    /// Returns the elements of the tuple type.
    pub fn elements(&self) -> &[Type] {
        &self.elements
    }

    /// Returns the length of the tuple type.
    pub fn length(&self) -> usize {
        self.elements.len()
    }
}

impl fmt::Display for TupleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.elements.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
    }
}
