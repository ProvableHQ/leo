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

use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

create_errors!(
    CompilerError,
    exit_code_mask: 2000u32,
    error_code_prefix: "CMP",

    @backtraced
    invalid_test_context {
        args: (name: impl Display),
        msg: format!("Cannot find input files with context name `{}`", name),
        help: None,
    }

    @backtraced
    file_read_error {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("Cannot read from the provided file path '{:?}': {}", path, error),
        help: None,
    }

    @backtraced
    no_main_function {
        args: (),
        msg: "There must be a function named `main`",
        help: None,
    }

    @backtraced
    no_test_input {
        args: (),
        msg: "Failed to find input files for the current test",
        help: None,
    }

    @formatted
    console_fmt_expected_left_or_right_brace {
        args: (),
        msg: "Formatter given a {. Expected a { or } after",
        help: None,
    }

    @formatted
    console_fmt_expected_escaped_right_brace {
        args: (),
        msg: "Formatter given a }. Expected a container {} or }}",
        help: None,
    }

    @formatted
    console_container_parameter_length_mismatch {
        args: (containers: impl Display, parameters: impl Display),
        msg: format!(
            "Formatter given {} containers and found {} parameters",
            containers, parameters
        ),
        help: None,
    }

    @formatted
    console_assertion_depends_on_input {
        args: (),
        msg: "console.assert() does not produce constraints and cannot use inputs. \
        Assertions should only be used in @test functions",
        help: None,
    }

    @formatted
    console_assertion_failed {
        args: (),
        msg:  "console.assert(...) failed",
        help: None,
    }

    @formatted
    console_assertion_must_be_boolean {
        args: (),
        msg: "Assertion expression must evaluate to a boolean value",
        help: None,
    }

    @formatted
    cannot_enforce_expression {
        args: (operation: impl Display, error: impl ErrorArg),
        msg: format!(
            "the gadget operation `{}` failed due to synthesis error `{:?}`",
            operation, error,
        ),
        help: None,
    }

    @formatted
    cannot_evaluate_expression {
        args: (operation: impl Display),
        msg: format!("Mismatched types found for operation `{}`", operation),
        help: None,
    }

    @formatted
    array_length_out_of_bounds {
        args: (),
        msg:  "array length cannot be >= 2^32",
        help: None,
    }

    @formatted
    array_index_out_of_legal_bounds {
        args: (),
        msg: "array index cannot be >= 2^32",
        help: None,
    }

    @formatted
    conditional_boolean_expression_fails_to_resolve_to_bool {
        args: (actual: impl Display),
        msg: format!("if, else conditional must resolve to a boolean, found `{}`", actual),
        help: None,
    }

    @formatted
    expected_circuit_member {
        args: (expected: impl Display),
        msg: format!("expected circuit member `{}`, not found", expected),
        help: None,
    }

    @formatted
    incompatible_types {
        args: (operation: impl Display),
        msg: format!("no implementation for `{}`", operation),
        help: None,
    }

    @formatted
    tuple_index_out_of_bounds {
        args: (index: impl Display),
        msg: format!("cannot access index {} of tuple out of bounds", index),
        help: None,
    }

    @formatted
    array_index_out_of_bounds {
        args: (index: impl Display),
        msg: format!("cannot access index {} of array out of bounds", index),
        help: None,
    }

    @formatted
    array_invalid_slice_length {
        args: (),
        msg: "illegal length of slice",
        help: None,
    }

    @formatted
    invalid_index_expression {
        args: (actual: impl Display),
        msg: format!("index must resolve to an integer, found `{}`", actual),
        help: None,
    }

    @formatted
    unexpected_array_length {
        args: (expected: impl Display, actual: impl Display),
        msg: format!("expected array length {}, found one with length {}", expected, actual),
        help: None,
    }

    @formatted
    invalid_circuit_static_member_access {
        args: (member: impl Display),
        msg: format!("invalid circuit static member `{}` must be accessed using `::` syntax", member),
        help: None,
    }

    @formatted
    undefined_array {
        args: (actual: impl Display),
        msg: format!("array `{}` must be declared before it is used in an expression", actual),
        help: None,
    }

    @formatted
    undefined_circuit {
        args: (actual: impl Display),
        msg:  format!(
            "circuit `{}` must be declared before it is used in an expression",
            actual
        ),
        help: None,
    }

    @formatted
    undefined_identifier {
        args: (name: impl Display),
        msg: format!("Cannot find value `{}` in this scope", name),
        help: None,
    }

    @formatted
    undefined_circuit_member_access {
        args: (circuit: impl Display, member: impl Display),
        msg: format!("Circuit `{}` has no member `{}`", circuit, member),
        help: None,
    }

    @formatted
    input_variable_type_mismatch {
        args: (expected: impl Display, actual: impl Display, variable: impl Display),
        msg: format!(
            "Expected input variable `{}` to be type `{}`, found type `{}`",
            variable, expected, actual
        ),
        help: None,
    }

    @formatted
    expected_const_input_variable {
        args: (variable: impl Display),
        msg:  format!(
            "Expected input variable `{}` to be constant. Move input variable `{}` to [constants] section of input file",
            variable, variable
        ),
        help: None,
    }

    @formatted
    expected_non_const_input_variable {
        args: (variable: impl Display),
        msg: format!(
            "Expected input variable `{}` to be non-constant. Move input variable `{}` to [main] section of input file",
            variable, variable
        ),
        help: None,
    }

    @formatted
    invalid_function_input_array {
        args: (actual: impl Display),
        msg: format!("Expected function input array, found `{}`", actual),
        help: None,
    }

    @formatted
    invalid_input_array_dimensions {
        args: (expected: impl Display, actual: impl Display),
        msg: format!(
            "Input array dimensions mismatch expected {}, found array dimensions {}",
            expected, actual
        ),
        help: None,
    }

    @formatted
    input_tuple_size_mismatch {
        args: (expected: impl Display, actual: impl Display),
        msg: format!(
            "Input tuple size mismatch expected {}, found tuple with length {}",
            expected, actual
        ),
        help: None,
    }

    @formatted
    invalid_function_input_tuple {
        args: (actual: impl Display),
        msg: format!("Expected function input tuple, found `{}`", actual),
        help: None,
    }

    @formatted
    function_input_not_found {
        args: (function: impl Display, expected: impl Display),
        msg: format!("function `{}` input {} not found", function, expected),
        help: None,
    }

    @formatted
    double_input_declaration {
        args: (input_name: impl Display),
        msg: format!("Input variable {} declared twice", input_name),
        help: None,
    }

    @formatted
    output_not_enough_registers {
        args: (),
        msg: "number of input registers must be greater than or equal to output registers",
        help: None,
    }

    @formatted
    output_mismatched_types {
        args: (left: impl Display, right: impl Display),
        msg: format!(
            "Mismatched types. Expected register output type `{}`, found type `{}`.",
            left, right
        ),
        help: None,
    }

    @backtraced
    output_file_error {
        args: (error: impl ErrorArg),
        msg: error,
        help: None,
    }

    @backtraced
    output_file_io_error {
        args: (error: impl ErrorArg),
        msg: error,
        help: None,
    }

    @backtraced
    output_file_cannot_read {
        args: (path: impl Debug),
        msg: format!("Cannot read the provided ouput file - {:?}", path),
        help: None,
    }

    @backtraced
    output_file_cannot_remove {
        args: (path: impl Debug),
        msg: format!("Cannot remove the provided ouput file - {:?}", path),
        help: None,
    }

    @formatted
    statement_array_assign_index {
        args: (),
        msg: "Cannot assign single index to array of values",
        help: None,
    }

    @formatted
    statement_array_assign_index_const {
        args: (),
        msg: "Cannot assign to non-const array index",
        help: None,
    }

    @formatted
    statement_array_assign_interior_index {
        args: (),
        msg: "Cannot assign single index to interior of array of values",
        help: None,
    }

    @formatted
    statement_array_assign_range {
        args: (),
        msg: "Cannot assign range of array values to single value",
        help: None,
    }

    @formatted
    statement_array_assign_index_bounds {
        args: (index: impl Display, length: impl Display),
        msg: format!(
            "Array assign index `{}` out of range for array of length `{}`",
            index, length
        ),
        help: None,
    }

    @formatted
    statement_array_assign_range_order {
        args: (start: impl Display, stop: impl Display, length: impl Display),
        msg: format!(
            "Array assign range `{}`..`{}` out of range for array of length `{}`",
            start, stop, length
        ),
        help: None,
    }

    @formatted
    statement_conditional_boolean_fails_to_resolve_to_boolean {
        args: (actual: impl Display),
        msg: format!("If, else conditional must resolve to a boolean, found `{}`", actual),
        help: None,
    }

    @formatted
    statement_indicator_calculation {
        args: (name: impl Display),
        msg: format!(
            "Constraint system failed to evaluate branch selection indicator `{}`",
            name
        ),
        help: None,
    }

    @formatted
    statement_invalid_number_of_definitions {
        args: (expected: impl Display, actual: impl Display),
        msg: format!(
            "Multiple definition statement expected {} return values, found {} values",
            expected, actual
        ),
        help: None,
    }

    @formatted
    statement_multiple_definition {
        args: (value: impl Display),
        msg: format!("cannot assign multiple variables to a single value: {}", value),
        help: None,
    }

    @formatted
    statement_multiple_returns {
        args: (),
        msg: "This function returns multiple times and produces unreachable circuits with undefined behavior.",
        help: None,
    }

    @formatted
    statement_no_returns {
        args: (expected: impl Display),
        msg: format!(
            "function expected `{}` return type but no valid branches returned a result",
            expected
        ),
        help: None,
    }

    @formatted
    statement_select_fail {
        args: (first: impl Display, second: impl Display),
        msg: format!(
            "Conditional select gadget failed to select between `{}` or `{}`",
            first, second
        ),
        help: None,
    }

    @formatted
    statement_tuple_assign_index {
        args: (),
        msg: "Cannot assign single index to tuple of values",
        help: None,
    }

    @formatted
    statement_tuple_assign_index_bounds {
        args: (index: impl Display, length: impl Display),
        msg: format!(
            "Tuple assign index `{}` out of range for tuple of length `{}`",
            index, length
        ),
        help: None,
    }

    @formatted
    statement_unassigned {
        args: (),
        msg: "Expected assignment of return values for expression",
        help: None,
    }

    @formatted
    statement_undefined_variable {
        args: (name: impl Display),
        msg: format!("Attempted to assign to unknown variable `{}`", name),
        help: None,
    }

    @formatted
    statement_undefined_circuit_variable {
        args: (name: impl Display),
        msg: format!("Attempted to assign to unknown circuit member variable `{}`", name),
        help: None,
    }

    @formatted
    statement_loop_index_const {
        args: (),
        msg: "iteration range must be const",
        help: None,
    }

    @formatted
    address_value_account_error {
        args: (error: impl ErrorArg),
        msg: format!("account creation failed due to `{}`", error),
        help: None,
    }

    @formatted
    address_value_invalid_address {
        args: (actual: impl Display),
        msg: format!("expected address input type, found `{}`", actual),
        help: None,
    }

    @formatted
    address_value_missing_address {
        args: (),
        msg: "expected address input not found",
        help: None,
    }

    @formatted
    boolean_value_cannot_enforce {
        args: (operation: impl Display, error: impl ErrorArg),
        msg: format!(
            "the boolean operation `{}` failed due to the synthesis error `{}`",
            operation, error,
        ),
        help: None,
    }

    @formatted
    boolean_value_cannot_evaluate {
        args: (operation: impl Display),
        msg: format!("no implementation found for `{}`", operation),
        help: None,
    }

    @formatted
    boolean_value_invalid_boolean {
        args: (actual: impl Display),
        msg: format!("expected boolean input type, found `{}`", actual),
        help: None,
    }

    @formatted
    boolean_value_missing_boolean {
        args: (expected: impl Display),
        msg: format!("expected boolean input `{}` not found", expected),
        help: None,
    }

    @formatted
    char_value_invalid_char {
        args: (actual: impl Display),
        msg: format!("expected char element input type, found `{}`", actual),
        help: None,
    }

    @formatted
    field_value_negate_operation {
        args: (error: impl ErrorArg),
        msg: format!("field negation failed due to synthesis error `{}`", error),
        help: None,
    }

    @formatted
    field_value_binary_operation {
        args: (operation: impl Display, error: impl ErrorArg),
        msg: format!(
            "the field binary operation `{}` failed due to synthesis error `{}`",
            operation, error,
        ),
        help: None,
    }

    @formatted
    field_value_invalid_field {
        args: (actual: impl Display),
        msg: format!("expected field element input type, found `{}`", actual),
        help: None,
    }

    @formatted
    field_value_missing_field {
        args: (expected: impl Display),
        msg: format!("expected field input `{}` not found", expected),
        help: None,
    }

    @formatted
    field_value_no_inverse {
        args: (field: impl Display),
        msg: format!("no multiplicative inverse found for field `{}`", field),
        help: None,
    }

    @formatted
    group_value_negate_operation {
        args: (error: impl ErrorArg),
        msg: format!("group negation failed due to the synthesis error `{}`", error),
        help: None,
    }

    @formatted
    group_value_binary_operation {
        args: (operation: impl Display, error: impl ErrorArg),
        msg: format!(
            "the group binary operation `{}` failed due to the synthesis error `{}`",
            operation, error,
        ),
        help: None,
    }

    @formatted
    group_value_invalid_group {
        args: (actual: impl Display),
        msg: format!("expected group affine point input type, found `{}`", actual),
        help: None,
    }

    @formatted
    group_value_missing_group {
        args: (expected: impl Display),
        msg: format!("expected group input `{}` not found", expected),
        help: None,
    }

    @formatted
    group_value_synthesis_error {
        args: (error: impl ErrorArg),
        msg: format!("compilation failed due to group synthesis error `{}`", error),
        help: None,
    }

    @formatted
    group_value_x_invalid {
        args: (x: impl Display),
        msg: format!("invalid x coordinate `{}`", x),
        help: None,
    }

    @formatted
    group_value_y_invalid {
        args: (y: impl Display),
        msg: format!("invalid y coordinate `{}`", y),
        help: None,
    }

    @formatted
    group_value_not_on_curve {
        args: (element: impl Display),
        msg: format!("group element `{}` is not on the supported curve", element),
        help: None,
    }

    @formatted
    group_value_x_recover {
        args: (),
        msg: "could not recover group element from x coordinate",
        help: None,
    }

    @formatted
    group_value_y_recover {
        args: (),
        msg: "could not recover group element from y coordinate",
        help: None,
    }

    @formatted
    group_value_n_group {
        args: (number: impl Display),
        msg: format!("cannot multiply group generator by \"{}\"", number),
        help: None,
    }

    @formatted
    integer_value_signed {
        args: (error: impl ErrorArg),
        msg: format!("integer operation failed due to the signed integer error `{}`", error),
        help: None,
    }

    @formatted
    integer_value_unsigned {
        args: (error: impl ErrorArg),
        msg: format!(
            "integer operation failed due to the unsigned integer error `{}`",
            error
        ),
        help: None,
    }

    @formatted
    integer_value_synthesis {
        args: (error: impl ErrorArg),
        msg: format!("integer operation failed due to the synthesis error `{}`", error),
        help: None,
    }

    @formatted
    integer_value_negate_operation {
        args: (),
        msg: "integer negation can only be enforced on signed integers",
        help: None,
    }

    @formatted
    integer_value_binary_operation {
        args: (operation: impl Display),
        msg: format!(
            "the integer binary operation `{}` can only be enforced on integers of the same type",
            operation
        ),
        help: None,
    }

    @formatted
    integer_value_integer_type_mismatch {
        args: (expected: impl Display, received: impl Display),
        msg: format!("expected data type `{}`, found `{}`", expected, received),
        help: None,
    }

    @formatted
    integer_value_invalid_integer {
        args: (actual: impl Display),
        msg: format!("failed to parse `{}` as expected integer type", actual),
        help: None,
    }

    @formatted
    integer_value_missing_integer {
        args: (expected: impl Display),
        msg: format!("expected integer input `{}` not found", expected),
        help: None,
    }

    @formatted
    integer_value_cannot_evaluate {
        args: (operation: impl Display),
        msg: format!("no implementation found for `{}`", operation),
        help: None,
    }
);
