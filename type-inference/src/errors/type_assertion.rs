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

use crate::TypeMembership;

use leo_ast::{Error as FormattedError, Span};
use leo_symbol_table::Type;

use std::path::Path;

/// Errors encountered when attempting to solve a type assertion.
#[derive(Debug, Error)]
pub enum TypeAssertionError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl TypeAssertionError {
    ///
    /// Set the filepath for the error stacktrace.
    ///
    pub fn set_path(&mut self, path: &Path) {
        match self {
            TypeAssertionError::Error(error) => error.set_path(path),
        }
    }

    ///
    /// Returns a new formatted error with a given message and span information.
    ///
    fn new_from_span(message: String, span: &Span) -> Self {
        TypeAssertionError::Error(FormattedError::new_from_span(message, span.to_owned()))
    }

    ///
    /// Found mismatched types during program parsing.
    ///
    pub fn equality_failed(left: &Type, right: &Type, span: &Span) -> Self {
        let message = format!("Mismatched types. Expected type `{}`, found type `{}`.", left, right);

        Self::new_from_span(message, span)
    }

    ///
    /// Given type is not a member of the set of expected types.
    ///
    pub fn membership_failed(given: &Type, set: &[Type], span: &Span) -> Self {
        let message = format!(
            "Mismatched types. Given type `{}` is not in the expected type set `{:?}`.",
            given, set
        );

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to generate pairs from a membership assertion.
    ///
    pub fn membership_pairs(membership: &TypeMembership) -> Self {
        let message = "Cannot generate a type variable -> type pair for the given type membership".to_string();

        Self::new_from_span(message, membership.span())
    }
}
