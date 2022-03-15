// Copyright (C) 2019-2022 Aleo Systems Inc.
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
    /// ParserError enum that represents all the errors for the `leo-parser` crate.
    ParserError,
    exit_code_mask: 0000i32,
    error_code_prefix: "PAR",

    /// For when the parser encountered an unexpected token.
    @formatted
    unexpected_token {
        args: (message: impl Display),
        msg: message,
        help: None,
    }

    /// For when the parser encountered an invalid address literal.
    @formatted
    invalid_address_lit {
        args: (token: impl Display),
        msg: format!("invalid address literal: '{}'", token),
        help: None,
    }

    /// For when the parser encountered an empty import list.
    @formatted
    invalid_import_list {
        args: (),
        msg: "Cannot import empty list",
        help: None,
    }

    /// For when the parser encountered an unexpected End of File.
    @formatted
    unexpected_eof {
        args: (),
        msg: "unexpected EOF",
        help: None,
    }

    /// For when the parser encountered an unexpected whitespace.
    @formatted
    unexpected_whitespace {
        args: (left: impl Display, right: impl Display),
        msg: format!("Unexpected white space between terms {} and {}", left, right),
        help: None,
    }

    /// For when the parser encountered an unexpected list of tokens.
    @formatted
    unexpected {
        args: (got: impl Display, expected: impl Display),
        msg: format!("expected {} -- got '{}'", expected, got),
        help: None,
    }

    /// For when the parser encountered a mix of commas and semi-colons in circuit member variables.
    @formatted
    mixed_commas_and_semicolons {
        args: (),
        msg: "Cannot mix use of commas and semi-colons for circuit member variable declarations.",
        help: None,
    }

    /// For when the parser encountered an unexpected identifier.
    @formatted
    unexpected_ident {
        args: (got: impl Display, expected: &[impl Display]),
        msg: format!(
            "unexpected identifier: expected {} -- got '{}'",
            expected
                .iter()
                .map(|x| format!("'{}'", x))
                .collect::<Vec<_>>()
                .join(", "),
            got
        ),
        help: None,
    }

    /// For when the parser encountered an unexpected statement.
    @formatted
    unexpected_statement {
        args: (got: impl Display, expected: impl Display),
        msg: format!("unexpected statement: expected '{}', got '{}'", expected, got),
        help: None,
    }

    /// For when the parser encountered an unexpected string.
    @formatted
    unexpected_str {
        args: (got: impl Display, expected: impl Display),
        msg: format!("unexpected string: expected '{}', got '{}'", expected, got),
        help: None,
    }

    /// For when the parser encountered an unexpected spread in an array init expression.
    @formatted
    spread_in_array_init {
        args: (),
        msg: "illegal spread in array initializer",
        help: None,
    }

    /// For when the parser encountered an invalid assignment target.
    @formatted
    invalid_assignment_target {
        args: (),
        msg: "invalid assignment target",
        help: None,
    }

    /// For when the parser encountered an invalid package name.
    @formatted
    invalid_package_name {
        args: (),
        msg: "package names must be lowercase alphanumeric ascii with underscores and singular dashes",
        help: None,
    }

    /// For when the parser encountered an illegal `const self` argument.
    @formatted
    illegal_self_const {
        args: (),
        msg: "cannot have const self",
        help: None,
    }

    /// For when the parser encountered a deprecated `mut` argument in a function.
    @formatted
    mut_function_input {
        args: (),
        msg: "function func(mut a: u32) { ... } is deprecated. Passed variables are mutable by default.",
        help: None,
    }

    /// For when the parser encountered a deprecated `mut` argument in a let statement.
    @formatted
    let_mut_statement {
        args: (),
        msg: "let mut = ... is deprecated. `let` keyword implies mutabality by default.",
        help: None,
    }

    /// For when the parser encountered a deprecated `test function`.
    @formatted
    test_function {
        args: (),
        msg: "\"test function...\" is deprecated. Did you mean @test annotation?",
        help: None,
    }

    /// For when the parser encountered a deprecated `@context(...)` annotation.
    @formatted
    context_annotation {
        args: (),
        msg: "\"@context(...)\" is deprecated. Did you mean @test annotation?",
        help: None,
    }

    /// For when the parser failed to parse array dimensions.
    @formatted
    unable_to_parse_array_dimensions {
        args: (),
        msg: "unable to parse array dimensions",
        help: None,
    }

    /// For when the parser encountered a deprecated `mut self` parameter in a member function declaration.
    @formatted
    mut_self_parameter {
        args: (),
        msg: "`mut self` is no longer accepted. Use `&self` if you would like to pass in a mutable reference to `self`",
        help: None,
    }

    /// When a member const comes after a member variable.
    @formatted
    member_const_after_var {
        args: (),
        msg: "Member variables must come after member consts.",
        help: None,
    }

    /// When a member const comes after a member function.
    @formatted
    member_const_after_fun {
        args: (),
        msg: "Member functions must come after member consts.",
        help: None,
    }

    /// When a member variable comes after a member function.
    @formatted
    member_var_after_fun {
        args: (),
        msg: "Member functions must come after member variables.",
        help: None,
    }

    /// E.g., on `[u8; ()]`.
    @formatted
    array_tuple_dimensions_empty {
        args: (),
        msg: "Array dimensions specified as a tuple cannot be empty.",
        help: None,
    }

    /// When an empty input tendril was expected but not found.
    @backtraced
    lexer_empty_input_tendril {
        args: (),
        msg: "Expected more characters to lex but found none.",
        help: None,
    }

    /// When an integer is started with a leading zero.
    @backtraced
    lexer_eat_integer_leading_zero {
    args: (input: impl Display),
    msg: format!("Tried to eat integer but found a leading zero on `{}`.", input),
    help: None,
    }

    /// When an integer is started with a leading zero.
    @backtraced
    lexer_expected_valid_escaped_char {
    args: (input: impl Display),
    msg: format!("Expected a valid escape character but found `{}`.", input),
    help: None,
    }

    /// When a string is not properly closed.
    @backtraced
    lexer_string_not_closed {
    args: (input: impl Display),
    msg: format!("Expected a closed string but found `{}`.", input),
    help: None,
    }

    /// When a string is not properly closed.
    @backtraced
    lexer_char_not_closed {
    args: (input: impl Display),
    msg: format!("Expected a closed char but found `{}`.", input),
    help: None,
    }

    /// When a string is not properly closed.
    @backtraced
    lexer_invalid_char {
    args: (input: impl Display),
    msg: format!("Expected valid character but found `{}`.", input),
    help: None,
    }

    /// When a block comment is empty.
    @backtraced
    lexer_empty_block_comment {
    args: (),
    msg: "Empty block comment.",
    help: None,
    }

    /// When a block comment is not closed before end of file.
    @backtraced
    lexer_block_comment_does_not_close_before_eof {
    args: (input: impl Display),
    msg: format!("Block comment does not close with content: `{}`.", input),
    help: None,
    }

    /// When the lexer could not lex some text.
    @backtraced
    could_not_lex {
    args: (input: impl Display),
    msg: format!("Could not lex the following content: `{}`.", input),
    help: None,
    }

    /// When a escaped character was given more than one char to escape.
    @backtraced
    lexer_escaped_char_incorrect_length {
        args: (input: impl Display),
        msg: format!("Could not lex the following escaped char due to being given more than one char: `{}`.", input),
        help: None,
    }

    /// When a escape was given but no following character
    @backtraced
    lexer_unclosed_escaped_char {
        args: (),
        msg: "There was no escaped character following the escape char symbol `\\`.",
        help: None,
    }

    /// When a escaped hex was given more than two chars to escape.
    @backtraced
    lexer_escaped_hex_incorrect_length {
        args: (input: impl Display),
        msg: format!("Could not lex the following escaped hex due to being given more than two chars: `{}`.", input),
        help: None,
    }

    /// When a valid hex character was expected.
    @backtraced
    lexer_expected_valid_hex_char {
    args: (input: impl Display),
    msg: format!("Expected a valid hex character but found `{}`.", input),
    help: None,
    }

    /// When a escaped unicode char was given but no following closing symbol.
    @backtraced
    lexer_unclosed_escaped_unicode_char {
        args: (input: impl Display),
        msg: format!("There was no closing `}}` after a escaped unicode `{}`.", input),
        help: None,
    }

    /// When a escaped unicode char was given but it had an incorrect length.
    @backtraced
    lexer_invalid_escaped_unicode_length {
        args: (input: impl Display),
        msg: format!("The escaped unicode char `{}` is not within valid length of [1, 6].", input),
        help: None,
    }

    /// When a escaped unicode char was given but exceeded maximum value.
    @backtraced
    lexer_invalid_character_exceeded_max_value {
        args: (input: impl Display),
        msg: format!("The escaped unicode char `{}` is greater than 0x10FFFF.", input),
        help: None,
    }
    /// When a hex number is provided.
    @backtraced
    lexer_hex_number_provided {
        args: (input: impl Display),
        msg: format!("A hex number `{}..` was provided but hex is not allowed.", input),
        help: None,
    }

    /// When a function recieved a self argument outside the first argument.
    @backtraced
    parser_self_outside_first_argument {
        args: (),
        msg: "A function received a self argument as not the first argument.",
        help: None,
    }

    /// When a escaped unicode char was given but not opened.
    @backtraced
    lexer_unopened_escaped_unicode_char {
        args: (input: impl Display),
        msg: format!("There was no opening `{{` after starting an escaped unicode `{}`.", input),
        help: None,
    }

    /// When a escaped unicode char has an emoji in it.
    @backtraced
    lexer_emoji_inside_escaped_unicode_char {
        args: (input: impl Display),
        msg: format!("There was an emoji found in the escaped unicode character: `{}`.", input),
        help: None,
    }

    /// For when a user puts parens around a single defined variable.
    @formatted
    invalid_parens_around_single_variable {
        args: (),
        msg: "do not put parens around single variable names",
        help: None,
    }

    /// For when a user puts parens around a single defined variable.
    @formatted
    invalid_parens_around_single_array_dimension_size {
        args: (),
        msg: "do not put parens around single dimension array size",
        help: None,
    }

    /// For when a user puts parens around a single defined variable.
    @backtraced
    lexer_expected_but_found {
        args: (found: impl Display, expected: impl Display),
        msg: format!("Found the char `{}`, but expected `{}`", found, expected),
        help: None,
    }

);
