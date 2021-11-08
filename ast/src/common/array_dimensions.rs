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

/// Type for handling different definitions of ArrayDimensions.
/// Can be used in an array [`Type`] or an array initializer [`Expression`].
#[derive(Clone, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum ArrayDimensions {
    Unspecified,
    Number(PositiveNumber),
    Multi(Vec<ArrayDimensions>),
}

impl ArrayDimensions {
    /// Returns `true` if the all array dimensions have been removed.
    ///
    /// This method is called after repeated calls to `remove_first`.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Multi(dimensions) => dimensions.len() != 0,
            Self::Number(_) | Self::Unspecified => false,
        }
    }

    pub fn is_specified(&self) -> bool {
        match self {
            Self::Multi(_) | Self::Number(_) => true,
            Self::Unspecified => false,
        }
    }

    pub fn flatten(&self) -> Vec<ArrayDimensions> {
        match self {
            dimension @ (ArrayDimensions::Number(_) | ArrayDimensions::Unspecified) => vec![dimension.clone()],
            ArrayDimensions::Multi(dimensions) => dimensions.iter().flat_map(|dim| dim.flatten()).collect(),
        }
    }

    /// Returns `true` if there is an array dimension equal to zero.
    pub fn is_zero(&self) -> bool {
        match self {
            ArrayDimensions::Multi(dimensions) => dimensions.iter().any(|a| a.is_zero()),
            ArrayDimensions::Number(num) => num.is_zero(),
            ArrayDimensions::Unspecified => false,
        }
    }

    /// Attempts to remove the first dimension from the array.
    ///
    /// If the first dimension exists, then remove and return `Some(PositiveNumber)`.
    /// If the first dimension does not exist, then return `None`.
    pub fn remove_first(&mut self) -> Option<ArrayDimensions> {
        match self {
            Self::Multi(dims) => Some(dims.remove(0)),
            _ => None,
        }
    }

    /// Attempts to remove the last dimension from the array.
    ///
    /// If the last dimension exists, then remove and return `Some(PositiveNumber)`.
    /// If the last dimension does not exist, then return `None`.
    pub fn remove_last(&mut self) -> Option<ArrayDimensions> {
        match self {
            Self::Multi(dims) => dims.pop(),
            _ => None,
        }
    }
}

impl ArrayDimensions {
    /// Returns a PositiveNumber if Dimension is specified, returns None otherwise.
    pub fn get_number(&self) -> Option<&PositiveNumber> {
        match self {
            ArrayDimensions::Number(num) => Some(num),
            ArrayDimensions::Unspecified => None,
            ArrayDimensions::Multi(_) => None,
        }
    }
}

/// Custom Serializer for ArrayDimensios is required to ignore internal ArrayDimension nodes in the AST.
impl Serialize for ArrayDimensions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dimensions = self.flatten();
        let mut seq = serializer.serialize_seq(Some(dimensions.len()))?;
        for dimension in dimensions.iter() {
            match dimension {
                ArrayDimensions::Number(num) => seq.serialize_element(&num)?,
                ArrayDimensions::Unspecified => seq.serialize_element(&PositiveNumber { value: "0".into() })?,
                ArrayDimensions::Multi(_) => unimplemented!("there are no multi dimensions after flattening"),
            }
        }
        seq.end()
    }
}

/// Create a new [`ArrayDimensions`] from a [`InputArrayDimensions`] in a Leo program file.
impl<'ast> From<InputArrayDimensions<'ast>> for ArrayDimensions {
    fn from(dimensions: InputArrayDimensions<'ast>) -> Self {
        match dimensions {
            InputArrayDimensions::Single(single) => ArrayDimensions::Number(PositiveNumber::from(single.number)),
            InputArrayDimensions::Multiple(multiple) => ArrayDimensions::Multi(
                multiple
                    .numbers
                    .into_iter()
                    .map(|num| ArrayDimensions::Number(PositiveNumber::from(num)))
                    .collect(),
            ),
        }
    }
}

impl fmt::Display for ArrayDimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArrayDimensions::Unspecified => write!(f, "_"),
            ArrayDimensions::Number(num) => write!(f, "{}", num),
            ArrayDimensions::Multi(dimensions) => {
                write!(
                    f,
                    "({})",
                    dimensions.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
                )
            }
        }
    }
}
