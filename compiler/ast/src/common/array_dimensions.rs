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

use crate::PositiveNumber;
use leo_input::types::ArrayDimensions as InputArrayDimensions;

use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};
use smallvec::{smallvec, SmallVec};
use std::{fmt, ops::Deref};

/// A single array dimension.
#[derive(Clone, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum Dimension {
    /// The dimension is `_`, that is unspecified and syntactically unknown.
    Unspecified,
    /// The dimension was specified, e.g., `5` elements.
    Number(PositiveNumber),
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unspecified => write!(f, "_"),
            Self::Number(num) => write!(f, "{}", num),
        }
    }
}

impl Dimension {
    /// } Returns `Some(n)` unless the dimension is [`Unspecified`].
    pub fn as_specified(&self) -> Option<&PositiveNumber> {
        match self {
            Self::Unspecified => None,
            Self::Number(n) => Some(n),
        }
    }

    /// Returns true if the dimension is known to be zero.
    fn is_zero(&self) -> bool {
        self.as_specified().filter(|n| n.is_zero()).is_some()
    }
}

/// Specifies array dimensions for array [`Type`]s or in array initializer [`Expression`]s.
#[derive(Clone, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct ArrayDimensions(pub SmallVec<[Dimension; 1]>);

impl Deref for ArrayDimensions {
    type Target = [Dimension];

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl ArrayDimensions {
    /// Returns a single-dimensional array dimension.
    pub fn single(dim: Dimension) -> Self {
        Self(smallvec![dim])
    }

    /// Returns true if the dimensions are not [`Unspecified`].
    pub fn is_specified(&self) -> bool {
        !self.contains(&Dimension::Unspecified)
    }

    /// Returns `true` if there is an array dimension equal to zero.
    pub fn is_zero(&self) -> bool {
        self.iter().any(|d| d.is_zero())
    }

    /// Attempts to remove the first dimension from the array, or returns `None` if it doesn't.
    pub fn remove_first(&mut self) -> Option<Dimension> {
        if self.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }

    /// Attempts to remove the last dimension from the array, or returns `None` if it doesn't.
    pub fn remove_last(&mut self) -> Option<Dimension> {
        self.0.pop()
    }
}

/// Custom Serializer for ArrayDimensios is required to ignore internal ArrayDimension nodes in the AST.
impl Serialize for ArrayDimensions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for dim in self.0.iter() {
            match dim {
                Dimension::Number(num) => seq.serialize_element(&num)?,
                Dimension::Unspecified => seq.serialize_element(&PositiveNumber { value: "0".into() })?,
            }
        }
        seq.end()
    }
}

/// Create a new [`ArrayDimensions`] from a [`InputArrayDimensions`] in a Leo program file.
impl<'ast> From<InputArrayDimensions<'ast>> for ArrayDimensions {
    fn from(dimensions: InputArrayDimensions<'ast>) -> Self {
        match dimensions {
            InputArrayDimensions::Single(single) => {
                Self(smallvec![Dimension::Number(PositiveNumber::from(single.number))])
            }
            InputArrayDimensions::Multiple(multiple) => Self(
                multiple
                    .numbers
                    .into_iter()
                    .map(|num| Dimension::Number(PositiveNumber::from(num)))
                    .collect(),
            ),
        }
    }
}

impl fmt::Display for ArrayDimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self.0 {
            [dim] => write!(f, "{}", dim),
            dimensions => write!(
                f,
                "({})",
                dimensions.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
            ),
        }
    }
}
