// Copyright (C) 2019-2026 Provable Inc.
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

use std::fmt::Display;

use leo_errors::{Backtraced, Formatted};

const CODE_PREFIX: &str = "PAR";
const CODE_MASK: i32 = 0;

pub(crate) fn invalid_address_lit(token: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 1, format!("invalid address literal: '{token}'"), span)
}

pub(crate) fn unexpected_eof(span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 3, "unexpected EOF", span)
}

pub(crate) fn unexpected(found: impl Display, expected: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 5, format!("expected {expected} -- found '{found}'"), span)
}

pub(crate) fn unexpected_str(found: impl Display, expected: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 9,
        format!("unexpected string: expected '{expected}', found '{found}'"),
        span,
    )
}

pub(crate) fn invalid_method_call(
    expr: impl Display,
    func: impl Display,
    num_args: impl Display,
    span: leo_span::Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 21,
        format!("The type of `{expr}` has no associated function `{func}` that takes {num_args} argument(s)."),
        span,
    )
}

pub(crate) fn missing_program_declaration(span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 27, "Missing a program scope in a Leo file.", span)
        .with_help("Add a program scope of the form: `program <name>.aleo { ... }` to the Leo file.")
}

pub(crate) fn invalid_network(span: leo_span::Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 28,
        "Invalid network identifier. The only supported identifier is `.aleo`.",
        span,
    )
}

pub(crate) fn tuple_must_have_at_least_two_elements(kind: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 29, format!("A tuple {kind} must have at least two elements."), span)
}

pub(crate) fn hexbin_literal_nonintegers(span: leo_span::Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 42,
        "Hex, octal, and binary literals may only be used for integer types.",
        span,
    )
}

pub(crate) fn identifier_too_long(
    ident: impl Display,
    length: usize,
    max_length: usize,
    span: leo_span::Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 44,
        format!("Identifier {ident} is too long ({length} bytes; maximum is {max_length})"),
        span,
    )
}

pub(crate) fn identifier_cannot_contain_double_underscore(ident: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 46,
        format!("Identifier {ident} cannot contain a double underscore `__`"),
        span,
    )
}

pub(crate) fn custom(msg: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 47, format!("{msg}"), span)
}

pub(crate) fn keyword_used_as_module_name(module_name: impl Display, keyword: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 49,
        format!("Module `{module_name}` uses the reserved keyword `{keyword}` as a name"),
    )
    .with_help(format!("Rename the module so it does not conflict with the language keyword `{keyword}`."))
}

pub(crate) fn could_not_lex_span(input: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 50, format!("Could not lex the following content: `{input}`.\n"), span)
}

pub(crate) fn lexer_bidi_override_span(span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 51, "Unicode bidi override code point encountered.", span)
}

pub(crate) fn wrong_digit_for_radix_span(
    digit: char,
    radix: u32,
    token: impl Display,
    span: leo_span::Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 52,
        format!("Digit {digit} invalid in radix {radix} (token {token})."),
        span,
    )
}

pub(crate) fn identifier_cannot_start_with_underscore(span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 53, "Identifiers cannot start with an underscore.", span)
        .with_help("Identifiers must start with a letter.")
}

pub(crate) fn multiple_program_declarations(span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 55, "A Leo program can only have one `program` declaration.", span)
        .with_help("Remove the duplicate `program` block. Only one is allowed per program.")
}

// Parser warnings

pub(crate) fn record_prototype_redundant(record_name: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 2,
        format!(
            "Record prototype `{record_name}` does not constrain any fields beyond the implicit `owner: address`. \
             Consider simplifying to `record {record_name};`."
        ),
        span,
    )
}
