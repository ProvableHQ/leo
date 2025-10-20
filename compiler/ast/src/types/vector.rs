// Copyright (C) 2019-2025 Provable Inc.
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

/// A vector type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VectorType {
    pub element_type: Box<Type>,
}

impl VectorType {
    /// Creates a new vector type.
    pub fn new(element: Type) -> Self {
        Self { element_type: Box::new(element) }
    }

    /// Returns the element type of the vector.
    pub fn element_type(&self) -> &Type {
        &self.element_type
    }
}

impl fmt::Display for VectorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.element_type)
    }
}

impl From<VectorType> for Type {
    fn from(value: VectorType) -> Self {
        Type::Vector(value)
    }
}
