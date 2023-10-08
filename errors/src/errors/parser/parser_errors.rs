// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::create_messages;

use std::fmt::{Debug, Display};

create_messages!(
    /// ParserError enum that represents all the errors for the `leo-parser` crate.
    ParserError,
    code_mask: 0000i32,
    code_prefix: "PAR",

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
        msg: format!("invalid address literal: '{token}'"),
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
        msg: format!("Unexpected white space between terms {left} and {right}"),
        help: None,
    }

    /// For when the parser encountered an unexpected list of tokens.
    @formatted
    unexpected {
        args: (found: impl Display, expected: impl Display),
        msg: format!("expected {expected} -- found '{found}'"),
        help: None,
    }

    /// For when the parser encountered a mix of commas and semi-colons in struct member variables.
    @formatted
    mixed_commas_and_semicolons {
        args: (),
        msg: "Cannot mix use of commas and semi-colons for struct member variable declarations.",
        help: None,
    }

    /// For when the parser encountered an unexpected identifier.
    @formatted
    unexpected_ident {
        args: (found: impl Display, expected: &[impl Display]),
        msg: format!(
            "unexpected identifier: expected {} -- found '{found}'",
            expected
                .iter()
                .map(|x| format!("'{x}'"))
                .collect::<Vec<_>>()
                .join(", "),
        ),
        help: None,
    }

    /// For when the parser encountered an unexpected statement.
    @formatted
    unexpected_statement {
        args: (found: impl Display, expected: impl Display),
        msg: format!("unexpected statement: expected '{expected}', found '{found}'"),
        help: None,
    }

    /// For when the parser encountered an unexpected string.
    @formatted
    unexpected_str {
        args: (found: impl Display, expected: impl Display),
        msg: format!("unexpected string: expected '{expected}', found '{found}'"),
        help: None,
    }

    /// For when the parser encountered an unexpected spread in an array init expression.
    @formatted
    spread_in_array_init {
        args: (),
        msg: "illegal spread in array initializer",
        help: None,
    }

    /// When more input was expected but not found.
    @backtraced
    lexer_empty_input {
        args: (),
        msg: "Expected more characters to lex but found none.",
        help: None,
    }

    /// When an integer is started with a leading zero.
    @backtraced
    lexer_expected_valid_escaped_char {
    args: (input: impl Display),
    msg: format!("Expected a valid escape character but found `{input}`."),
    help: None,
    }

    /// When a string is not properly closed.
    @backtraced
    lexer_string_not_closed {
    args: (input: impl Display),
    msg: format!("Expected a closed string but found `{input}`."),
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
    msg: format!("Block comment does not close with content: `{input}`."),
    help: None,
    }

    /// When the lexer could not lex some text.
    @backtraced
    could_not_lex {
    args: (input: impl Display),
    msg: format!("Could not lex the following content: `{input}`.\n"),
    help: None,
    }

    /// When the user tries to pass an implicit value.
    @formatted
    implicit_values_not_allowed {
        args: (input: impl Display),
        msg: format!("Could not parse the implicit value: {input}."),
        help: None,
    }

    /// When a hex number is provided.
    @backtraced
    lexer_hex_number_provided {
        args: (input: impl Display),
        msg: format!("A hex number `{input}..` was provided but hex is not allowed."),
        help: None,
    }

    /// For when a user specified more than one mode on a parameter.
    @formatted
    inputs_multiple_variable_modes_specified {
        args: (),
        msg: "A parameter cannot have multiple modes.",
        help: Some("Consider using either `constant`, `public`, `private`, or none at all.".to_string()),
    }

    /// For when the lexer encountered a bidi override character
    @backtraced
    lexer_bidi_override {
        args: (),
        msg: "Unicode bidi override code point encountered.",
        help: None,
    }

    /// Parsed an unknown method call on the type of an expression.
    @formatted
    invalid_method_call {
        args: (expr: impl Display, func: impl Display, num_args: impl Display),
        msg: format!("The type of `{expr}` has no associated function `{func}` that takes {num_args} argument(s)."),
        help: None,
    }

    @formatted
    invalid_associated_access {
        args: (name: impl Display),
        msg: format!("Invalid associated access call to struct {name}."),
        help: Some("Double colon `::` syntax is only supported for core functions in Leo for testnet3.".to_string()),
    }

    @formatted
    leo_imports_only {
        args: (),
        msg: "Invalid import call to non-leo file.",
        help: Some("Only imports of Leo `.leo` files are currently supported.".to_string()),
    }

    @formatted
    space_in_annotation {
        args: (),
        msg: "Illegal spacing in the annotation declaration.",
        help: Some("Remove whitespace between the `@` symbol and the identifier.".to_string()),
    }

    @formatted
    circuit_is_deprecated {
        args: (),
        msg: "The keyword `circuit` is deprecated.",
        help: Some("Use `struct` instead.".to_string()),
    }

    @formatted
    only_one_program_scope_is_allowed {
        args: (),
        msg: "Only one program scope is allowed in a Leo file.",
        help: None,
    }

    @formatted
    missing_program_scope {
        args: (),
        msg: "Missing a program scope in a Leo file.",
        help: Some("Add a program scope of the form: `program <name>.aleo { ... }` to the Leo file.".to_string()),
    }

    @formatted
    invalid_network {
        args: (),
        msg: "Invalid network identifier. The only supported identifier is `aleo`.",
        help: None,
    }

    @formatted
    tuple_must_have_at_least_two_elements {
        args: (kind: impl Display),
        msg: format!("A tuple {kind} must have at least two elements."),
        help: None,
    }

    @formatted
    async_finalize_is_deprecated {
        args: (),
        msg: format!("`async finalize` is deprecated."),
        help: Some("Use `return <expr> then finalize(<args>)` instead.".to_string()),
    }

    @formatted
    finalize_statements_are_deprecated {
        args: (),
        msg: format!("`finalize` statements are deprecated."),
        help: Some("Use `return <expr> then finalize(<args>)` instead.".to_string()),
    }

    @formatted
    console_statements_are_not_yet_supported {
        args: (),
        msg: format!("`console` statements are not yet supported."),
        help: Some("Consider using `assert`, `assert_eq`, or `assert_neq` instead.".to_string()),
    }

    /// Enforce that tuple index must not have leading 0, or underscore in between digits
    @formatted
    tuple_index_must_be_whole_number {
        args: (found: impl Display),
        msg: format!("expected no underscores or leading zeros -- found '{found}'"),
        help: None,
    }

    @formatted
    array_must_have_at_least_one_element {
        args: (kind: impl Display),
        msg: format!("An array {kind} must have at least one element."),
        help: None,
    }
);
