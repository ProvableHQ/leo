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

//! The tokenizer to convert Leo code text into tokens.
//!
//! This module contains the [`tokenize()`] method which breaks down string text into tokens,
//! separated by whitespace.

pub(crate) mod token;
use std::iter;

pub use self::token::KEYWORD_TOKENS;
pub(crate) use self::token::*;

pub(crate) mod lexer;
pub(crate) use self::lexer::*;

use leo_errors::{ParserError, Result};
use leo_span::{
    span::{BytePos, Pos},
    Span,
};

/// Creates a new vector of spanned tokens from a given file path and source code text.
pub(crate) fn tokenize(input: &str, start_pos: BytePos) -> Result<Vec<SpannedToken>> {
    tokenize_iter(input, start_pos).collect()
}

/// Yields spanned tokens from the given source code text.
///
/// The `lo` byte position determines where spans will start.
pub(crate) fn tokenize_iter(input: &str, mut lo: BytePos) -> impl '_ + Iterator<Item = Result<SpannedToken>> {
    let mut index = 0usize;
    iter::from_fn(move || {
        while input.len() > index {
            let (token_len, token) = match Token::eat(&input[index..]) {
                Err(e) => return Some(Err(e)),
                Ok(t) => t,
            };
            index += token_len;

            let span = Span::new(lo, lo + BytePos::from_usize(token_len));
            lo = span.hi;

            match token {
                Token::WhiteSpace => continue,
                Token::AddressLit(address) if !check_address(&address) => {
                    return Some(Err(ParserError::invalid_address_lit(address, span).into()));
                }
                _ => return Some(Ok(SpannedToken { token, span })),
            }
        }

        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use leo_span::{source_map::FileName, symbol::create_session_if_not_set_then};

    #[test]
    fn test_tokenizer() {
        create_session_if_not_set_then(|s| {
            let raw = r#"
    'a'
    '😭'
    "test"
    "test{}test"
    "test{}"
    "{}test"
    "test{"
    "test}"
    "test{test"
    "test}test"
    "te{{}}"
    aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8
    test_ident
    12345
    address
    bool
    const
    else
    false
    field
    for
    function
    group
    i128
    i64
    i32
    i16
    i8
    if
    in
    input
    let
    mut
    return
    string
    test
    true
    u128
    u64
    u32
    u16
    u8
    console
    !
    !=
    &&
    (
    )
    *
    **
    +
    ,
    -
    ->
    _
    .
    ..
    /
    :
    ;
    <
    <=
    =
    ==
    >
    >=
    [
    ]
    {{
    }}
    ||
    ?
    // test
    /* test */
    //"#;
            let sf = s.source_map.new_source(raw, FileName::Custom("test".into()));
            let tokens = tokenize(&sf.src, sf.start_pos).unwrap();
            let mut output = String::new();
            for SpannedToken { token, .. } in tokens.iter() {
                output += &format!("{} ", token);
            }

            assert_eq!(
                output,
                r#"'a' '😭' "test" "test{}test" "test{}" "{}test" "test{" "test}" "test{test" "test}test" "te{{}}" aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8 test_ident 12345 address bool const else false field for function group i128 i64 i32 i16 i8 if in input let mut return string test true u128 u64 u32 u16 u8 console ! != && ( ) * ** + , - -> _ . .. / : ; < <= = == > >= [ ] { { } } || ? // test
 /* test */ // "#
            );
        });
    }

    #[test]
    fn test_spans() {
        create_session_if_not_set_then(|s| {
            let raw = r#"
ppp            test
            // test
            test
            /* test */
            test
            /* test
            test */
            test
            "#;

            let sm = &s.source_map;
            let sf = sm.new_source(raw, FileName::Custom("test".into()));
            let tokens = tokenize(&sf.src, sf.start_pos).unwrap();
            let mut line_indicies = vec![0];
            for (i, c) in raw.chars().enumerate() {
                if c == '\n' {
                    line_indicies.push(i + 1);
                }
            }
            for token in tokens.iter() {
                assert_eq!(token.token.to_string(), sm.contents_of_span(token.span).unwrap());
            }
        })
    }
}
