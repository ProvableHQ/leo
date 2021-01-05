// Copyright (C) 2019-2020 Aleo Systems Inc.
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
use leo_grammar::types::ArrayDimensions as GrammarArrayDimensions;
use leo_input::types::ArrayDimensions as InputArrayDimensions;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A vector of positive numbers that represent array dimensions.
/// Can be used in an array [`Type`] or an array initializer [`Expression`].
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Default, Hash)]
pub struct ArrayDimensions(pub Vec<PositiveNumber>);

impl ArrayDimensions {
    ///
    /// Creates a new `PositiveNumber` from the given `usize` and `Span`.
    /// Appends the new `PositiveNumber` to the array dimensions.
    ///
    pub fn push_usize(&mut self, number: usize) {
        let positive_number = PositiveNumber {
            value: number.to_string(),
        };

        self.0.push(positive_number)
    }

    ///
    /// Appends a vector of array dimensions to the self array dimensions.
    ///
    pub fn append(&mut self, other: &mut ArrayDimensions) {
        self.0.append(&mut other.0)
    }

    ///
    /// Returns the array dimensions as strings.
    ///
    pub fn to_strings(&self) -> Vec<String> {
        self.0.iter().map(|number| number.to_string()).collect()
    }

    ///
    /// Returns `true` if the all array dimensions have been removed.
    ///
    /// This method is called after repeated calls to `remove_first`.
    ///
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    ///
    /// Returns `true` if there is an array dimension equal to zero.
    ///
    pub fn is_zero(&self) -> bool {
        self.0.iter().any(|number| number.is_zero())
    }

    ///
    /// Returns the first dimension of the array.
    ///
    pub fn first(&self) -> Option<&PositiveNumber> {
        self.0.first()
    }

    ///
    /// Attempts to remove the first dimension from the array.
    ///
    /// If the first dimension exists, then remove and return `Some(PositiveNumber)`.
    /// If the first dimension does not exist, then return `None`.
    ///
    pub fn remove_first(&mut self) -> Option<PositiveNumber> {
        // If there are no dimensions in the array, then return None.
        self.0.first()?;

        // Remove the first dimension.
        let removed = self.0.remove(0);

        // Return the first dimension.
        Some(removed)
    }

    ///
    /// Attempts to remove the last dimension from the array.
    ///
    /// If the last dimension exists, then remove and return `Some(PositiveNumber)`.
    /// If the last dimension does not exist, then return `None`.
    ///
    pub fn remove_last(&mut self) -> Option<PositiveNumber> {
        self.0.pop()
    }
}

/// Create a new [`ArrayDimensions`] from a [`GrammarArrayDimensions`] in a Leo program file.
impl<'ast> From<GrammarArrayDimensions<'ast>> for ArrayDimensions {
    fn from(dimensions: GrammarArrayDimensions<'ast>) -> Self {
        Self(match dimensions {
            GrammarArrayDimensions::Single(single) => vec![PositiveNumber::from(single.number)],
            GrammarArrayDimensions::Multiple(multiple) => {
                multiple.numbers.into_iter().map(PositiveNumber::from).collect()
            }
        })
    }
}

/// Create a new [`ArrayDimensions`] from a [`InputArrayDimensions`] in a Leo program file.
impl<'ast> From<InputArrayDimensions<'ast>> for ArrayDimensions {
    fn from(dimensions: InputArrayDimensions<'ast>) -> Self {
        Self(match dimensions {
            InputArrayDimensions::Single(single) => vec![PositiveNumber::from(single.number)],
            InputArrayDimensions::Multiple(multiple) => {
                multiple.numbers.into_iter().map(PositiveNumber::from).collect()
            }
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
