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

use crate::{Expression, Node};
use leo_span::Span;

use std::fmt;

use serde::{Deserialize, Serialize};

/// An access to a certain range of elements in an `array`.
///
/// Examples include `array[0..3]`, `array[3..]`, `array[..3]`, and `array[..]`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArrayRangeAccess {
    /// The array to extract a range of elements from.
    pub array: Box<Expression>,
    /// The lower bound of the index-range, or the start of the array when `None`.
    pub left: Option<Box<Expression>>,
    /// The higher bound of the index-range, or the end of the array when `None`.
    pub right: Option<Box<Expression>>,
    /// A span for the entire expression `array[<range>]`.
    pub span: Span,
}

impl fmt::Display for ArrayRangeAccess {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}[{}..{}]",
            self.array,
            self.left.as_ref().map(|e| e.to_string()).unwrap_or_default(),
            self.right.as_ref().map(|e| e.to_string()).unwrap_or_default()
        )
    }
}

impl Node for ArrayRangeAccess {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
