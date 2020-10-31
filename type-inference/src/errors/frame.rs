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

use crate::{ScopeError, TypeAssertionError};
use leo_ast::{Error as FormattedError, Expression, Identifier, Span};
use leo_symbol_table::{Type, TypeError};

use std::path::Path;

/// Errors encountered when tracking variable names in a program.
#[derive(Debug, Error)]
pub enum FrameError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    ScopeError(#[from] ScopeError),

    #[error("{}", _0)]
    TypeAssertionError(#[from] TypeAssertionError),

    #[error("{}", _0)]
    TypeError(#[from] TypeError),
}

impl FrameError {
    ///
    /// Set the filepath for the error stacktrace
    ///
    pub fn set_path(&mut self, path: &Path) {
        match self {
            FrameError::Error(error) => error.set_path(path),
            FrameError::ScopeError(error) => error.set_path(path),
            FrameError::TypeAssertionError(error) => error.set_path(path),
            FrameError::TypeError(error) => error.set_path(path),
        }
    }

    ///
    /// Return a new formatted error with a given message and span information
    ///
    fn new_from_span(message: String, span: &Span) -> Self {
        FrameError::Error(FormattedError::new_from_span(message, span.to_owned()))
    }

    ///
    /// Attempted to access the index of a non-array type.
    ///
    pub fn array_access(actual: &Type, span: &Span) -> Self {
        let message = format!("Cannot access the index of non-array type `{}`.", actual);

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to access the `Self` type outside of a circuit context.
    ///
    pub fn circuit_self(span: &Span) -> Self {
        let message = "The `Self` keyword is only valid inside a circuit context.".to_string();

        Self::new_from_span(message, span)
    }

    ///
    /// Two variables have been defined with the same name.
    ///
    pub fn duplicate_variable(name: &str, span: &Span) -> Self {
        let message = format!("Duplicate variable definition found for `{}`", name);

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to create an empty array in a Leo program.
    ///
    /// Arrays in Leo are not resizeable so defining empty arrays are effectively dead code.
    ///
    pub fn empty_array(span: &Span) -> Self {
        let message = "Cannot create an empty array in a Leo program.".to_string();

        Self::new_from_span(message, span)
    }

    ///
    /// Expected a circuit name but found a different type.
    ///
    pub fn invalid_circuit(type_: Type, span: &Span) -> Self {
        let message = format!("Expected a circuit type. Found type `{}`.", type_);

        Self::new_from_span(message, span)
    }

    ///
    /// Expected a function name but found a different expression.
    ///
    pub fn invalid_function(expression: &Expression, span: &Span) -> Self {
        let message = format!("Expected a function name. Found expression `{}`.", expression);

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to call non-static member using `::`.
    ///
    pub fn invalid_member_access(identifier: &Identifier) -> Self {
        let message = format!("non-static member `{}` must be accessed using `.` syntax.", identifier);

        Self::new_from_span(message, &identifier.span)
    }

    ///
    /// Attempted to use the spread operator on a non-array type.
    ///
    pub fn invalid_spread(actual: Type, span: &Span) -> Self {
        let message = format!(
            "The spread operator `...` can only be applied to array types. Found type `{}`.",
            actual
        );

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to call static member using `.`.
    ///
    pub fn invalid_static_access(identifier: &Identifier) -> Self {
        let message = format!("Static member `{}` must be accessed using `::` syntax.", identifier);

        Self::new_from_span(message, &identifier.span)
    }

    ///
    /// Attempted to create a circuit with the incorrect number of member variables.
    ///
    pub fn num_circuit_variables(expected: usize, actual: usize, span: &Span) -> Self {
        let message = format!("Circuit expected {} variables, found {} variables.", expected, actual);

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to call a function with the incorrect number of inputs.
    ///
    pub fn num_inputs(expected: usize, actual: usize, span: &Span) -> Self {
        let message = format!(
            "Function expected {} input variables, found {} inputs.",
            expected, actual
        );

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to access the index of a non-tuple type.
    ///
    pub fn tuple_access(actual: &Type, span: &Span) -> Self {
        let message = format!("Cannot access the index of non-tuple type `{}`.", actual);

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to call a circuit type that is not defined in the current context.
    ///
    pub fn undefined_circuit(identifier: &Identifier) -> Self {
        let message = format!("The circuit `{}` is not defined.", identifier);

        Self::new_from_span(message, &identifier.span)
    }

    ///
    /// Attempted to call a circuit function that is not defined in the current context.
    ///
    pub fn undefined_circuit_function(identifier: &Identifier) -> Self {
        let message = format!("The circuit function `{}` is not defined.", identifier);

        Self::new_from_span(message, &identifier.span)
    }

    ///
    /// Attempted to call a function that is not defined in the current context.
    ///
    pub fn undefined_function(identifier: &Identifier) -> Self {
        let message = format!("The function `{}` is not defined.", identifier);

        Self::new_from_span(message, &identifier.span)
    }

    ///
    /// Attempted to call a variable that is not defined in the current context.
    ///
    pub fn undefined_variable(identifier: &Identifier) -> Self {
        let message = format!("The variable `{}` is not defined.", identifier);

        Self::new_from_span(message, &identifier.span)
    }

    ///
    /// Attempted to assign a tuple of variables to a single value.
    ///
    pub fn not_enough_values(span: &Span) -> Self {
        let message = "Expected a tuple type for multiple defined variables".to_string();

        Self::new_from_span(message, span)
    }

    ///
    /// Attempted to assign a tuple with a different number of variables than values.
    ///
    pub fn invalid_number_of_values(expected: usize, actual: usize, span: &Span) -> Self {
        let message = format!(
            "Incorrect number of defined variables. Expected `{}`, found `{}`.",
            expected, actual
        );

        Self::new_from_span(message, span)
    }
}
