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
    /// CompilerError enum that represents all the errors for the `leo-compiler` crate.
    CompilerError,
    exit_code_mask: 6000i32,
    error_code_prefix: "CMP",

    /// For when the test function has invalid test context.
    @backtraced
    invalid_test_context {
        args: (name: impl Display),
        msg: format!("Cannot find input files with context name `{}`", name),
        help: None,
    }

    /// For when the compiler can't read a file from the provided path.
    @backtraced
    file_read_error {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("Cannot read from the provided file path '{:?}': {}", path, error),
        help: None,
    }

     /// For when there is no main function in a Leo program.
    @backtraced
    no_main_function {
        args: (),
        msg: "There must be a function named `main`",
        help: None,
    }

     /// For when the compiler can't find the test input files with the specified name.
    @backtraced
    no_test_input {
        args: (),
        msg: "Failed to find input files for the current test",
        help: None,
    }

    /// For when the console formatter expected a left or right brace after a left brace.
    @formatted
    console_fmt_expected_left_or_right_brace {
        args: (),
        msg: "Formatter given a {. Expected a { or } after",
        help: None,
    }

    /// For when the console formatter expected a right brace after a right brace.
    @formatted
    console_fmt_expected_escaped_right_brace {
        args: (),
        msg: "Formatter given a }. Expected a container {} or }}",
        help: None,
    }

    /// For when the amount of arguments, and number of containers mismatch
    /// in a format statement.
    @formatted
    console_container_parameter_length_mismatch {
        args: (containers: impl Display, parameters: impl Display),
        msg: format!(
            "Formatter given {} containers and found {} parameters",
            containers, parameters
        ),
        help: None,
    }

    /// For when a experssion gadget oepration cannot be enforced due to a SnarkVM syntehsis error.
    @formatted
    cannot_enforce_expression {
        args: (operation: impl Display, error: impl ErrorArg),
        msg: format!(
            "the gadget operation `{}` failed due to synthesis error `{:?}`",
            operation, error,
        ),
        help: None,
    }

    /// For when an expression has mismatching types for an operation.
    @formatted
    cannot_evaluate_expression {
        args: (operation: impl Display),
        msg: format!("Mismatched types found for operation `{}`", operation),
        help: None,
    }

    /// For when the expected circuit member could not be found.
    @formatted
    expected_circuit_member {
        args: (expected: impl Display),
        msg: format!("expected circuit member `{}`, not found", expected),
        help: None,
    }

    /// For when an array index does not resolve to an integer type.
    @formatted
    invalid_index_expression {
        args: (actual: impl Display),
        msg: format!("index must resolve to an integer, found `{}`", actual),
        help: None,
    }

    /// For when the input variable type mismatches the declared function input type.
    @formatted
    input_variable_type_mismatch {
        args: (expected: impl Display, actual: impl Display, variable: impl Display),
        msg: format!(
            "Expected input variable `{}` to be type `{}`, found type `{}`",
            variable, expected, actual
        ),
        help: None,
    }

    /// For when the declared function input variable was expected to be a valid array
    /// in the input file.
    @formatted
    invalid_function_input_array {
        args: (actual: impl Display),
        msg: format!("Expected function input array, found `{}`", actual),
        help: None,
    }

    /// For when the declared function input variable was expected to be an array with differing dimensions.
    @formatted
    invalid_input_array_dimensions {
        args: (expected: impl Display, actual: impl Display),
        msg: format!(
            "Input array dimensions mismatch expected {}, found array dimensions {}",
            expected, actual
        ),
        help: None,
    }

    /// For when the declared function input variable was expected to be a tuple
    /// with a different number of arguments.
    @formatted
    input_tuple_size_mismatch {
        args: (expected: impl Display, actual: impl Display),
        msg: format!(
            "Input tuple size mismatch expected {}, found tuple with length {}",
            expected, actual
        ),
        help: None,
    }

    /// For when the declared function input variable was expected to be a valid tuple
    /// in the input file.
    @formatted
    invalid_function_input_tuple {
        args: (actual: impl Display),
        msg: format!("Expected function input tuple, found `{}`", actual),
        help: None,
    }

    /// For when the declared function input variable was not defined
    /// in the input file.
    @formatted
    function_input_not_found {
        args: (function: impl Display, expected: impl Display),
        msg: format!("function `{}` input {} not found", function, expected),
        help: None,
    }

    /// For when the declared function input register was not defined
    @formatted
    function_missing_input_register {
        args: (expected: impl Display),
        msg: format!("missing input '{}' for registers", expected),
        help: None,
    }

    /// For when the declared function input variable was defined twice
    /// in the input file.
    @formatted
    double_input_declaration {
        args: (input_name: impl Display),
        msg: format!("Input variable {} declared twice", input_name),
        help: None,
    }

    /// For when the input file does not define enough registers.
    @formatted
    output_not_enough_registers {
        args: (),
        msg: "number of input registers must be greater than or equal to output registers",
        help: None,
    }

    /// For when the input file register types do not match the output types being generated.
    @formatted
    output_mismatched_types {
        args: (left: impl Display, right: impl Display),
        msg: format!(
            "Mismatched types. Expected register output type `{}`, found type `{}`.",
            left, right
        ),
        help: None,
    }

    /// For when a circuit was passed as input
    @formatted
    circuit_as_input {
        args: (),
        msg: "input circuits not supported for input",
        help: None,
    }

    /// For when there's an IO error with the output file.
    @backtraced
    output_file_io_error {
        args: (error: impl ErrorArg),
        msg: error,
        help: None,
    }

    /// For when the output file cannot be removed.
    @backtraced
    output_file_cannot_remove {
        args: (path: impl Debug),
        msg: format!("Cannot remove the provided ouput file - {:?}", path),
        help: None,
    }

    /// For when a function returns multiple times.
    @formatted
    statement_multiple_returns {
        args: (),
        msg: "This function returns multiple times and produces unreachable circuits with undefined behavior.",
        help: None,
    }

    /// For when a function expects a return type and has no valid return statements.
    @formatted
    statement_no_returns {
        args: (expected: impl Display),
        msg: format!(
            "function expected `{}` return type but no valid branches returned a result",
            expected
        ),
        help: None,
    }

    /// For when SnarkVM fails to conditionally select between values for a gadget.
    @formatted
    statement_select_fail {
        args: (first: impl Display, second: impl Display),
        msg: format!(
            "Conditional select gadget failed to select between `{}` or `{}`",
            first, second
        ),
        help: None,
    }

    /// For when there is an invalid address value.
    @formatted
    address_value_invalid_address {
        args: (actual: impl Display),
        msg: format!("expected address input type, found `{}`", actual),
        help: None,
    }

    /// For when there is an integer type mismatch, one kind was expected but another was received.
    @formatted
    integer_value_integer_type_mismatch {
        args: (expected: impl Display, received: impl Display),
        msg: format!("expected data type `{}`, found `{}`", expected, received),
        help: None,
    }

    /// For when .len() method is used on non-array values/variables.
    @formatted
    lengthof_can_only_be_used_on_arrays {
        args: (),
        msg: "len() can only be called on an array value".to_string(),
        help: None,
    }
);
