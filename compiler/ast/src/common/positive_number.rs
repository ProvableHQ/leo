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

use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// A number string guaranteed to be non-negative.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct NonNegativeNumber {
    /// The string representation of the non-negative number.
    string: String,
    /// The numeric value of the non-negative number.
    value: usize,
}

impl NonNegativeNumber {
    /// Returns the string representation of the non-negative number.
    pub fn string(&self) -> &str {
        &self.string
    }

    /// Returns the numeric value of the non-negative number.
    pub fn value(&self) -> usize {
        self.value
    }

    /// Returns `true` if this number is zero.
    pub fn is_zero(&self) -> bool {
        self.value == 0
    }
}

impl From<String> for NonNegativeNumber {
    fn from(string: String) -> Self {
        let value = usize::from_str(&string).unwrap();
        Self { string, value }
    }
}

impl From<usize> for NonNegativeNumber {
    fn from(value: usize) -> Self {
        let string = value.to_string();
        Self { string, value }
    }
}

impl fmt::Display for NonNegativeNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
