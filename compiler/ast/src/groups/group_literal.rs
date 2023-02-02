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

use crate::groups::GroupCoordinate;
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A group literal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupLiteral {
    /// Product group literal, e.g., `42group`.
    Single(String, #[serde(with = "leo_span::span_json")] Span),
    /// An affine group literal with (x, y) coordinates.
    Tuple(GroupTuple),
}

impl GroupLiteral {
    pub fn set_span(&mut self, new_span: Span) {
        match self {
            Self::Single(_, old_span) => *old_span = new_span,
            Self::Tuple(tuple) => tuple.span = new_span,
        }
    }

    pub fn span(&self) -> &Span {
        match self {
            Self::Single(_, span) => span,
            Self::Tuple(tuple) => &tuple.span,
        }
    }
}

impl fmt::Display for GroupLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Single(string, _) => write!(f, "{string}"),
            Self::Tuple(tuple) => write!(f, "{}", tuple.x), // Temporarily emit x coordinate only.
        }
    }
}

/// An affine group literal, e.g., `(42, 24)group`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupTuple {
    /// The left component of the type, e.g., `42` in the case above.
    pub x: GroupCoordinate,
    /// The right component of the type, e.g., `24` in the case above.
    pub y: GroupCoordinate,
    /// The span from `(` to `)`.
    pub span: Span,
}
