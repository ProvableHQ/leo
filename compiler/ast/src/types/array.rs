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

/// An array type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArrayType {
    element_type: Box<Type>,
    size: u32,
}

impl ArrayType {
    /// Creates a new array type.
    pub fn new(element: Type, size: u32) -> Self {
        Self { element_type: Box::new(element), size }
    }

    /// Returns the element type of the array.
    pub fn element_type(&self) -> &Type {
        &self.element_type
    }

    /// Returns the size of the array.
    pub fn size(&self) -> u32 {
        self.size
    }
}

impl fmt::Display for ArrayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}; {}]", self.element_type, self.size)
    }
}
