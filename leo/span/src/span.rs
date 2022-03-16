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

//! Defines the `Span` type used to track where code comes from.

use std::{fmt, sync::Arc, usize};

use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use tendril::StrTendril;

/// The span type which tracks where formatted errors originate from in a Leo file.
/// This is used in many spots throughout the rest of the Leo crates.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq)]
pub struct Span {
    /// The line number where the error started.
    pub line_start: usize,
    /// The line number where the error stopped.
    pub line_stop: usize,
    /// The column number where the error started.
    pub col_start: usize,
    /// The column number where the error stopped.
    pub col_stop: usize,
    /// The path to the Leo file containing the error.
    pub path: Arc<String>,
    #[serde(with = "crate::tendril_json")]
    /// The content of the line(s) that the span is found on.
    pub content: StrTendril,
}

impl Span {
    /// Generate a new span from:
    /// - Where the Leo line starts.
    /// - Where the Leo line stops.
    /// - Where the Leo column starts.
    /// - Where the Leo column stops.
    /// - The path to the Leo file.
    /// - The content of those specified bounds.
    pub fn new(
        line_start: usize,
        line_stop: usize,
        col_start: usize,
        col_stop: usize,
        path: Arc<String>,
        content: StrTendril,
    ) -> Self {
        Self {
            line_start,
            line_stop,
            col_start,
            col_stop,
            path,
            content,
        }
    }
}

impl Serialize for Span {
    /// Custom serialization for testing purposes.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Color", 3)?;
        state.serialize_field("line_start", &self.line_start)?;
        state.serialize_field("line_stop", &self.line_stop)?;
        state.serialize_field("col_start", &self.col_start)?;
        state.serialize_field("col_stop", &self.col_stop)?;
        // This is for testing purposes since the tests are run on a variety of OSes.
        if std::env::var("LEO_TESTFRAMEWORK")
            .unwrap_or_default()
            .trim()
            .to_owned()
            .is_empty()
        {
            state.serialize_field("path", &self.path)?;
        } else {
            state.serialize_field("path", "")?;
        }
        state.serialize_field("content", self.content.as_ref())?;
        state.end()
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line_start == self.line_stop {
            write!(f, "{}:{}-{}", self.line_start, self.col_start, self.col_stop)
        } else {
            write!(
                f,
                "{}:{}-{}:{}",
                self.line_start, self.col_start, self.line_stop, self.col_stop
            )
        }
    }
}

impl std::ops::Add for &Span {
    type Output = Span;

    fn add(self, other: &Span) -> Span {
        self.clone() + other.clone()
    }
}

impl std::ops::Add for Span {
    type Output = Self;

    #[allow(clippy::comparison_chain)]
    fn add(self, other: Self) -> Self {
        if self.line_start == other.line_stop {
            Span {
                line_start: self.line_start,
                line_stop: self.line_stop,
                col_start: self.col_start.min(other.col_start),
                col_stop: self.col_stop.max(other.col_stop),
                path: self.path,
                content: self.content,
            }
        } else {
            let mut new_content = vec![];
            let self_lines = self.content.lines().collect::<Vec<_>>();
            let other_lines = other.content.lines().collect::<Vec<_>>();
            for line in self.line_start.min(other.line_start)..self.line_stop.max(other.line_stop) + 1 {
                if line >= self.line_start && line <= self.line_stop {
                    new_content.push(
                        self_lines
                            .get(line - self.line_start)
                            .copied()
                            .unwrap_or_default()
                            .to_string(),
                    );
                } else if line >= other.line_start && line <= other.line_stop {
                    new_content.push(
                        other_lines
                            .get(line - other.line_start)
                            .copied()
                            .unwrap_or_default()
                            .to_string(),
                    );
                } else if new_content.last().map(|x| *x != "...").unwrap_or(true) {
                    new_content.push(format!("{:<1$}...", " ", other.col_start + 4));
                }
            }
            let new_content = new_content.join("\n").into();
            if self.line_start < other.line_stop {
                Span {
                    line_start: self.line_start,
                    line_stop: other.line_stop,
                    col_start: self.col_start,
                    col_stop: other.col_stop,
                    path: self.path,
                    content: new_content,
                }
            } else {
                Span {
                    line_start: other.line_start,
                    line_stop: self.line_stop,
                    col_start: other.col_start,
                    col_stop: self.col_stop,
                    path: self.path,
                    content: new_content,
                }
            }
        }
    }
}
