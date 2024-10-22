// Copyright (C) 2019-2024 Aleo Systems Inc.
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
//! This module contains the [`tokenize()`] function, which breaks down string text into tokens,
//! optionally separated by whitespace.

pub(crate) mod token;

pub use self::token::KEYWORD_TOKENS;
pub(crate) use self::token::*;

pub(crate) mod lexer;
pub(crate) use self::lexer::*;

use leo_errors::Result;
use leo_span::span::{BytePos, Pos, Span};
use std::iter;

///CST Handler
/// Creates a new vector of spanned tokens from a given file path and source code text.
pub(crate) fn tokenize(input: &str, start_pos: BytePos) -> Result<Vec<SpannedToken>> {
    tokenize_iter(input, start_pos).collect()
}

/// Yields spanned tokens from the given source code text.
///
/// The `lo` byte position determines where spans will start.
pub(crate) fn tokenize_iter(mut input: &str, mut lo: BytePos) -> impl '_ + Iterator<Item = Result<SpannedToken>> {
    let mut prev_prev = Token::WhiteSpace;
    let mut prev = Token::WhiteSpace;
    iter::from_fn(move || {
        while !input.is_empty() {
            let (token_len, token) = match Token::eat(input) {
                Err(e) => return Some(Err(e)),
                Ok(t) => t,
            };
            input = &input[token_len..];

            let span = Span::new(lo, lo + BytePos::from_usize(token_len));
            lo = span.hi;
            match token {
                Token::WhiteSpace => continue,
                Token::NewLine => {
                    if prev != Token::NewLine {
                        prev_prev = prev.clone();
                        prev = token.clone();
                    }
                    continue;
                }
                Token::CommentBlock(string) => {
                    if matches!(prev, Token::Semicolon) {
                        return Some(Ok(SpannedToken { token: Token::_CommentBlock(string), span }));
                    }else if matches!(prev, Token::LeftCurly){
                        prev_prev = prev.clone();
                        prev = Token::CommentBlock(string.clone());
                        return Some(Ok(SpannedToken { token: Token::_CommentBlock(string), span }));
                    }else if matches!(prev, Token::Comma){
                        prev_prev = prev.clone();
                        prev = Token::CommentBlock(string.clone());
                        return Some(Ok(SpannedToken { token: Token::_CommentBlock(string), span }));
                    }else if matches!(prev, Token::NewLine){
                        if 
                            matches!(prev_prev, Token::CommentLine(_)) ||  
                            matches!(prev_prev, Token::CommentBlock(_)) ||
                            matches!(prev_prev, Token::Semicolon) ||  
                            matches!(prev_prev, Token::LeftCurly) ||
                            matches!(prev_prev, Token::RightCurly) ||
                            matches!(prev_prev, Token::Comma)
                        {
                            return Some(Ok(SpannedToken { token: Token::CommentBlock(string), span }));
                        }
                    }else if matches!(prev, Token::WhiteSpace) & matches!(prev_prev, Token::WhiteSpace){
                        return Some(Ok(SpannedToken { token: Token::CommentBlock(string), span }));
                    }else if matches!(prev, Token::CommentLine(_)){
                        return Some(Ok(SpannedToken { token: Token::CommentBlock(string), span }));
                    }
                    continue;
                }
                Token::CommentLine(string) => {
                    if matches!(prev, Token::Semicolon) {
                        prev_prev = prev.clone();
                        prev = Token::CommentLine(string.clone());
                        return Some(Ok(SpannedToken { token: Token::_CommentLine(string), span }));
                    }else if matches!(prev, Token::LeftCurly){
                        prev_prev = prev.clone();
                        prev = Token::CommentLine(string.clone());
                        return Some(Ok(SpannedToken { token: Token::_CommentLine(string), span }));
                    }else if matches!(prev, Token::Comma){
                        prev_prev = prev.clone();
                        prev = Token::CommentLine(string.clone());
                        return Some(Ok(SpannedToken { token: Token::_CommentLine(string), span }));
                    }else if matches!(prev, Token::NewLine){
                        if 
                            matches!(prev_prev, Token::CommentLine(_)) ||  
                            matches!(prev_prev, Token::CommentBlock(_)) ||
                            matches!(prev_prev, Token::Semicolon) ||  
                            matches!(prev_prev, Token::LeftCurly) ||
                            matches!(prev_prev, Token::RightCurly) ||
                            matches!(prev_prev, Token::Comma)
                        {
                            return Some(Ok(SpannedToken { token: Token::CommentLine(string), span }));
                        }
                    }else if matches!(prev, Token::WhiteSpace) & matches!(prev_prev, Token::WhiteSpace){
                        return Some(Ok(SpannedToken { token: Token::CommentLine(string), span }));
                    }else if matches!(prev, Token::CommentLine(_)){
                        return Some(Ok(SpannedToken { token: Token::CommentLine(string), span }));
                    }
                    continue;
                }
                _ => {
                    prev_prev = prev.clone();
                    prev = token.clone();
                    return Some(Ok(SpannedToken { token, span }));
                },
            }
        }

        None
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use leo_span::{source_map::FileName, symbol::create_session_if_not_set_then};
    use std::fmt::Write;

    #[test]
    fn test_tokenizer() {
        create_session_if_not_set_then(|s| {
            let raw = r#"
    "test"
    "test{}test"
    "test{}"
    "{}test"
    "test{"
    "test}"
    "test{test"
    "test}test"
    "te{{}}"
    test_ident
    12345
    address
    as
    assert
    assert_eq
    assert_neq
    async
    bool
    const
    else
    false
    field
    for
    function
    Future
    group
    i128
    i64
    i32
    i16
    i8
    if
    in
    inline
    input
    let
    mut
    private
    program
    public
    return
    scalar
    self
    signature
    string
    struct
    test
    transition
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
    =>
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
    @
    // test
    /* test */
    //"#;
            let sf = s.source_map.new_source(raw, FileName::Custom("test".into()));
            let tokens = tokenize(&sf.src, sf.start_pos).unwrap();
            let mut output = String::new();
            for SpannedToken { token, .. } in tokens.iter() {
                write!(output, "{token} ").expect("failed to write string");
            }

            assert_eq!(
                output,
                r#""test" "test{}test" "test{}" "{}test" "test{" "test}" "test{test" "test}test" "te{{}}" test_ident 12345 address as assert assert_eq assert_neq async bool const else false field for function Future group i128 i64 i32 i16 i8 if in inline input let mut private program public return scalar self signature string struct test transition true u128 u64 u32 u16 u8 console ! != && ( ) * ** + , - -> => _ . .. / : ; < <= = == > >= [ ] { { } } || ? @ // test
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
            let mut line_indices = vec![0];
            for (i, c) in raw.chars().enumerate() {
                if c == '\n' {
                    line_indices.push(i + 1);
                }
            }
            for token in tokens.iter() {
                assert_eq!(token.token.to_string(), sm.contents_of_span(token.span).unwrap());
            }
        })
    }
}
