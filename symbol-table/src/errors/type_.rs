// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::Type;
use leo_core_ast::{Error as FormattedError, Identifier, Span};

use std::path::Path;

/// Errors encountered when resolving types.
#[derive(Debug, Error)]
pub enum TypeError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl TypeError {
    ///
    /// Set the filepath for the error stacktrace.
    ///
    pub fn set_path(&mut self, path: &Path) {
        match self {
            TypeError::Error(error) => error.set_path(path),
        }
    }

    ///
    /// Return a new formatted error with a given message and span information.
    ///
    fn new_from_span(message: String, span: Span) -> Self {
        TypeError::Error(FormattedError::new_from_span(message, span))
    }

    ///
    /// Expected an array type from the given expression.
    ///
    pub fn invalid_array(actual: &Type, span: Span) -> Self {
        let message = format!("Expected array type, found type `{}`.", actual);

        Self::new_from_span(message, span)
    }

    ///
    /// Expected a circuit type from the given expression.
    ///
    pub fn invalid_circuit(actual: &Type, span: Span) -> Self {
        let message = format!("Expected circuit type, found type `{}`.", actual);

        Self::new_from_span(message, span)
    }

    ///
    /// Expected a function type from the given expression.
    ///
    pub fn invalid_function(actual: &Type, span: Span) -> Self {
        let message = format!("Expected function type, found type `{}`.", actual);

        Self::new_from_span(message, span)
    }

    ///
    /// Expected an integer type from the given expression.
    ///
    pub fn invalid_integer(actual: &Type, span: Span) -> Self {
        let message = format!("Expected integer type, found type `{}`.", actual);

        Self::new_from_span(message, span)
    }

    ///
    /// Expected a tuple type from the given expression.
    ///
    pub fn invalid_tuple(actual: &Type, span: Span) -> Self {
        let message = format!("Expected tuple type, found type `{}`.", actual);

        Self::new_from_span(message, span)
    }

    ///
    /// The value of the expression does not match the given explicit type.
    ///
    pub fn mismatched_types(expected: &Type, actual: &Type, span: Span) -> Self {
        let message = format!("Expected type `{}`, found type `{}`.", expected, actual);

        Self::new_from_span(message, span)
    }

    ///
    /// The `Self` keyword was used outside of a circuit.
    ///
    pub fn self_not_available(span: Span) -> Self {
        let message = format!("Type `Self` is only available in circuit definitions and circuit functions.");

        Self::new_from_span(message, span)
    }

    ///
    /// Found an unknown circuit name.
    ///
    pub fn undefined_circuit(identifier: Identifier) -> Self {
        let message = format!(
            "Type circuit `{}` must be defined before it is used in an expression.",
            identifier.name
        );

        Self::new_from_span(message, identifier.span)
    }

    ///
    /// Found an unknown circuit member name.
    ///
    pub fn undefined_circuit_member(identifier: Identifier) -> Self {
        let message = format!("Circuit has no member `{}`.", identifier.name);

        Self::new_from_span(message, identifier.span)
    }

    ///
    /// Found an unknown function name.
    ///
    pub fn undefined_function(identifier: Identifier) -> Self {
        let message = format!(
            "Type function `{}` must be defined before it is used in an expression.",
            identifier.name
        );

        Self::new_from_span(message, identifier.span)
    }
}
