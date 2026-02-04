// Copyright (C) 2019-2026 Provable Inc.
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

//! Defines the `Span` type used to track where code comes from.

use crate::symbol::with_session_globals;

use serde::{Deserialize, Serialize};
use std::fmt;

/// The span type which tracks where formatted errors originate from in a Leo file.
/// This is used in many spots throughout the rest of the Leo crates.
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Span {
    /// The start (low) position of the span, inclusive.
    pub lo: u32,
    /// The end (high) position of the span, exclusive.
    /// The length of the span is `hi - lo`.
    pub hi: u32,
}

impl Span {
    /// Generate a new span from the `start`ing and `end`ing positions.
    pub fn new(start: u32, end: u32) -> Self {
        Self { lo: start, hi: end }
    }

    /// Generates a dummy span with all defaults.
    /// Should only be used in temporary situations.
    pub const fn dummy() -> Self {
        Self { lo: 0, hi: 0 }
    }

    /// Is the span a dummy?
    pub fn is_dummy(&self) -> bool {
        self == &Self::dummy()
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        with_session_globals(|s| {
            let source_file = s.source_map.find_source_file(self.lo).unwrap();
            let (start_line, start_col) = source_file.line_col(self.lo);
            let (end_line, end_col) = source_file.line_col(self.hi);
            if start_line == end_line {
                write!(f, "{}:{}-{}", start_line + 1, start_col + 1, end_col + 1)
            } else {
                write!(f, "{}:{}-{}:{}", start_line + 1, start_col + 1, end_line + 1, end_col + 1)
            }
        })
    }
}

impl std::ops::Add for &Span {
    type Output = Span;

    /// Add two spans (by reference) together.
    fn add(self, other: &Span) -> Span {
        *self + *other
    }
}

impl std::ops::Add for Span {
    type Output = Self;

    /// Add two spans together.
    /// The resulting span is the smallest span that includes both.
    fn add(self, other: Self) -> Self {
        let lo = self.lo.min(other.lo);
        let hi = self.hi.max(other.hi);
        Self::new(lo, hi)
    }
}
