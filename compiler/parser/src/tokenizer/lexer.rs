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

use crate::tokenizer::Token;
use leo_errors::{ParserError, Result};
use leo_span::{Span, Symbol};
use snarkvm_dpc::{prelude::*, testnet2::Testnet2};

use serde::{Deserialize, Serialize};
use std::{
    fmt,
    iter::{from_fn, Peekable},
    str::FromStr,
};

/// Eat an identifier, that is, a string matching '[a-zA-Z][a-zA-Z\d_]*', if any.
fn eat_identifier(input: &mut Peekable<impl Iterator<Item = char>>) -> Option<String> {
    input.peek().filter(|c| c.is_ascii_alphabetic())?;
    Some(from_fn(|| input.next_if(|c| c.is_ascii_alphanumeric() || c == &'_')).collect())
}

/// Checks if a char is a Unicode Bidirectional Override code point
fn is_bidi_override(c: char) -> bool {
    let i = c as u32;
    (0x202A..=0x202E).contains(&i) || (0x2066..=0x2069).contains(&i)
}

impl Token {
    // todo: remove this unused code or reference https://github.com/Geal/nom/blob/main/examples/string.rs
    // // Eats the parts of the unicode character after \u.
    // fn eat_unicode_char(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<(usize, Char)> {
    //     let mut unicode = String::new();
    //     // Account for the chars '\' and 'u'.
    //     let mut len = 2;
    //
    //     if input.next_if_eq(&'{').is_some() {
    //         len += 1;
    //     } else if let Some(c) = input.next() {
    //         return Err(ParserError::lexer_unopened_escaped_unicode_char(c).into());
    //     } else {
    //         return Err(ParserError::lexer_empty_input_tendril().into());
    //     }
    //
    //     while let Some(c) = input.next_if(|c| c != &'}') {
    //         len += 1;
    //         unicode.push(c);
    //     }
    //
    //     if input.next_if_eq(&'}').is_some() {
    //         len += 1;
    //     } else {
    //         return Err(ParserError::lexer_unclosed_escaped_unicode_char(unicode).into());
    //     }
    //
    //     // Max of 6 digits.
    //     // Minimum of 1 digit.
    //     if unicode.len() > 6 || unicode.is_empty() {
    //         return Err(ParserError::lexer_invalid_escaped_unicode_length(unicode).into());
    //     }
    //
    //     if let Ok(hex) = u32::from_str_radix(&unicode, 16) {
    //         if let Some(character) = std::char::from_u32(hex) {
    //             Ok((len, Char::Scalar(character)))
    //         } else if hex <= 0x10FFFF {
    //             Ok((len, Char::NonScalar(hex)))
    //         } else {
    //             Err(ParserError::lexer_invalid_character_exceeded_max_value(unicode).into())
    //         }
    //     } else {
    //         Err(ParserError::lexer_expected_valid_hex_char(unicode).into())
    //     }
    // }

    // // Eats the parts of the hex character after \x.
    // fn eat_hex_char(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<(usize, Char)> {
    //     let mut hex = String::new();
    //     // Account for the chars '\' and 'x'.
    //     let mut len = 2;
    //
    //     // First hex character.
    //     if let Some(c) = input.next_if(|c| c != &'\'') {
    //         len += 1;
    //         hex.push(c);
    //     } else if let Some(c) = input.next() {
    //         return Err(ParserError::lexer_expected_valid_hex_char(c).into());
    //     } else {
    //         return Err(ParserError::lexer_empty_input_tendril().into());
    //     }
    //
    //     // Second hex character.
    //     if let Some(c) = input.next_if(|c| c != &'\'') {
    //         len += 1;
    //         hex.push(c);
    //     } else if let Some(c) = input.next() {
    //         return Err(ParserError::lexer_expected_valid_hex_char(c).into());
    //     } else {
    //         return Err(ParserError::lexer_empty_input_tendril().into());
    //     }
    //
    //     if let Ok(ascii_number) = u8::from_str_radix(&hex, 16) {
    //         // According to RFC, we allow only values less than 128.
    //         if ascii_number > 127 {
    //             return Err(ParserError::lexer_expected_valid_hex_char(hex).into());
    //         }
    //
    //         Ok((len, Char::Scalar(ascii_number as char)))
    //     } else {
    //         Err(ParserError::lexer_expected_valid_hex_char(hex).into())
    //     }
    // }

