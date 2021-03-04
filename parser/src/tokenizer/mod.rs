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
//! This module contains the [`tokenize()`] method.

pub(crate) mod token;
pub(crate) use self::token::*;

pub(crate) mod tokenizer;
pub(crate) use self::tokenizer::*;

use crate::TokenError;
use leo_ast::Span;

use std::rc::Rc;

/// Creates a new vector of spanned tokens from a given file path and source code text.
pub(crate) fn tokenize(input: &str, path: &str) -> Result<Vec<SpannedToken>, TokenError> {
    let path = Rc::new(path.to_string());
    let mut input = input.as_bytes();
    let mut tokens = vec![];
    let mut index = 0usize;
    let mut line_no = 1usize;
    let mut line_start = 0usize;
    while !input.is_empty() {
        match Token::gobble(input) {
            (output, Some(token)) => {
                let span = Span {
                    line_start: line_no,
                    line_stop: line_no,
                    col_start: index - line_start + 1,
                    col_stop: index - line_start + (input.len() - output.len()) + 1,
                    path: path.clone(),
                };
                if let Token::AddressLit(address) = &token {
                    if !validate_address(address) {
                        return Err(TokenError::invalid_address_lit(address, &span));
                    }
                }
                tokens.push(SpannedToken { token, span });
                index += input.len() - output.len();
                input = output;
            }
            (output, None) => {
                if output.is_empty() {
                    break;
                } else if output.len() == input.len() {
                    return Err(TokenError::unexpected_token(
                        &String::from_utf8_lossy(&[input[0]]),
                        &Span {
                            line_start: line_no,
                            line_stop: line_no,
                            col_start: index - line_start + 1,
                            col_stop: index - line_start + 2,
                            path,
                        },
                    ));
                }
                index += input.len() - output.len();
                if input[0] == b'\n' {
                    line_no += 1;
                    line_start = index;
                }
                input = output;
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
        let tokens = tokenize(
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
        &
        &=
        |
        |=
        ^
        ^=
        ~
        <<
        <<=
        >>
        >>=
        >>>
        >>>=
        %
        %=
        ||=
        &&=
        ?
        // test
        /* test */
        //"#,
            "test_path",
        )
        .unwrap();
        let mut output = String::new();
        for SpannedToken { token, .. } in tokens.iter() {
            output += &format!("{} ", &token.to_string());
        }
        assert_eq!(
            output,
            r#""test" "test{}test" "test{}" "{}test" "test{" "test}" "test{test" "test}test" "te{{}}" aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8 test_ident 12345 address as bool circuit const else false field for function group i128 i64 i32 i16 i8 if import in input let mut return static string test true u128 u64 u32 u16 u8 self Self console ! != && ( ) * ** **= *= + += , - -= -> . .. ... / /= : :: ; < <= = == > >= @ [ ] { { } } || & &= | |= ^ ^= ~ << <<= >> >>= >>> >>>= % %= ||= &&= ? // test
 /* test */  //
 "#
        );
    }
}
