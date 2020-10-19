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

use crate::VariableTableError;
use leo_static_check::{Type, TypeError};
use leo_typed::{Error as FormattedError, Identifier, Span};

use std::path::PathBuf;

/// Errors encountered when resolving an expression.
#[derive(Debug, Error)]
pub enum ExpressionError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    TypeError(#[from] TypeError),

    #[error("{}", _0)]
    VariableTableError(#[from] VariableTableError),
}

impl ExpressionError {
    ///
    /// Set the filepath for the error stacktrace.
    ///
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            ExpressionError::Error(error) => error.set_path(path),
            ExpressionError::TypeError(error) => error.set_path(path),
            ExpressionError::VariableTableError(error) => error.set_path(path),
        }
    }

    ///
    /// Return a new formatted error with a given message and span information.
    ///
    fn new_from_span(message: String, span: Span) -> Self {
        ExpressionError::Error(FormattedError::new_from_span(message, span))
    }

    ///
    /// Attempted to access a circuit member that is static using double colon syntax.
    ///
    pub fn invalid_member_access(member: String, span: Span) -> Self {
        let message = format!("Circuit member `{}` must be accessed using `::` syntax.", member);

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to access a circuit member that is static using dot syntax.
    ///
    pub fn invalid_static_member_access(member: String, span: Span) -> Self {
        let message = format!("Static member `{}` must be accessed using `.` syntax.", member);

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to access a tuple index that does not exist.
    ///
    pub fn invalid_index_tuple(index: usize, max: usize, span: Span) -> Self {
        let message = format!("Attempted to access index {} of tuple with length {}.", index, max);

        Self::new_from_span(message, span)
    }

    ///
    /// Found an array with an unexpected length.
    ///
    pub fn invalid_length_array(expected: usize, actual: usize, span: Span) -> Self {
        let message = format!(
            "Expected array with length {}, found array with length {}.",
            expected, actual
        );

        Self::new_from_span(message, span)
    }

    ///
    /// Found a circuit with an incorrect number of members
    ///
    pub fn invalid_length_circuit_members(expected: usize, actual: usize, span: Span) -> Self {
        let message = format!(
            "Expected circuit with {} members, found circuit with {} members.",
            expected, actual
        );

        Self::new_from_span(message, span)
    }

    ///
    /// Found a circuit with an incorrect number of members.
    ///
    pub fn invalid_length_function_inputs(expected: usize, actual: usize, span: Span) -> Self {
        let message = format!(
            "Function expected {} inputs, found function with {} members.",
            expected, actual
        );

        Self::new_from_span(message, span)
    }

    ///
    /// Found a tuple with an unexpected length.
    ///
    pub fn invalid_length_tuple(expected: usize, actual: usize, span: Span) -> Self {
        let message = format!(
            "Expected tuple with length {}, found tuple with length {}.",
            expected, actual
        );

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to assign a tuple expression to a variable with a different explicit type.
    ///
    pub fn invalid_type_tuple(actual: &Type, span: &Span) -> Self {
        let message = format!("Expected tuple type, found type {}", actual);

        Self::new_from_span(message, span.clone())
    }

    ///
    /// Attempted to lookup an unknown variable name.
    ///
    pub fn undefined_identifier(identifier: Identifier) -> Self {
        let message = format!("Cannot find variable `{}` in this scope.", identifier.name);

        Self::new_from_span(message, identifier.span)
    }

    ///
    /// Attempted to lookup an unknown circuit name.
    ///
    pub fn undefined_circuit(identifier: Identifier) -> Self {
        let message = format!("Cannot find circuit `{}` in this scope.", identifier.name);

        Self::new_from_span(message, identifier.span)
    }

    ///
    /// Attempted to lookup an unknown circuit variable name.
    ///
    pub fn undefined_circuit_variable(identifier: Identifier) -> Self {
        let message = format!("Circuit has no member variable named `{}`.", identifier.name);

        Self::new_from_span(message, identifier.span)
    }

    ///
    /// Attempted to lookup an unknown circuit function name.
    ///
    pub fn undefined_circuit_function(identifier: Identifier, span: Span) -> Self {
        let message = format!("Circuit has no member function named `{}`.", identifier.name);

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to lookup an unknown circuit function static name.
    ///
    pub fn undefined_circuit_function_static(identifier: Identifier, span: Span) -> Self {
        let message = format!("Circuit has no static member function named `{}`.", identifier.name);

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to lookup an unknown circuit name.
    ///
    pub fn undefined_function(identifier: Identifier) -> Self {
        let message = format!("Cannot find function `{}` in this scope.", identifier.name);

        Self::new_from_span(message, identifier.span)
    }
}