    // fn eat_escaped_char(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<(usize, Char)> {
    //     match input.next() {
    //         None => Err(ParserError::lexer_empty_input_tendril().into()),
    //         // Length of 2 to account the '\'.
    //         Some('0') => Ok((2, Char::Scalar(0 as char))),
    //         Some('t') => Ok((2, Char::Scalar(9 as char))),
    //         Some('n') => Ok((2, Char::Scalar(10 as char))),
    //         Some('r') => Ok((2, Char::Scalar(13 as char))),
    //         Some('\"') => Ok((2, Char::Scalar(34 as char))),
    //         Some('\'') => Ok((2, Char::Scalar(39 as char))),
    //         Some('\\') => Ok((2, Char::Scalar(92 as char))),
    //         Some('u') => Self::eat_unicode_char(input),
    //         Some('x') => Self::eat_hex_char(input),
    //         Some(c) => Err(ParserError::lexer_expected_valid_escaped_char(c).into()),
    //     }
    // }

    // /// Returns a `char` if a character can be eaten, otherwise returns [`None`].
    // fn eat_char(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<(usize, Char)> {
    //     match input.next() {
    //         None => Err(ParserError::lexer_empty_input_tendril().into()),
    //         Some('\\') => Self::eat_escaped_char(input),
    //         Some(c) => Ok((c.len_utf8(), Char::Scalar(c))),
    //     }
    // }

