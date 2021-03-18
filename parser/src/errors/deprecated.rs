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

use leo_ast::{FormattedError, LeoError, Span};

#[derive(Debug, Error)]
pub enum DeprecatedError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl DeprecatedError {
    fn new_from_span(message: String, span: &Span) -> Self {
        DeprecatedError::Error(FormattedError::new_from_span(message, span))
    }
}

impl LeoError for DeprecatedError {
    fn get_path(&self) -> Option<&str> {
        match self {
            DeprecatedError::Error(error) => error.get_path(),
        }
    }

    fn set_path(&mut self, path: &str, contents: &[String]) {
        match self {
            DeprecatedError::Error(error) => error.set_path(path, contents),
        }
    }
}

impl DeprecatedError {
    pub fn let_mut_statement(span: &Span) -> Self {
        let message = "let mut = ... is deprecated. `let` keyword implies mutabality by default.".to_string();
        Self::new_from_span(message, span)
    }

    pub fn test_function(span: &Span) -> Self {
        let message = "\"test function...\" is deprecated. Did you mean @test annotation?".to_string();
        Self::new_from_span(message, span)
    }

    pub fn context_annotation(span: &Span) -> Self {
        let message = "\"@context(...)\" is deprecated. Did you mean @test annotation?".to_string();
        Self::new_from_span(message, span)
    }
}
