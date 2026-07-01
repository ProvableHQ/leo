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
    Formatted::error(CODE_PREFIX, CODE_MASK + 1, format!("invalid address literal: `{token}`"), span).with_help(
        "Aleo addresses are bech32-encoded and start with `aleo1`, e.g. `aleo1qg…`. Check the address for typos.",
    )
}

pub(crate) fn unexpected_eof(span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 3, "unexpected end of file", span)
        .with_help("The parser reached the end of the file while still expecting more input. Complete the current item. Common causes are a missing `}`, `)`, or `;`.")
}

pub(crate) fn unexpected(found: impl Display, expected: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 5, format!("expected {expected}, found `{found}`"), span).with_help(
        "Replace the highlighted token with what the parser expects, or insert the missing syntax before it.",
    )
}

pub(crate) fn unexpected_str(found: impl Display, expected: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 9, format!("expected `{expected}`, found `{found}`"), span)
        .with_help(format!("Replace `{found}` with `{expected}`."))
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
        format!("the type of `{expr}` has no associated function `{func}` that takes {num_args} argument(s)"),
        span,
    )
    .with_help("Check the method name for typos and confirm the argument count matches the function's signature.")
}

pub(crate) fn missing_program_declaration(span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 27, "missing a program scope in a Leo file", span)
        .with_help("Add a program scope of the form `program <name>.aleo { ... }` to the Leo file.")
}

pub(crate) fn invalid_network(span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 28, "invalid network identifier", span)
        .with_help("The only supported network identifier is `.aleo`. Replace the network suffix accordingly.")
}

pub(crate) fn tuple_must_have_at_least_two_elements(kind: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 29, format!("a tuple {kind} must have at least two elements"), span)
        .with_help("Add another element, or drop the tuple syntax entirely if only one value is needed.")
}

pub(crate) fn hexbin_literal_nonintegers(span: leo_span::Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 42,
        "hex, octal, and binary literals may only be used for integer types",
        span,
    )
    .with_help("Use a decimal literal for non-integer types, e.g. `1field` or `1group`.")
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
        format!("identifier `{ident}` is too long ({length} bytes; maximum is {max_length})"),
        span,
    )
    .with_help(format!("Shorten the identifier to at most {max_length} bytes."))
}

pub(crate) fn identifier_cannot_contain_double_underscore(ident: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 46,
        format!("identifier `{ident}` cannot contain a double underscore `__`"),
        span,
    )
    .with_help("Rename the identifier so it uses at most single underscores between segments.")
}

pub(crate) fn custom(msg: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 47, format!("{msg}"), span)
}

pub(crate) fn keyword_used_as_module_name(module_name: impl Display, keyword: impl Display) -> Backtraced {
    Backtraced::error(
        CODE_PREFIX,
        CODE_MASK + 49,
        format!("module `{module_name}` uses the reserved keyword `{keyword}` as a name"),
    )
    .with_help(format!("Rename the module so it does not conflict with the language keyword `{keyword}`."))
}

pub(crate) fn could_not_lex_span(input: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 50, format!("could not tokenize the following content: `{input}`"), span)
        .with_help(
            "Remove or replace the unrecognized characters. Only ASCII source is supported outside of string literals.",
        )
}

pub(crate) fn lexer_bidi_override_span(span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 51, "unicode bidirectional override code point encountered", span)
        .with_note(
            "Bidi override characters can disguise source code and are rejected to prevent \"trojan source\" attacks.",
        )
        .with_help("Remove the bidi override character from the source.")
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
        format!("digit `{digit}` is invalid in radix {radix} (token `{token}`)"),
        span,
    )
    .with_help(format!(
        "Use only digits valid in base-{radix} (binary `0b…` accepts `0`–`1`, octal `0o…` accepts `0`–`7`, hex `0x…` accepts `0`–`9` and `a`–`f`)."
    ))
}

pub(crate) fn identifier_cannot_start_with_underscore(span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 53, "identifiers cannot start with an underscore", span)
        .with_help("Rename the identifier so it begins with an ASCII letter.")
}

pub(crate) fn multiple_program_declarations(span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 55, "a Leo program can only have one `program` declaration", span)
        .with_help("Remove the duplicate `program` block. Only one is allowed per program.")
}

pub(crate) fn obsolete_context_access(old: impl Display, replacement: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 56, format!("the `{old}` syntax has been removed"), span).with_help(
        format!("Use `{replacement}` instead. Execution-context accessors now live in the `std::ctx` module."),
    )
}

pub(crate) fn obsolete_context_keyword(keyword: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 57, format!("`{keyword}` is no longer a valid expression"), span)
        .with_help(format!(
            "`{keyword}` is reserved and no longer denotes an execution-context value. Use functions from the `std::ctx` module to access caller, signer, block, or network information."
        ))
}

pub(crate) fn reserved_identifier(name: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 58, format!("`{name}` is reserved for future use"), span)
        .with_help(format!("Rename this identifier. `{name}` is reserved by the language for an upcoming feature."))
}

// Parser warnings

pub(crate) fn record_prototype_redundant(record_name: impl Display, span: leo_span::Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 2,
        format!("record prototype `{record_name}` does not constrain any fields beyond the implicit `owner: address`"),
        span,
    )
    .with_help(format!("Simplify the declaration to `record {record_name};`."))
}
