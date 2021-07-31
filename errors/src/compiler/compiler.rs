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
    CompilerError,
    exit_code_mask: 2000u32,
    error_code_prefix: "G",
);

// impl CompilerError {
//     pub fn expected_left_or_right_brace(span: &Span) -> Self {
//         let message = "Formatter given a {. Expected a { or } after".to_string();

//         Self::new_from_span(message, None, 0, span)
//     }

//     pub fn expected_escaped_right_brace(span: &Span) -> Self {
//         let message = "Formatter given a }. Expected a container {} or }}".to_string();

//         Self::new_from_span(message, None, 1, span)
//     }

//     pub fn length(containers: usize, parameters: usize, span: &Span) -> Self {
//         let message = format!(
//             "Formatter given {} containers and found {} parameters",
//             containers, parameters
//         );

//         Self::new_from_span(message, None, 2, span)
//     }

//     pub fn assertion_depends_on_input(span: &Span) -> Self {
//         let message = "console.assert() does not produce constraints and cannot use inputs. \
//         Assertions should only be used in @test functions"
//             .to_string();

//         Self::new_from_span(message, None, 3, span)
//     }

//     pub fn assertion_failed(span: &Span) -> Self {
//         let message = "Assertion failed".to_string();

//         Self::new_from_span(message, None, 4, span)
//     }

//     pub fn assertion_must_be_boolean(span: &Span) -> Self {
//         let message = "Assertion expression must evaluate to a boolean value".to_string();

//         Self::new_from_span(message, None, 5, span)
//     }

//     pub fn cannot_enforce(operation: String, error: String, span: &Span) -> Self {
//         let message = format!(
//             "the gadget operation `{}` failed due to synthesis error `{}`",
//             operation, error,
//         );

//         Self::new_from_span(message, None, 6, span)
//     }

//     pub fn cannot_evaluate(operation: String, span: &Span) -> Self {
//         let message = format!("Mismatched types found for operation `{}`", operation);

//         Self::new_from_span(message, None, 7, span)
//     }

//     pub fn array_length_out_of_bounds(span: &Span) -> Self {
//         let message = "array length cannot be >= 2^32".to_string();

//         Self::new_from_span(message, None, 8, span)
//     }

//     pub fn array_index_out_of_legal_bounds(span: &Span) -> Self {
//         let message = "array index cannot be >= 2^32".to_string();

//         Self::new_from_span(message, None, 9, span)
//     }

//     pub fn conditional_boolean(actual: String, span: &Span) -> Self {
//         let message = format!("if, else conditional must resolve to a boolean, found `{}`", actual);

//         Self::new_from_span(message, None, 10, span)
//     }

//     pub fn expected_circuit_member(expected: String, span: &Span) -> Self {
//         let message = format!("expected circuit member `{}`, not found", expected);

//         Self::new_from_span(message, None, 11, span)
//     }

//     pub fn incompatible_types(operation: String, span: &Span) -> Self {
//         let message = format!("no implementation for `{}`", operation);

//         Self::new_from_span(message, None, 12, span)
//     }

//     pub fn tuple_index_out_of_bounds(index: usize, span: &Span) -> Self {
//         let message = format!("cannot access index {} of tuple out of bounds", index);

//         Self::new_from_span(message, None, 13, span)
//     }

//     pub fn array_index_out_of_bounds(index: usize, span: &Span) -> Self {
//         let message = format!("cannot access index {} of array out of bounds", index);

//         Self::new_from_span(message, None, 14, span)
//     }

//     pub fn array_invalid_slice_length(span: &Span) -> Self {
//         let message = "illegal length of slice".to_string();

//         Self::new_from_span(message, None, 15, span)
//     }

//     pub fn invalid_index(actual: String, span: &Span) -> Self {
//         let message = format!("index must resolve to an integer, found `{}`", actual);

//         Self::new_from_span(message, None, 16, span)
//     }

//     pub fn invalid_length(expected: usize, actual: usize, span: &Span) -> Self {
//         let message = format!("expected array length {}, found one with length {}", expected, actual);

//         Self::new_from_span(message, None, 17, span)
//     }

//     pub fn invalid_static_access(member: String, span: &Span) -> Self {
//         let message = format!("static member `{}` must be accessed using `::` syntax", member);

//         Self::new_from_span(message, None, 18, span)
//     }

