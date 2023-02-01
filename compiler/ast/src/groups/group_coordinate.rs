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

use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A coordinate in a affine group literal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupCoordinate {
    /// A number, e.g., `42`.
    Number(String, #[serde(with = "leo_span::span_json")] Span),
    /// A sign high recovery, i.e. `+`.
    SignHigh,
    /// A sign low recovery, i.e., `-`.
    SignLow,
    /// Recovery with an inferred value.
    Inferred,
}

impl fmt::Display for GroupCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GroupCoordinate::Number(number, _) => write!(f, "{number}"),
            GroupCoordinate::SignHigh => write!(f, "+"),
            GroupCoordinate::SignLow => write!(f, "-"),
            GroupCoordinate::Inferred => write!(f, "_"),
        }
    }
}
