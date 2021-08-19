// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
o// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_ast::{FormattedError, LeoError, Span};

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl LeoError for TokenError {}

impl TokenError {
    fn new_from_span(message: String, span: &Span) -> Self {
        TokenError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn unexpected_token(token: &str, span: &Span) -> Self {
        TokenError::new_from_span(format!("unexpected token: '{}'", token), span)
    }

    pub fn invalid_address_lit(token: &str, span: &Span) -> Self {
        TokenError::new_from_span(format!("invalid address literal: '{}'", token), span)
    }
}