//     pub fn undefined_array(actual: String, span: &Span) -> Self {
//         let message = format!("array `{}` must be declared before it is used in an expression", actual);

//         Self::new_from_span(message, None, 19, span)
//     }

//     pub fn undefined_circuit(actual: String, span: &Span) -> Self {
//         let message = format!(
//             "circuit `{}` must be declared before it is used in an expression",
//             actual
//         );

//         Self::new_from_span(message, None, 20, span)
//     }

//     pub fn undefined_identifier(name: &str, span: &Span) -> Self {
//         let message = format!("Cannot find value `{}` in this scope", name);

//         Self::new_from_span(message, None, 21, span)
//     }

//     pub fn undefined_member_access(circuit: String, member: String, span: &Span) -> Self {
//         let message = format!("Circuit `{}` has no member `{}`", circuit, member);

//         Self::new_from_span(message, None, 22, span)
//     }

//     pub fn input_type_mismatch(expected: String, actual: String, variable: String, span: &Span) -> Self {
//         let message = format!(
//             "Expected input variable `{}` to be type `{}`, found type `{}`",
//             variable, expected, actual
//         );

//         Self::new_from_span(message, None, 23, span)
//     }

//     pub fn expected_const_input(variable: String, span: &Span) -> Self {
//         let message = format!(
//             "Expected input variable `{}` to be constant. Move input variable `{}` to [constants] section of input file",
//             variable, variable
//         );

//         Self::new_from_span(message, None, 24, span)
//     }

//     pub fn expected_non_const_input(variable: String, span: &Span) -> Self {
//         let message = format!(
//             "Expected input variable `{}` to be non-constant. Move input variable `{}` to [main] section of input file",
//             variable, variable
//         );

//         Self::new_from_span(message, None, 25, span)
//     }

//     pub fn invalid_array(actual: String, span: &Span) -> Self {
//         let message = format!("Expected function input array, found `{}`", actual);

//         Self::new_from_span(message, None, 26, span)
//     }

//     pub fn invalid_input_array_dimensions(expected: usize, actual: usize, span: &Span) -> Self {
//         let message = format!(
//             "Input array dimensions mismatch expected {}, found array dimensions {}",
//             expected, actual
//         );

//         Self::new_from_span(message, None, 27, span)
//     }

//     pub fn tuple_size_mismatch(expected: usize, actual: usize, span: &Span) -> Self {
//         let message = format!(
//             "Input tuple size mismatch expected {}, found tuple with length {}",
//             expected, actual
//         );

//         Self::new_from_span(message, None, 28, span)
//     }

//     pub fn invalid_tuple(actual: String, span: &Span) -> Self {
//         let message = format!("Expected function input tuple, found `{}`", actual);

//         Self::new_from_span(message, None, 29, span)
//     }

//     pub fn input_not_found(expected: String, span: &Span) -> Self {
//         let message = format!("main function input {} not found", expected);

//         Self::new_from_span(message, None, 30, span)
//     }

//     pub fn double_input_declaration(input_name: String, span: &Span) -> Self {
//         let message = format!("Input variable {} declared twice", input_name);

//         Self::new_from_span(message, None, 31, span)
//     }

//     pub fn not_enough_registers(span: &Span) -> Self {
//         let message = "number of input registers must be greater than or equal to output registers".to_string();

//         Self::new_from_span(message, None, 32, span)
//     }

//     pub fn mismatched_output_types(left: &str, right: &str, span: &Span) -> Self {
//         let message = format!(
//             "Mismatched types. Expected register output type `{}`, found type `{}`.",
//             left, right
//         );

//         Self::new_from_span(message, None, 33, span)
//     }

//     pub fn account_error(error: String, span: &Span) -> Self {
//         let message = format!("account creation failed due to `{}`", error);

//         Self::new_from_span(message, None, 34, span)
//     }

//     pub fn invalid_address(actual: String, span: &Span) -> Self {
//         let message = format!("expected address input type, found `{}`", actual);

//         Self::new_from_span(message, None, 35, span)
//     }

//     pub fn missing_address(span: &Span) -> Self {
//         let message = "expected address input not found".to_string();

//         Self::new_from_span(message, None, 36, span)
//     }

//     pub fn negate_operation(error: String, span: &Span) -> Self {
//         let message = format!("field negation failed due to synthesis error `{:?}`", error,);

