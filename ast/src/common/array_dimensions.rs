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

use crate::PositiveNumber;
use leo_input::types::ArrayDimensions as InputArrayDimensions;

use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};
use std::fmt;

/// Internal type for handling unspecified size in `(_)` (parenthesis) array
/// definiton.
///
/// It has no representation on the AST level, its contents are displayed directly.
#[derive(Clone, Deserialize, Debug, PartialEq, Serialize, Eq, Hash)]
pub enum ArrayDimension {
    Number(PositiveNumber),
    Unspecified,
}

/// A vector of positive numbers that represent array dimensions.
/// Can be used in an array [`Type`] or an array initializer [`Expression`].
#[derive(Clone, Deserialize, Debug, PartialEq, Eq, Default, Hash)]
pub struct ArrayDimensions(pub Vec<ArrayDimension>);

impl ArrayDimensions {
    /// Appends a vector of array dimensions to the self array dimensions.
    pub fn append(&mut self, other: &mut ArrayDimensions) {
        self.0.append(&mut other.0)
    }

    /// Returns the array dimensions as strings.
    pub fn to_strings(&self) -> Vec<String> {
        self.0.iter().map(|number| number.to_string()).collect()
    }

    /// Returns `true` if the all array dimensions have been removed.
    ///
    /// This method is called after repeated calls to `remove_first`.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if there is an array dimension equal to zero.
    pub fn is_zero(&self) -> bool {
        self.0.iter().any(|dimension| match dimension {
            ArrayDimension::Number(num) => num.is_zero(),
            ArrayDimension::Unspecified => false,
        })
    }

    /// Returns the first dimension of the array.
    pub fn first(&self) -> Option<&ArrayDimension> {
        self.0.first()
    }

    /// Attempts to remove the first dimension from the array.
    ///
    /// If the first dimension exists, then remove and return `Some(PositiveNumber)`.
    /// If the first dimension does not exist, then return `None`.
    pub fn remove_first(&mut self) -> Option<ArrayDimension> {
        // If there are no dimensions in the array, then return None.
        self.0.first()?;

        // Remove the first dimension.
        let removed = self.0.remove(0);

        // Return the first dimension.
        Some(removed)
    }

    /// Attempts to remove the last dimension from the array.
    ///
    /// If the last dimension exists, then remove and return `Some(PositiveNumber)`.
    /// If the last dimension does not exist, then return `None`.
    pub fn remove_last(&mut self) -> Option<ArrayDimension> {
        self.0.pop()
    }
}

impl ArrayDimension {
    /// Returns a PositiveNumber if Dimension is specified, returns None otherwise.
    pub fn get_number(&self) -> Option<&PositiveNumber> {
        match self {
            ArrayDimension::Number(num) => Some(num),
            ArrayDimension::Unspecified => None,
        }
    }
}

impl fmt::Display for ArrayDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArrayDimension::Number(num) => num.fmt(f),
            ArrayDimension::Unspecified => write!(f, "_"),
        }
    }
}

/// Custom Serializer for ArrayDimensios is required to ignore internal ArrayDimension nodes in the AST.
impl Serialize for ArrayDimensions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for dimension in self.0.iter() {
            match dimension {
                ArrayDimension::Number(num) => seq.serialize_element(&num)?,
                ArrayDimension::Unspecified => seq.serialize_element(&PositiveNumber { value: "0".into() })?,
            }
        }
        seq.end()
    }
}

/// Create a new [`ArrayDimensions`] from a [`InputArrayDimensions`] in a Leo program file.
impl<'ast> From<InputArrayDimensions<'ast>> for ArrayDimensions {
    fn from(dimensions: InputArrayDimensions<'ast>) -> Self {
        Self(match dimensions {
            InputArrayDimensions::Single(single) => vec![ArrayDimension::Number(PositiveNumber::from(single.number))],
            InputArrayDimensions::Multiple(multiple) => multiple
                .numbers
                .into_iter()
                .map(|num| ArrayDimension::Number(PositiveNumber::from(num)))
                .collect(),
        })
    }
}

impl fmt::Display for ArrayDimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.len() == 1 {
            // Write dimensions without parenthesis.
            write!(f, "{}", self.0[0])
        } else {
            // Write dimensions with parenthesis.
            let dimensions = self.0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");

            write!(f, "({})", dimensions)
        }
    }
}
