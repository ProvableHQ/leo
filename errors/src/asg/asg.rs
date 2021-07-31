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

use crate::{ErrorCode, FormattedError, LeoErrorCode, Span};

#[derive(Debug, Error)]
pub enum AsgError {
    #[error(transparent)]
    FormattedError(#[from] FormattedError),
}

impl LeoErrorCode for AsgError {}

impl ErrorCode for AsgError {
    #[inline(always)]
    fn exit_code_mask() -> u32 {
        2000
    }

    #[inline(always)]
    fn error_type() -> String {
        "G".to_string()
    }

    fn new_from_span(message: String, help: Option<String>, exit_code: u32, span: &Span) -> Self {
        Self::FormattedError(FormattedError::new_from_span(
            message,
            help,
            exit_code ^ Self::exit_code_mask(),
            Self::code_identifier(),
            Self::error_type(),
            span,
        ))
    }
}

impl AsgError {
    pub fn unresolved_circuit(name: &str, span: &Span) -> Self {
        Self::new_from_span(format!("failed to resolve circuit: '{}'", name), None, 0, span)
    }

    pub fn unresolved_import(name: &str, span: &Span) -> Self {
        Self::new_from_span(format!("failed to resolve import: '{}'", name), None, 1, span)
    }

    pub fn unresolved_circuit_member(circuit_name: &str, name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "illegal reference to non-existant member '{}' of circuit '{}'",
                name, circuit_name
            ),
            None,
            2,
            span,
        )
    }

    pub fn missing_circuit_member(circuit_name: &str, name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "missing circuit member '{}' for initialization of circuit '{}'",
                name, circuit_name
            ),
            None,
            3,
            span,
        )
    }

    pub fn overridden_circuit_member(circuit_name: &str, name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "cannot declare circuit member '{}' more than once for initialization of circuit '{}'",
                name, circuit_name
            ),
            None,
            4,
            span,
        )
    }

    pub fn redefined_circuit_member(circuit_name: &str, name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "cannot declare circuit member '{}' multiple times in circuit '{}'",
                name, circuit_name
            ),
            None,
            5,
            span,
        )
    }

    pub fn extra_circuit_member(circuit_name: &str, name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "extra circuit member '{}' for initialization of circuit '{}' is not allowed",
                name, circuit_name
            ),
            None,
            6,
            span,
        )
    }

    pub fn illegal_function_assign(name: &str, span: &Span) -> Self {
        Self::new_from_span(format!("attempt to assign to function '{}'", name), None, 7, span)
    }

    pub fn circuit_variable_call(circuit_name: &str, name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!("cannot call variable member '{}' of circuit '{}'", name, circuit_name),
            None,
            8,
            span,
        )
    }

    pub fn circuit_static_call_invalid(circuit_name: &str, name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "cannot call static function '{}' of circuit '{}' from target",
                name, circuit_name
            ),
            None,
            9,
            span,
        )
    }

    pub fn circuit_member_mut_call_invalid(circuit_name: &str, name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "cannot call mutable member function '{}' of circuit '{}' from immutable context",
                name, circuit_name
            ),
            None,
            10,
            span,
        )
    }

    pub fn circuit_member_call_invalid(circuit_name: &str, name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "cannot call member function '{}' of circuit '{}' from static context",
                name, circuit_name
            ),
            None,
            11,
            span,
        )
    }

    pub fn circuit_function_ref(circuit_name: &str, name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "cannot reference function member '{}' of circuit '{}' as value",
                name, circuit_name
            ),
            None,
            12,
            span,
        )
    }

    pub fn index_into_non_array(name: &str, span: &Span) -> Self {
        Self::new_from_span(format!("failed to index into non-array '{}'", name), None, 13, span)
    }

    pub fn invalid_assign_index(name: &str, num: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!("failed to index array with invalid integer '{}'[{}]", name, num),
            None,
            14,
            span,
        )
    }

    pub fn invalid_backwards_assignment(name: &str, left: usize, right: usize, span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "failed to index array range for assignment with left > right '{}'[{}..{}]",
                name, left, right
            ),
            None,
            15,
            span,
        )
    }

    pub fn invalid_const_assign(name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "failed to create const variable(s) '{}' with non constant values.",
                name
            ),
            None,
            16,
            span,
        )
    }

    pub fn duplicate_function_definition(name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!("a function named \"{}\" already exists in this scope", name),
            None,
            17,
            span,
        )
    }

    pub fn duplicate_variable_definition(name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!("a variable named \"{}\" already exists in this scope", name),
            None,
            18,
            span,
        )
    }

    pub fn index_into_non_tuple(name: &str, span: &Span) -> Self {
        Self::new_from_span(format!("failed to index into non-tuple '{}'", name), None, 19, span)
    }

    pub fn tuple_index_out_of_bounds(index: usize, span: &Span) -> Self {
        Self::new_from_span(format!("tuple index out of bounds: '{}'", index), None, 20, span)
    }

    pub fn array_index_out_of_bounds(index: usize, span: &Span) -> Self {
        Self::new_from_span(format!("array index out of bounds: '{}'", index), None, 21, span)
    }

    pub fn ternary_different_types(left: &str, right: &str, span: &Span) -> Self {
        let message = format!("ternary sides had different types: left {}, right {}", left, right);

        Self::new_from_span(message, None, 22, span)
    }

    pub fn unknown_array_size(span: &Span) -> Self {
        Self::new_from_span(
            "array size cannot be inferred, add explicit types".to_string(),
            None,
            23,
            span,
        )
    }

    pub fn unexpected_call_argument_count(expected: usize, got: usize, span: &Span) -> Self {
        Self::new_from_span(
            format!("function call expected {} arguments, got {}", expected, got),
            None,
            24,
            span,
        )
    }

    pub fn unresolved_function(name: &str, span: &Span) -> Self {
        Self::new_from_span(format!("failed to resolve function: '{}'", name), None, 25, span)
    }

    pub fn unresolved_type(name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!("failed to resolve type for variable definition '{}'", name),
            None,
            26,
            span,
        )
    }

    pub fn unexpected_type(expected: &str, received: Option<&str>, span: &Span) -> Self {
        Self::new_from_span(
            format!(
                "unexpected type, expected: '{}', received: '{}'",
                expected,
                received.unwrap_or("unknown")
            ),
            None,
            27,
            span,
        )
    }

    pub fn unexpected_nonconst(span: &Span) -> Self {
        Self::new_from_span("expected const, found non-const value".to_string(), None, 28, span)
    }

    pub fn unresolved_reference(name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!("failed to resolve variable reference '{}'", name),
            None,
            29,
            span,
        )
    }

    pub fn invalid_boolean(value: &str, span: &Span) -> Self {
        Self::new_from_span(format!("failed to parse boolean value '{}'", value), None, 30, span)
    }

    pub fn invalid_char(value: &str, span: &Span) -> Self {
        Self::new_from_span(format!("failed to parse char value '{}'", value), None, 31, span)
    }

    pub fn invalid_int(value: &str, span: &Span) -> Self {
        Self::new_from_span(format!("failed to parse int value '{}'", value), None, 32, span)
    }

    pub fn unsigned_negation(span: &Span) -> Self {
        Self::new_from_span("cannot negate unsigned integer".to_string(), None, 33, span)
    }

    pub fn immutable_assignment(name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!("illegal assignment to immutable variable '{}'", name),
            None,
            34,
            span,
        )
    }

    pub fn function_missing_return(name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!("function '{}' missing return for all paths", name),
            None,
            35,
            span,
        )
    }

    pub fn function_return_validation(name: &str, description: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!("function '{}' failed to validate return path: '{}'", name, description),
            None,
            36,
            span,
        )
    }

    pub fn input_ref_needs_type(category: &str, name: &str, span: &Span) -> Self {
        Self::new_from_span(
            format!("could not infer type for input in '{}': '{}'", category, name),
            None,
            37,
            span,
        )
    }

    pub fn invalid_self_in_global(span: &Span) -> Self {
        Self::new_from_span(
            "cannot have `mut self` or `self` arguments in global functions".to_string(),
            None,
            38,
            span,
        )
    }

    pub fn call_test_function(span: &Span) -> Self {
        Self::new_from_span("cannot call test function".to_string(), None, 39, span)
    }

    pub fn circuit_test_function(span: &Span) -> Self {
        Self::new_from_span(
            "cannot have test function as member of circuit".to_string(),
            None,
            40,
            span,
        )
    }

    pub fn parse_index_error(span: &Span) -> Self {
        Self::new_from_span("failed to parse index".to_string(), None, 41, span)
    }

    pub fn parse_dimension_error(span: &Span) -> Self {
        Self::new_from_span("failed to parse dimension".to_string(), None, 42, span)
    }

    pub fn reference_self_outside_circuit(span: &Span) -> Self {
        Self::new_from_span(
            "referenced self outside of circuit function".to_string(),
            None,
            43,
            span,
        )
    }

    pub fn illegal_ast_structure(details: &str, span: &Span) -> Self {
        Self::new_from_span(format!("illegal ast structure: {}", details), None, 44, span)
    }

    pub fn illegal_input_variable_reference(details: &str, span: &Span) -> Self {
        Self::new_from_span(format!("illegal ast structure: {}", details), None, 45, span)
    }
}