//         Self::new_from_span(message, None, 37, span)
//     }

//     pub fn binary_operation(operation: String, error: String, span: &Span) -> Self {
//         let message = format!(
//             "the field binary operation `{}` failed due to synthesis error `{:?}`",
//             operation, error,
//         );

//         Self::new_from_span(message, None, 38, span)
//     }

//     pub fn invalid_field(actual: String, span: &Span) -> Self {
//         let message = format!("expected field element input type, found `{}`", actual);

//         Self::new_from_span(message, None, 39, span)
//     }

//     pub fn missing_field(expected: String, span: &Span) -> Self {
//         let message = format!("expected field input `{}` not found", expected);

//         Self::new_from_span(message, None, 40, span)
//     }

//     pub fn no_inverse(field: String, span: &Span) -> Self {
//         let message = format!("no multiplicative inverse found for field `{}`", field);

//         Self::new_from_span(message, None, 41, span)
//     }

//     pub fn x_invalid(x: String, span: &Span) -> Self {
//         let message = format!("invalid x coordinate `{}`", x);

//         Self::new_from_span(message, None, 42, span)
//     }

//     pub fn y_invalid(y: String, span: &Span) -> Self {
//         let message = format!("invalid y coordinate `{}`", y);

//         Self::new_from_span(message, None, 43, span)
//     }

//     pub fn not_on_curve(element: String, span: &Span) -> Self {
//         let message = format!("group element `{}` is not on the supported curve", element);

//         Self::new_from_span(message, None, 44, span)
//     }

//     pub fn x_recover(span: &Span) -> Self {
//         let message = "could not recover group element from x coordinate".to_string();

//         Self::new_from_span(message, None, 45, span)
//     }

//     pub fn y_recover(span: &Span) -> Self {
//         let message = "could not recover group element from y coordinate".to_string();

//         Self::new_from_span(message, None, 46, span)
//     }

//     pub fn n_group(number: String, span: &Span) -> Self {
//         let message = format!("cannot multiply group generator by \"{}\"", number);

//         Self::new_from_span(message, None, 47, span)
//     }

//     pub fn signed(error: String, span: &Span) -> Self {
//         let message = format!("integer operation failed due to the signed integer error `{:?}`", error);

//         Self::new_from_span(message, None, 48, span)
//     }

//     pub fn unsigned(error: String, span: &Span) -> Self {
//         let message = format!(
//             "integer operation failed due to the unsigned integer error `{:?}`",
//             error
//         );

//         Self::new_from_span(message, None, 49, span)
//     }

//     pub fn integer_synthesis(error: String, span: &Span) -> Self {
//         let message = format!("integer operation failed due to the synthesis error `{}`", error);

//         Self::new_from_span(message, None, 50, span)
//     }
    
//     pub fn invalid_unsigned_negate(span: &Span) -> Self {
//         let message = "integer negation can only be enforced on signed integers".to_string();

//         Self::new_from_span(message, None, 51, span)
//     }

//     pub fn invalid_integer_binary_operation(operation: String, span: &Span) -> Self {
//         let message = format!(
//             "the integer binary operation `{}` can only be enforced on integers of the same type",
//             operation
//         );

//         Self::new_from_span(message, None, 52, span)
//     }

//     pub fn integer_type_mismatch(expected: String, received: String, span: &Span) -> Self {
//         let message = format!("expected data type `{}`, found `{}`", expected, received);

//         Self::new_from_span(message, None, 53, span)
//     }

//     pub fn invalid_integer(actual: String, span: &Span) -> Self {
//         let message = format!("failed to parse `{}` as expected integer type", actual);

//         Self::new_from_span(message, None, 54, span)
//     }

//     pub fn missing_integer(expected: String, span: &Span) -> Self {
//         let message = format!("expected integer input `{}` not found", expected);

//         Self::new_from_span(message, None, 55, span)
//     }

//     pub fn invalid_group(actual: String, span: &Span) -> Self {
//         let message = format!("expected group affine point input type, found `{}`", actual);

//         Self::new_from_span(message, None, 56, span)
//     }

//     pub fn missing_group(expected: String, span: &Span) -> Self {
//         let message = format!("expected group input `{}` not found", expected);

//         Self::new_from_span(message, None, 57, span)
//     }

// }
