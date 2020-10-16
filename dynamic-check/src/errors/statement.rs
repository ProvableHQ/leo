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

use crate::{ExpressionError, VariableTableError};
use leo_static_check::{Type, TypeError};
use leo_typed::{Error as FormattedError, Expression as UnresolvedExpression, Identifier, Span};

use std::path::PathBuf;

///
/// Errors encountered when resolving a statement
///
#[derive(Debug, Error)]
pub enum StatementError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    ExpressionError(#[from] ExpressionError),

    #[error("{}", _0)]
    TypeError(#[from] TypeError),

    #[error("{}", _0)]
    VariableTableError(#[from] VariableTableError),
}

impl StatementError {
    ///
    /// Set the filepath for the error stacktrace.
    ///
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            StatementError::Error(error) => error.set_path(path),
            StatementError::ExpressionError(error) => error.set_path(path),
            StatementError::TypeError(error) => error.set_path(path),
            StatementError::VariableTableError(error) => error.set_path(path),
        }
    }

    ///
    /// Return a new formatted error with a given message and span information.
    ///
    fn new_from_span(message: String, span: Span) -> Self {
        StatementError::Error(FormattedError::new_from_span(message, span))
    }

    ///
    /// Attempted to define a variable name twice.
    ///
    pub fn duplicate_variable(name: String, span: Span) -> Self {
        let message = format!("Duplicate variable definition found for `{}`", name);

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to assign to an immutable variable.
    ///
    pub fn immutable_assign(name: String, span: Span) -> Self {
        let message = format!("Cannot assign to immutable variable `{}`.", name);

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to assign to a non-variable type.
    ///
    pub fn invalid_assign(type_: &Type, span: Span) -> Self {
        let message = format!("Cannot assign to type `{}` in a statement.", type_);

        Self::new_from_span(message, span)
    }

    ///
    /// Provided a different number of explicit types than variables being defined in a tuple
    ///
    pub fn multiple_variable_types(expected: usize, actual: usize, span: Span) -> Self {
        let message = format!(
            "Expected {} explicit types when defining variables, found {}",
            expected, actual
        );

        Self::new_from_span(message, span)
    }

    ///
    /// Provided a different number of expression values than variables being defined in a tuple
    ///
    pub fn multiple_variable_expressions(expected: usize, actual: usize, span: &Span) -> Self {
        let message = format!(
            "Expected {} values when defining variables, found {} values",
            expected, actual
        );

        Self::new_from_span(message, span.clone())
    }

    ///
    /// Attempted to assign multiple variables to a single expression value.
    ///
    pub fn invalid_tuple(expected: usize, expression: UnresolvedExpression, span: Span) -> Self {
        let message = format!(
            "Expected {} values when defining variables, found single value `{}`",
            expected, expression
        );

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to assign to an unknown variable.
    ///
    pub fn undefined_variable(name: String, span: Span) -> Self {
        let message = format!("Attempted to assign to unknown variable `{}`.", name);

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to assign to an undefined circuit.
    ///
    pub fn undefined_circuit(identifier: Identifier) -> Self {
        let message = format!("Attempted to assign to unknown circuit `{}`.", identifier.name);

        Self::new_from_span(message, identifier.span)
    }
}
