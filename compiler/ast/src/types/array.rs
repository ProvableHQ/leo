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

use crate::{NonNegativeNumber, Type};

use serde::{Deserialize, Serialize};
use std::fmt;

/// An array type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArrayType {
    element_type: Box<Type>,
    length: NonNegativeNumber,
}

impl ArrayType {
    /// Creates a new array type.
    pub fn new(element: Type, length: NonNegativeNumber) -> Self {
        Self { element_type: Box::new(element), length }
    }

    /// Returns the element type of the array.
    pub fn element_type(&self) -> &Type {
        &self.element_type
    }

    /// Returns the length of the array.
    pub fn length(&self) -> usize {
        self.length.value()
    }

    /// Returns the base element type of the array.
    pub fn base_element_type(&self) -> &Type {
        match self.element_type.as_ref() {
            Type::Array(array_type) => array_type.base_element_type(),
            type_ => type_,
        }
    }
}

impl fmt::Display for ArrayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}; {}]", self.element_type, self.length)
    }
}
