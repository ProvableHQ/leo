// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use std::fmt;

use leo_span::Span;

/// Represents the interpreter halting, which should not be considered an
/// actual runtime error.
#[derive(Clone, Debug, Error)]
pub struct InterpreterHalt {
    /// Optional Span where the halt occurred.
    span: Option<Span>,

    /// User visible message.
    message: String,
}

impl InterpreterHalt {
    pub fn new(message: String) -> Self {
        InterpreterHalt { span: None, message }
    }

    pub fn new_spanned(message: String, span: Span) -> Self {
        InterpreterHalt { span: Some(span), message }
    }

    pub fn span(&self) -> Option<Span> {
        self.span
    }
}

impl fmt::Display for InterpreterHalt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
