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

use leo_input::values::PositiveNumber as InputPositiveNumber;

use serde::{Deserialize, Serialize};
use std::fmt;
use tendril::StrTendril;

/// A number string guaranteed to be positive by the pest grammar.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct PositiveNumber {
    #[serde(with = "leo_errors::common::tendril_json")]
    pub value: StrTendril,
}

impl PositiveNumber {
    ///
    /// Returns `true` if this number is zero.
    ///
    pub fn is_zero(&self) -> bool {
        self.value.as_ref().eq("0")
    }
}

/// Create a new [`PositiveNumber`] from an [`InputPositiveNumber`]  in a Leo input file.
impl<'ast> From<InputPositiveNumber<'ast>> for PositiveNumber {
    fn from(array: InputPositiveNumber<'ast>) -> Self {
        Self {
            value: array.value.into(),
        }
    }
}

impl fmt::Display for PositiveNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
