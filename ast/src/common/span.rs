// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use pest::Span as GrammarSpan;
use serde::Deserialize;
use serde::Serialize;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Span {
    /// text of input string
    pub text: String,
    /// program line
    pub line: usize,
    /// start column
    pub start: usize,
    /// end column
    pub end: usize,
}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.start == other.start && self.end == other.end
    }
}

impl Eq for Span {}

impl Hash for Span {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.line.hash(state);
        self.start.hash(state);
        self.end.hash(state);
    }
}

impl Span {
    pub fn from_internal_string(value: &str) -> Span {
        Span {
            text: value.to_string(),
            line: 0,
            start: 0,
            end: 0,
        }
    }
}

impl<'ast> From<GrammarSpan<'ast>> for Span {
    fn from(span: GrammarSpan<'ast>) -> Self {
        let mut text = " ".to_string();
        let line_col = span.start_pos().line_col();
        let end = span.end_pos().line_col().1;

        text.push_str(span.start_pos().line_of().trim_end());

        Self {
            text,
            line: line_col.0,
            start: line_col.1,
            end,
        }
    }
}
