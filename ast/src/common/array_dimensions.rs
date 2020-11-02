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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArrayDimensions(pub Vec<PositiveNumber>);

impl ArrayDimensions {
    ///
    /// Returns the array dimensions as strings.
    ///
    pub fn to_strings(&self) -> Vec<String> {
        self.0.iter().map(|number| number.to_string()).collect()
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
