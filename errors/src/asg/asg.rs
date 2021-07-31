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

use crate::create_errors;

create_errors!(
    AsgError,
    exit_code_mask: 0u32,
    error_code_prefix: "G",

    unresolved_circuit {
        args: (name: &str),
        msg: format!("failed to resolve circuit: '{}'", name),
        help: None,
    }

    unresolved_import {
        args: (name: &str),
        msg: format!("failed to resolve import: '{}'", name),
        help: None,
    }

    unresolved_circuit_member {
        args: (circuit_name: &str, name: &str),
        msg: format!(
            "illegal reference to non-existant member '{}' of circuit '{}'",
            name, circuit_name
        ),
        help: None,
    }

    missing_circuit_member {
        args: (circuit_name: &str, name: &str),
        msg: format!(
            "missing circuit member '{}' for initialization of circuit '{}'",
            name, circuit_name
        ),
        help: None,
    }

    overridden_circuit_member {
        args: (circuit_name: &str, name: &str),
        msg: format!(
            "cannot declare circuit member '{}' more than once for initialization of circuit '{}'",
            name, circuit_name
        ),
        help: None,
    }

    redefined_circuit_member {
        args: (circuit_name: &str, name: &str),
        msg: format!(
            "cannot declare circuit member '{}' multiple times in circuit '{}'",
            name, circuit_name
        ),
        help: None,
    }

    extra_circuit_member {
        args: (circuit_name: &str, name: &str),
        msg: format!(
            "extra circuit member '{}' for initialization of circuit '{}' is not allowed",
            name, circuit_name
        ),
        help: None,
    }

    illegal_function_assign {
        args: (name: &str),
        msg: format!("attempt to assign to function '{}'", name),
        help: None,
    }

    circuit_variable_call {
        args: (circuit_name: &str, name: &str),
        msg: format!("cannot call variable member '{}' of circuit '{}'", name, circuit_name),
        help: None,
    }

    circuit_static_call_invalid {
        args: (circuit_name: &str, name: &str),
        msg: format!(
            "cannot call static function '{}' of circuit '{}' from target",
            name, circuit_name
        ),
        help: None,
    }

    circuit_member_mut_call_invalid {
        args: (circuit_name: &str, name: &str),
        msg: format!(
            "cannot call mutable member function '{}' of circuit '{}' from immutable context",
            name, circuit_name
        ),
        help: None,
    }

    circuit_member_call_invalid {
        args: (circuit_name: &str, name: &str),
        msg: format!(
            "cannot call member function '{}' of circuit '{}' from static context",
            name, circuit_name
        ),
        help: None,
    }

    circuit_function_ref {
        args: (circuit_name: &str, name: &str),
        msg: format!(
            "cannot reference function member '{}' of circuit '{}' as value",
            name, circuit_name
        ),
        help: None,
    }

    index_into_non_array {
        args: (name: &str),
        msg: format!("failed to index into non-array '{}'", name),
        help: None,
    }
    
    invalid_assign_index {
        args: (name: &str, num: &str),
        msg: format!("failed to index array with invalid integer '{}'[{}]", name, num),
        help: None,
    }
    
    invalid_backwards_assignment {
        args: (name: &str, left: usize, right: usize),
        msg: format!(
            "failed to index array range for assignment with left > right '{}'[{}..{}]",
            name, left, right
        ),
        help: None,
    }

    invalid_const_assign {
        args: (name: &str),
        msg: format!(
            "failed to create const variable(s) '{}' with non constant values.",
            name
        ),
        help: None,
    }

    duplicate_function_definition {
        args: (name: &str),
        msg: format!("a function named \"{}\" already exists in this scope", name),
        help: None,
    }

    duplicate_variable_definition {
        args: (name: &str),
        msg: format!("a variable named \"{}\" already exists in this scope", name),
        help: None,
    }

    index_into_non_tuple {
        args: (name: &str),
        msg: format!("failed to index into non-tuple '{}'", name),
        help: None,
    }

    tuple_index_out_of_bounds {
        args: (index: usize),
        msg: format!("tuple index out of bounds: '{}'", index),
        help: None,
    }

    array_index_out_of_bounds {
        args: (index: usize),
        msg: format!("array index out of bounds: '{}'", index),
        help: None,
    }

    ternary_different_types {
        args: (left: &str, right: &str),
        msg: format!("ternary sides had different types: left {}, right {}", left, right),
        help: None,
    }

    unknown_array_size {
        args: (),
        msg: "array size cannot be inferred, add explicit types",
        help: None,
    }

    unexpected_call_argument_count {
        args: (expected: usize, got: usize),
        msg: format!("function call expected {} arguments, got {}", expected, got),
        help: None,
    }

    unresolved_function {
        args: (name: &str),
        msg: format!("failed to resolve function: '{}'", name),
        help: None,
    }

    unresolved_type {
        args: (name: &str),
        msg: format!("failed to resolve type for variable definition '{}'", name),
        help: None,
    }

    unexpected_type {
        args: (expected: &str, received: Option<&str>),
        msg: format!(
            "unexpected type, expected: '{}', received: '{}'",
            expected,
            received.unwrap_or("unknown")
        ),
        help: None,
    }

    unexpected_nonconst {
        args: (),
        msg: "expected const, found non-const value",
        help: None,
    }

    unresolved_reference {
        args: (name: &str),
        msg: format!("failed to resolve variable reference '{}'", name),
        help: None,
    }

    invalid_boolean {
        args: (value: &str),
        msg: format!("failed to parse boolean value '{}'", value),
        help: None,
    }

    invalid_char {
        args: (value: &str),
        msg: format!("failed to parse char value '{}'", value),
        help: None,
    }

    invalid_int {
        args: (value: &str),
        msg: format!("failed to parse int value '{}'", value),
        help: None,
    }

    unsigned_negation {
        args: (),
        msg: "cannot negate unsigned integer",
        help: None,
    }

    immutable_assignment {
        args: (name: &str),
        msg: format!("illegal assignment to immutable variable '{}'", name),
        help: None,
    }

    function_missing_return {
        args: (name: &str),
        msg: format!("function '{}' missing return for all paths", name),
        help: None,
    }

    function_return_validation {
        args: (name: &str, description: &str),
        msg: format!("function '{}' failed to validate return path: '{}'", name, description),
        help: None,
    }

    input_ref_needs_type {
        args: (category: &str, name: &str),
        msg: format!("could not infer type for input in '{}': '{}'", category, name),
        help: None,
    }

    invalid_self_in_global {
        args: (),
        msg: "cannot have `mut self` or `self` arguments in global functions", 
        help: None,
    }

    call_test_function {
        args: (),
        msg: "cannot call test function",
        help: None,
    }

    circuit_test_function {
        args: (),
        msg: "cannot have test function as member of circuit",
        help: None,
    }

    parse_index_error {
        args: (),
        msg: "failed to parse index",
        help: None,
    }

    parse_dimension_error {
        args: (),
        msg: "failed to parse dimension",
        help: None,
    }

    reference_self_outside_circuit {
        args: (),
        msg: "referenced self outside of circuit function",
        help: None,
    }

    illegal_ast_structure {
        args: (details: &str),
        msg: format!("illegal ast structure: {}", details),
        help: None,
    }

    illegal_input_variable_reference {
        args: (details: &str),
        msg: format!("illegal ast structure: {}", details),
        help: None,
    }
);
