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

//! The tokenizer to convert Leo code text into tokens.
//!
//! This module contains the [`tokenize()`] method which breaks down string text into tokens,
//! separated by whitespace.

pub(crate) mod token;
use std::sync::Arc;

pub(crate) use self::token::*;

pub(crate) mod lexer;
pub(crate) use self::lexer::*;

use crate::TokenError;
use leo_ast::Span;
use tendril::StrTendril;

/// Creates a new vector of spanned tokens from a given file path and source code text.
pub(crate) fn tokenize(path: &str, input: StrTendril) -> Result<Vec<SpannedToken>, TokenError> {
    let path = Arc::new(path.to_string());
    let mut tokens = vec![];
    let mut index = 0usize;
    let mut line_no = 1usize;
    let mut line_start = 0usize;
    while input.len() > index {
        match Token::eat(input.subtendril(index as u32, (input.len() - index) as u32)) {
            (token_len, Some(token)) => {
                let mut span = Span {
                    line_start: line_no,
                    line_stop: line_no,
                    col_start: index - line_start + 1,
                    col_stop: index - line_start + token_len + 1,
                    path: path.clone(),
                    content: input.subtendril(
                        line_start as u32,
                        input[line_start..].find('\n').unwrap_or(input.len() - line_start) as u32,
                    ),
                };
                match &token {
                    Token::CommentLine(_) => {
                        line_no += 1;
                        line_start = index + token_len;
                    }
                    Token::CommentBlock(block) => {
                        let line_ct = block.chars().filter(|x| *x == '\n').count();
                        line_no += line_ct;
                        if line_ct > 0 {
                            let last_line_index = block.rfind('\n').unwrap();
                            line_start = index + last_line_index + 1;
                            span.col_stop = index + token_len - line_start + 1;
                        }
                        span.line_stop = line_no;
                    }
                    Token::AddressLit(address) => {
                        if !check_address(address) {
                            return Err(TokenError::invalid_address_lit(address, &span));
                        }
                    }
                    _ => (),
                }
                tokens.push(SpannedToken { token, span });
                index += token_len;
            }
            (token_len, None) => {
                if token_len == 0 && index == input.len() {
                    break;
                } else if token_len == 0 {
                    return Err(TokenError::unexpected_token(&input[index..index + 1], &Span {
                        line_start: line_no,
                        line_stop: line_no,
                        col_start: index - line_start + 1,
                        col_stop: index - line_start + 2,
                        path,
                        content: input.subtendril(
                            line_start as u32,
                            input[line_start..].find('\n').unwrap_or_else(|| input.len()) as u32,
                        ),
                    }));
                }
                if input.as_bytes()[index] == b'\n' {
                    line_no += 1;
                    line_start = index + token_len;
                }
                index += token_len;
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        // &
        // &=
        // |
        // |=
        // ^
        // ^=
        // ~
        // <<
        // <<=
        // >>
        // >>=
        // >>>
        // >>>=
        // %
        // %=
        // ||=
        // &&=

        let tokens = tokenize(
            "test_path",
            r#"
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
        as
        bool
        circuit
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
        import
        in
        input
        let
        mut
        return
        static
        string
        test
        true
        u128
        u64
        u32
        u16
        u8
        self
        Self
        console
        !
        !=
        &&
        (
        )
        *
        **
        **=
        *=
        +
        +=
        ,
        -
        -=
        ->
        _
        .
        ..
        ...
        /
        /=
        :
        ::
        ;
        <
        <=
        =
        ==
        >
        >=
        @
        [
        ]
        {{
        }}
        ||
        ?
        // test
        /* test */
        //"#
            .into(),
        )
        .unwrap();
        let mut output = String::new();
        for SpannedToken { token, .. } in tokens.iter() {
            output += &format!("{} ", token.to_string());
        }
        // & &= | |= ^ ^= ~ << <<= >> >>= >>> >>>= % %= ||= &&=
        assert_eq!(
            output,
            r#""test" "test{}test" "test{}" "{}test" "test{" "test}" "test{test" "test}test" "te{{}}" aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8 test_ident 12345 address as bool circuit const else false field for function group i128 i64 i32 i16 i8 if import in input let mut return static string test true u128 u64 u32 u16 u8 self Self console ! != && ( ) * ** **= *= + += , - -= -> _ . .. ... / /= : :: ; < <= = == > >= @ [ ] { { } } || ? // test
 /* test */ // "#
        );
    }

    #[test]
    fn test_spans() {
        let raw = r#"
            test
            // test
            test
            /* test */
            test
            /* test
            test */
            test
            "#;
        let tokens = tokenize("test_path", raw.into()).unwrap();
        let mut line_indicies = vec![0];
        for (i, c) in raw.chars().enumerate() {
            if c == '\n' {
                line_indicies.push(i + 1);
            }
        }
        for token in tokens.iter() {
            let token_raw = token.token.to_string();
            let start = line_indicies.get(token.span.line_start - 1).unwrap();
            let stop = line_indicies.get(token.span.line_stop - 1).unwrap();
            let original = &raw[*start + token.span.col_start - 1..*stop + token.span.col_stop - 1];
            assert_eq!(original, &token_raw);
        }
        // println!("{}", serde_json::to_string_pretty(&tokens).unwrap());
    }
}
