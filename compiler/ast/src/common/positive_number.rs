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

/// A number string guaranteed to be positive.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct PositiveNumber {
    /// The string representation of the positive number.
    // FIXME(Centril): This should become an `u128`.
    pub value: String,
}

impl PositiveNumber {
    /// Returns `true` if this number is zero.
    pub fn is_zero(&self) -> bool {
        self.value.eq("0")
    }

    /// Converts the positive number into a `usize` or panics if it was malformed.
    pub fn to_usize(&self) -> usize {
        usize::from_str(&self.value).expect("failed to parse positive number")
    }
}

impl fmt::Display for PositiveNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