    /// Returns a tuple: [(integer length, integer token)] if an integer can be eaten, otherwise returns [`None`].
    /// An integer can be eaten if its bytes are at the front of the given `input` string.
    fn eat_integer(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<(usize, Token)> {
        if input.peek().is_none() {
            return Err(ParserError::lexer_empty_input().into());
        }

        let mut int = String::new();
        while let Some(c) = input.next_if(|c| c.is_ascii_digit()) {
            if c == '0' && matches!(input.peek(), Some('x')) {
                int.push(c);
                int.push(input.next().unwrap());
                return Err(ParserError::lexer_hex_number_provided(int).into());
            }

            int.push(c);
        }

        Ok((int.len(), Token::Int(int)))
    }

    /// Returns a tuple: [(token length, token)] if the next token can be eaten, otherwise returns [`None`].
    /// The next token can be eaten if the bytes at the front of the given `input` string can be scanned into a token.
    pub(crate) fn eat(input: &str) -> Result<(usize, Token)> {
        if input.is_empty() {
            return Err(ParserError::lexer_empty_input().into());
        }

        let mut input = input.chars().peekable();

        // Consumes a single character token.
        let single = |input: &mut Peekable<_>, token| {
            input.next();
            Ok((1, token))
        };
        // Consumes a character followed by `on` with `then` if found or `els` otherwise.
        let followed_by = |input: &mut Peekable<_>, on, then, els| {
            input.next();
            Ok(if input.next_if_eq(&on).is_some() {
                (2, then)
            } else {
                (1, els)
            })
        };
        // Consumes a character followed by `on_1`, `on_2` or none. Outputs case_1, case_2, or els.
        let three_cases = |input: &mut Peekable<_>, on_1, case_1, on_2, case_2, els| {
            input.next();
            Ok(if input.next_if_eq(&on_1).is_some() {
                (2, case_1)
            } else if input.next_if_eq(&on_2).is_some() {
                (2, case_2)
            } else {
                (1, els)
            })
        };

        match *input.peek().ok_or_else(ParserError::lexer_empty_input)? {
            x if x.is_ascii_whitespace() => return single(&mut input, Token::WhiteSpace),
            '"' => {
                let mut string = String::new();
                input.next();

                let mut ended = false;
                while let Some(c) = input.next() {
                    // Check for illegal characters.
                    if is_bidi_override(c) {
                        return Err(ParserError::lexer_bidi_override().into());
                    }

                    // Check for end string quotation mark.
                    if c == '"' {
                        input.next();
                        ended = true;
                        break;
                    }
                    string.push(c);
                }

                if !ended {
                    return Err(ParserError::lexer_string_not_closed(string).into());
                }

                // + 2 to account for parsing quotation marks.
                return Ok((string.len() + 2, Token::StaticString(string)));
            }
            x if x.is_ascii_digit() => return Self::eat_integer(&mut input),
            '!' => return followed_by(&mut input, '=', Token::NotEq, Token::Not),
            '?' => return single(&mut input, Token::Question),
            '&' => return followed_by(&mut input, '&', Token::And, Token::BitwiseAnd),
            '(' => return single(&mut input, Token::LeftParen),
            ')' => return single(&mut input, Token::RightParen),
            '_' => return single(&mut input, Token::Underscore),
            '*' => return followed_by(&mut input, '*', Token::Exp, Token::Mul),
            '+' => return single(&mut input, Token::Add),
            ',' => return single(&mut input, Token::Comma),
            '-' => return followed_by(&mut input, '>', Token::Arrow, Token::Minus),
            '.' => return followed_by(&mut input, '.', Token::DotDot, Token::Dot),
            '/' => {
                input.next();
                if input.next_if_eq(&'/').is_some() {
                    let mut comment = String::from("//");

                    while let Some(c) = input.next_if(|c| c != &'\n') {
                        if is_bidi_override(c) {
                            return Err(ParserError::lexer_bidi_override().into());
                        }
                        comment.push(c);
                    }

                    if let Some(newline) = input.next_if_eq(&'\n') {
                        comment.push(newline);
                        return Ok((comment.len(), Token::CommentLine(comment)));
                    }

                    return Ok((comment.len(), Token::CommentLine(comment)));
                } else if input.next_if_eq(&'*').is_some() {
                    let mut comment = String::from("/*");

                    if input.peek().is_none() {
                        return Err(ParserError::lexer_empty_block_comment().into());
                    }

                    let mut ended = false;
                    while let Some(c) = input.next() {
                        if is_bidi_override(c) {
                            return Err(ParserError::lexer_bidi_override().into());
                        }
                        comment.push(c);
                        if c == '*' && input.next_if_eq(&'/').is_some() {
                            comment.push('/');
                            ended = true;
                            break;
                        }
                    }

                    if !ended {
                        return Err(ParserError::lexer_block_comment_does_not_close_before_eof(comment).into());
                    }
                    return Ok((comment.len(), Token::CommentBlock(comment)));
                }
                return Ok((1, Token::Div));
            }
            ':' => return followed_by(&mut input, ':', Token::DoubleColon, Token::Colon),
            ';' => return single(&mut input, Token::Semicolon),
            '<' => return three_cases(&mut input, '=', Token::LtEq, '<', Token::Shl, Token::Lt),
            '>' => return three_cases(&mut input, '=', Token::GtEq, '>', Token::Shr, Token::Gt),
            '=' => return followed_by(&mut input, '=', Token::Eq, Token::Assign),
            '[' => return single(&mut input, Token::LeftSquare),
            ']' => return single(&mut input, Token::RightSquare),
            '{' => return single(&mut input, Token::LeftCurly),
            '}' => return single(&mut input, Token::RightCurly),
            '|' => return followed_by(&mut input, '|', Token::Or, Token::BitwiseOr),
            '^' => return single(&mut input, Token::Xor),
            _ => (),
        }
        if let Some(ident) = eat_identifier(&mut input) {
            return Ok((
                ident.len(),
                // todo: match on symbols instead of hard-coded &str's
                match &*ident {
                    x if x.starts_with("aleo1") => Token::AddressLit(ident),
                    "address" => Token::Address,
                    "bool" => Token::Bool,
                    "circuit" => Token::Circuit,
                    "console" => Token::Console,
                    "const" => Token::Const,
                    "constant" => Token::Constant,
                    "else" => Token::Else,
                    "false" => Token::False,
                    "field" => Token::Field,
                    "for" => Token::For,
                    "function" => Token::Function,
                    "group" => Token::Group,
                    "i8" => Token::I8,
                    "i16" => Token::I16,
                    "i32" => Token::I32,
                    "i64" => Token::I64,
                    "i128" => Token::I128,
                    "if" => Token::If,
                    "in" => Token::In,
                    "let" => Token::Let,
                    "public" => Token::Public,
                    "return" => Token::Return,
                    "scalar" => Token::Scalar,
                    "string" => Token::String,
                    "true" => Token::True,
                    "u8" => Token::U8,
                    "u16" => Token::U16,
                    "u32" => Token::U32,
                    "u64" => Token::U64,
                    "u128" => Token::U128,
                    _ => Token::Ident(Symbol::intern(&ident)),
                },
            ));
        }

        Err(ParserError::could_not_lex(
            input
                .take_while(|c| *c != ';' && !c.is_whitespace())
                .collect::<String>(),
        )
        .into())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

impl SpannedToken {
    /// Returns a dummy token at a dummy span.
    pub const fn dummy() -> Self {
        Self {
            token: Token::Question,
            span: Span::dummy(),
        }
    }
}

impl fmt::Display for SpannedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}' @ ", self.token.to_string().trim())?;
        self.span.fmt(f)
    }
}

impl fmt::Debug for SpannedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <SpannedToken as fmt::Display>::fmt(self, f)
    }
}

/// Returns true if the given string is a valid Aleo address.
pub(crate) fn check_address(address: &str) -> bool {
    Address::<Testnet2>::from_str(address).is_ok()
}
