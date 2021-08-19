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

use std::fmt::{Debug, Display};

create_errors!(
    /// AsgError enum that represents all the errors for the `leo-asg` crate.
    AsgError,
    exit_code_mask: 3000i32,
    error_code_prefix: "ASG",

    /// For when a circuit of the specified type is unresolved.
    /// Note that the type for a circuit is represented by a name.
    @formatted
    unresolved_circuit {
        args: (name: impl Display),
        msg: format!("failed to resolve circuit: '{}'", name),
        help: None,
    }

     /// For when a circuit member of the specified name is unresolved.
    @formatted
    unresolved_circuit_member {
        args: (circuit_name: impl Display, name: impl Display),
        msg: format!(
            "illegal reference to non-existant member '{}' of circuit '{}'",
            name, circuit_name
        ),
        help: None,
    }

    /// For when a user is initializing a circuit, and it's missing circuit member.
    @formatted
    missing_circuit_member {
        args: (circuit_name: impl Display, name: impl Display),
        msg: format!(
            "missing circuit member '{}' for initialization of circuit '{}'",
            name, circuit_name
        ),
        help: None,
    }

    /// For when a user is initializing a circuit, and they declare a cirucit member twice.
    @formatted
    overridden_circuit_member {
        args: (circuit_name: impl Display, name: impl Display),
        msg: format!(
            "cannot declare circuit member '{}' more than once for initialization of circuit '{}'",
            name, circuit_name
        ),
        help: None,
    }

    /// For when a user is defining a circuit, and they define a circuit member multiple times.
    @formatted
    redefined_circuit_member {
        args: (circuit_name: impl Display, name: impl Display),
        msg: format!(
            "cannot declare circuit member '{}' multiple times in circuit '{}'",
            name, circuit_name
        ),
        help: None,
    }

    /// For when a user is initializing a circuit, and they add an extra circuit member.
    @formatted
    extra_circuit_member {
        args: (circuit_name: impl Display, name: impl Display),
        msg: format!(
            "extra circuit member '{}' for initialization of circuit '{}' is not allowed",
            name, circuit_name
        ),
        help: None,
    }

    /// For when a user attempts to assign to a function.
    @formatted
    illegal_function_assign {
        args: (name: impl Display),
        msg: format!("attempt to assign to function '{}'", name),
        help: None,
    }

    /// For when a user tries to call a circuit variable as a function.
    @formatted
    circuit_variable_call {
        args: (circuit_name: impl Display, name: impl Display),
        msg: format!("cannot call variable member '{}' of circuit '{}'", name, circuit_name),
        help: None,
    }

    /// For when a user tries to call an invalid circuit static function.
    @formatted
    circuit_static_call_invalid {
        args: (circuit_name: impl Display, name: impl Display),
        msg: format!(
            "cannot call static function '{}' of circuit '{}' from target",
            name, circuit_name
        ),
        help: None,
    }

    /// For when a user tries to call a mutable circuit member function from immutable context.
    @formatted
    circuit_member_mut_call_invalid {
        args: (circuit_name: impl Display, name: impl Display),
        msg: format!(
            "cannot call mutable member function '{}' of circuit '{}' from immutable context",
            name, circuit_name
        ),
        help: None,
    }

    /// For when a user tries to call a circuit member function from static context.
    @formatted
    circuit_member_call_invalid {
        args: (circuit_name: impl Display, name: impl Display),
        msg: format!(
            "cannot call member function '{}' of circuit '{}' from static context",
            name, circuit_name
        ),
        help: None,
    }

    /// For when a user tries to index into a non-array type.
    @formatted
    index_into_non_array {
        args: (name: impl Display),
        msg: format!("failed to index into non-array '{}'", name),
        help: None,
    }

    /// For when a user tries index with an invalid integer.
    @formatted
    invalid_assign_index {
        args: (name: impl Display, num: impl Display),
        msg: format!("failed to index array with invalid integer '{}'[{}]", name, num),
        help: None,
    }

    /// For when a user tries to index an array range, with a left value greater than right value.
    @formatted
    invalid_backwards_assignment {
        args: (name: impl Display, left: impl Display, right: impl Display),
        msg: format!(
            "failed to index array range for assignment with left > right '{}'[{}..{}]",
            name, left, right
        ),
        help: None,
    }

    /// For when a user tries to create a constant varaible from non constant values.
    @formatted
    invalid_const_assign {
        args: (name: impl Display),
        msg: format!(
            "failed to create const variable(s) '{}' with non constant values.",
            name
        ),
        help: None,
    }

    /// For when a user defines function with the same name twice.
    @formatted
    duplicate_function_definition {
        args: (name: impl Display),
        msg: format!("a function named \"{}\" already exists in this scope", name),
        help: None,
    }

    /// For when a user defines variable with the same name twice.
    @formatted
    duplicate_variable_definition {
        args: (name: impl Display),
        msg: format!("a variable named \"{}\" already exists in this scope", name),
        help: None,
    }

    /// For when a user tries to index into a non-tuple type.
    @formatted
    index_into_non_tuple {
        args: (name: impl Display),
        msg: format!("failed to index into non-tuple '{}'", name),
        help: None,
    }

    /// For when a user tries access a tuple index out of bounds.
    @formatted
    tuple_index_out_of_bounds {
        args: (index: impl Display),
        msg: format!("tuple index out of bounds: '{}'", index),
        help: None,
    }

    /// For when a user tries access a array index out of bounds.
    @formatted
    array_index_out_of_bounds {
        args: (index: impl Display),
        msg: format!("array index out of bounds: '{}'", index),
        help: None,
    }

    /// For when a user tries have either side of a ternary return different variable types.
    @formatted
    ternary_different_types {
        args: (left: impl Display, right: impl Display),
        msg: format!("ternary sides had different types: left {}, right {}", left, right),
        help: None,
    }

    /// For when an array size cannot be inferred.
    @formatted
    unknown_array_size {
        args: (),
        msg: "array size cannot be inferred, add explicit types",
        help: None,
    }

    /// For when a user passes more arguements to a function than expected.
    @formatted
    unexpected_call_argument_count {
        args: (expected: impl Display, got: impl Display),
        msg: format!("function call expected {} arguments, got {}", expected, got),
        help: None,
    }

    /// For whan a function is unresolved.
    @formatted
    unresolved_function {
        args: (name: impl Display),
        msg: format!("failed to resolve function: '{}'", name),
        help: None,
    }

    /// For when a type cannot be resolved.
    @formatted
    unresolved_type {
        args: (name: impl Display),
        msg: format!("failed to resolve type for variable definition '{}'", name),
        help: None,
    }

    /// For when a user passes a type, but another was expected.
    @formatted
    unexpected_type {
        args: (expected: impl Display, received: impl Display),
        msg: format!(
            "unexpected type, expected: '{}', received: '{}'",
            expected,
            received,
        ),
        help: None,
    }

    /// For when a constant value was expected, but a non-constant one was received.
    @formatted
    unexpected_nonconst {
        args: (),
        msg: "expected const, found non-const value",
        help: None,
    }

    /// For whan a variable is unresolved.
    @formatted
    unresolved_reference {
        args: (name: impl Display),
        msg: format!("failed to resolve variable reference '{}'", name),
        help: None,
    }

    /// For when a boolean value cannot be parsed.
    @formatted
    invalid_boolean {
        args: (value: impl Display),
        msg: format!("failed to parse boolean value '{}'", value),
        help: None,
    }

    /// For when a char value cannot be parsed.
    @formatted
    invalid_char {
        args: (value: impl Display),
        msg: format!("failed to parse char value '{}'", value),
        help: None,
    }

    /// For when an int value cannot be parsed.
    @formatted
    invalid_int {
        args: (value: impl Display),
        msg: format!("failed to parse int value '{}'", value),
        help: None,
    }

    /// For when a user tries to negate an unsigned integer.
    @formatted
    unsigned_negation {
        args: (),
        msg: "cannot negate unsigned integer",
        help: None,
    }

    /// For when a user tries to assign to an immutable variable.
    @formatted
    immutable_assignment {
        args: (name: impl Display),
        msg: format!("illegal assignment to immutable variable '{}'", name),
        help: None,
    }

    /// For when a function is missing a return statement.
    @formatted
    function_missing_return {
        args: (name: impl Display),
        msg: format!("function '{}' missing return for all paths", name),
        help: None,
    }

    /// For when a function fails to resolve the correct return.
    @formatted
    function_return_validation {
        args: (name: impl Display, description: impl Display),
        msg: format!("function '{}' failed to validate return path: '{}'", name, description),
        help: None,
    }

    /// For when the type for an input variable could not be infered.
    @formatted
    input_ref_needs_type {
        args: (category: impl Display, name: impl Display),
        msg: format!("could not infer type for input in '{}': '{}'", category, name),
        help: None,
    }

    /// For when a user tries to call a test function.
    @formatted
    call_test_function {
        args: (),
        msg: "cannot call test function",
        help: None,
    }

    /// For when a user tries to define a circuit function as a test function.
    @formatted
    circuit_test_function {
        args: (),
        msg: "cannot have test function as member of circuit",
        help: None,
    }

    /// Failed to parse index.
    @formatted
    parse_index_error {
        args: (),
        msg: "failed to parse index",
        help: None,
    }

    /// Failed to parse array dimensions.
    @formatted
    parse_dimension_error {
        args: (),
        msg: "failed to parse dimension",
        help: None,
    }

    /// For when there is an illegal ast structure.
    @formatted
    illegal_ast_structure {
        args: (details: impl Display),
        msg: format!("illegal ast structure: {}", details),
        help: None,
    }

    /// For when a user tries to reference an input varaible but none is in scope.
    @formatted
    illegal_input_variable_reference {
        args: (),
        msg:  "attempted to reference input when none is in scope",
        help: None,
    }

    /// For the ASG receives an big Self, which should never happen
    /// as they should be resolved in an earlier compiler phase.
    @formatted
    unexpected_big_self {
        args: (),
        msg:  "received a Self statement, which should never happen.",
        help: Some("Something went wrong during canonicalization, or you ran the ASG on an uncanonicalized AST.".to_string()),
    }
);
