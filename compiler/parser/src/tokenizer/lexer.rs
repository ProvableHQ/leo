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

use crate::tokenizer::{Char, Token};
use leo_errors::{ParserError, Result};
use leo_span::{Span, Symbol};

use serde::{Deserialize, Serialize};
use tendril::StrTendril;

use std::fmt;

///
/// Returns the length of the given `wanted` string if the string can be eaten, otherwise returns [`None`].
/// A string can be eaten if its bytes are at the front of the given `input` array.
///
fn eat(input: &[u8], wanted: &str) -> Option<usize> {
    let wanted = wanted.as_bytes();
    if input.len() < wanted.len() {
        return None;
    }
    if &input[0..wanted.len()] == wanted {
        return Some(wanted.len());
    }
    None
}

///
/// Returns a new `StrTendril` string if an identifier can be eaten, otherwise returns [`None`].
/// An identifier can be eaten if its bytes are at the front of the given `input_tendril` string.
///
fn eat_identifier(input_tendril: &StrTendril) -> Option<StrTendril> {
    if input_tendril.is_empty() {
        return None;
    }
    let input = input_tendril[..].as_bytes();

    if !input[0].is_ascii_alphabetic() {
        return None;
    }

    let mut i = 1usize;
    while i < input.len() {
        if !input[i].is_ascii_alphanumeric() && input[i] != b'_' {
            break;
        }
        i += 1;
    }
    Some(input_tendril.subtendril(0, i as u32))
}

impl Token {
    ///
    /// Returns a `char` if a character can be eaten, otherwise returns [`None`].
    ///
    fn eat_char(input_tendril: StrTendril, escaped: bool, hex: bool, unicode: bool) -> Option<Char> {
        if input_tendril.is_empty() {
            return None;
        }

        if escaped {
            let string = input_tendril.to_string();
            let escaped = &string[1..string.len()];

            if escaped.len() != 1 {
                return None;
            }

            if let Some(character) = escaped.chars().next() {
                return match character {
                    '0' => Some(Char::Scalar(0 as char)),
                    't' => Some(Char::Scalar(9 as char)),
                    'n' => Some(Char::Scalar(10 as char)),
                    'r' => Some(Char::Scalar(13 as char)),
                    '\"' => Some(Char::Scalar(34 as char)),
                    '\'' => Some(Char::Scalar(39 as char)),
                    '\\' => Some(Char::Scalar(92 as char)),
                    _ => None,
                };
            } else {
                return None;
            }
        }

        if hex {
            let string = input_tendril.to_string();
            let hex_string = &string[2..string.len()];

            if hex_string.len() != 2 {
                return None;
            }

            if let Ok(ascii_number) = u8::from_str_radix(hex_string, 16) {
                // According to RFC, we allow only values less than 128.
                if ascii_number > 127 {
                    return None;
                }

                return Some(Char::Scalar(ascii_number as char));
            }
        }

        if unicode {
            let string = input_tendril.to_string();
            if &string[string.len() - 1..] != "}" {
                return None;
            }

            let unicode_number = &string[3..string.len() - 1];
            let len = unicode_number.len();
            if !(1..=6).contains(&len) {
                return None;
            }

            if let Ok(hex) = u32::from_str_radix(unicode_number, 16) {
                if let Some(character) = std::char::from_u32(hex) {
                    // scalar
                    return Some(Char::Scalar(character));
                } else if hex <= 0x10FFFF {
                    return Some(Char::NonScalar(hex));
                }
            }
        }

        if input_tendril.to_string().chars().count() != 1 {
            return None;
        } else if let Some(character) = input_tendril.to_string().chars().next() {
            return Some(Char::Scalar(character));
        }

        None
    }

    ///
    /// Returns a tuple: [(integer length, integer token)] if an integer can be eaten, otherwise returns [`None`].
    /// An integer can be eaten if its bytes are at the front of the given `input_tendril` string.
    ///
    fn eat_integer(input_tendril: &StrTendril) -> Result<(usize, Token)> {
        if input_tendril.is_empty() {
            return Err(ParserError::lexer_empty_input_tendril().into());
        }
        let input = input_tendril[..].as_bytes();
        if !input[0].is_ascii_digit() {
            return Err(ParserError::lexer_eat_integer_leading_zero(String::from_utf8_lossy(input)).into());
        }
        let mut i = 1;
        let mut is_hex = false;
        while i < input.len() {
            if i == 1 && input[0] == b'0' && input[i] == b'x' {
                is_hex = true;
                i += 1;
                continue;
            }
            if is_hex {
                if !input[i].is_ascii_hexdigit() {
                    break;
                }
            } else if !input[i].is_ascii_digit() {
                break;
            }

            i += 1;
        }
        Ok((i, Token::Int(input_tendril.subtendril(0, i as u32))))
    }

    /// Returns the number of bytes in an emoji via a bit mask.
    fn utf8_byte_count(byte: u8) -> u8 {
        let mut mask = 0x80;
        let mut result = 0;
        while byte & mask > 0 {
            result += 1;
            mask >>= 1;
        }
        if result == 0 {
            1
        } else if result > 4 {
            4
        } else {
            result
        }
    }

    ///
    /// Returns a tuple: [(token length, token)] if the next token can be eaten, otherwise returns [`None`].
    /// The next token can be eaten if the bytes at the front of the given `input_tendril` string can be scanned into a token.
    ///
    pub(crate) fn eat(input_tendril: StrTendril) -> Result<(usize, Token)> {
        if input_tendril.is_empty() {
            return Err(ParserError::lexer_empty_input_tendril().into());
        }
        let input = input_tendril[..].as_bytes();
        match input[0] {
            x if x.is_ascii_whitespace() => return Ok((1, Token::WhiteSpace)),
            b'"' => {
                let mut i = 1;
                let mut len: u8 = 1;
                let mut start = 1;
                let mut in_escape = false;
                let mut escaped = false;
                let mut hex = false;
                let mut unicode = false;
                let mut end = false;
                let mut string = Vec::new();

                while i < input.len() {
                    // If it's an emoji get the length.
                    if input[i] & 0x80 > 0 {
                        len = Self::utf8_byte_count(input[i]);
                        i += (len as usize) - 1;
                    }

                    if !in_escape {
                        if input[i] == b'"' {
                            end = true;
                            break;
                        } else if input[i] == b'\\' {
                            in_escape = true;
                            start = i;
                            i += 1;
                            continue;
                        }
                    } else {
                        len += 1;

                        match input[i] {
                            b'x' => {
                                hex = true;
                            }
                            b'u' => {
                                unicode = true;
                            }
                            b'}' if unicode => {
                                in_escape = false;
                            }
                            _ if !hex && !unicode => {
                                escaped = true;
                                in_escape = false;
                            }
                            _ if hex && len == 4 => {
                                in_escape = false;
                            }
                            _ => {}
                        }
                    }

                    if !in_escape {
                        match Self::eat_char(
                            input_tendril.subtendril(start as u32, len as u32),
                            escaped,
                            hex,
                            unicode,
                        ) {
                            Some(character) => {
                                len = 1;
                                escaped = false;
                                hex = false;
                                unicode = false;
                                string.push(character.into());
                            }
                            None => {
                                return Err(ParserError::lexer_expected_valid_escaped_char(
                                    input_tendril.subtendril(start as u32, len as u32),
                                )
                                .into())
                            }
                        }
                    }

                    i += 1;

                    if !escaped && !hex && !unicode {
                        start = i;
                    }
                }

                if i == input.len() || !end {
                    return Err(ParserError::lexer_string_not_closed(String::from_utf8_lossy(&input[0..i])).into());
                }

                return Ok((i + 1, Token::StringLit(string)));
            }
            b'\'' => {
                let mut i = 1;
                let mut in_escape = false;
                let mut escaped = false;
                let mut hex = false;
                let mut unicode = false;
                let mut end = false;

                while i < input.len() {
                    if !in_escape {
                        if input[i] == b'\'' {
                            end = true;
                            break;
                        } else if input[i] == b'\\' {
                            in_escape = true;
                        }
                    } else {
                        if input[i] == b'x' {
                            hex = true;
                        } else if input[i] == b'u' {
                            if input[i + 1] == b'{' {
                                unicode = true;
                            } else {
                                return Err(ParserError::lexer_expected_valid_escaped_char(input[i]).into());
                            }
                        } else {
                            escaped = true;
                        }

                        in_escape = false;
                    }

                    i += 1;
                }

                if !end {
                    return Err(ParserError::lexer_char_not_closed(String::from_utf8_lossy(&input[0..i])).into());
                }

                return match Self::eat_char(input_tendril.subtendril(1, (i - 1) as u32), escaped, hex, unicode) {
                    Some(character) => Ok((i + 1, Token::CharLit(character))),
                    None => Err(ParserError::lexer_invalid_char(String::from_utf8_lossy(&input[0..i - 1])).into()),
                };
            }
            x if x.is_ascii_digit() => {
                return Self::eat_integer(&input_tendril);
            }
            b'!' => {
                if let Some(len) = eat(input, "!=") {
                    return Ok((len, Token::NotEq));
                }
                return Ok((1, Token::Not));
            }
            b'?' => {
                return Ok((1, Token::Question));
            }
            b'&' => {
                if let Some(len) = eat(input, "&&") {
                    return Ok((len, Token::And));
                }
                return Ok((1, Token::Ampersand));
            }
            b'(' => return Ok((1, Token::LeftParen)),
            b')' => return Ok((1, Token::RightParen)),
            b'_' => return Ok((1, Token::Underscore)),
            b'*' => {
                if let Some(len) = eat(input, "**") {
                    if let Some(inner_len) = eat(&input[len..], "=") {
                        return Ok((len + inner_len, Token::ExpEq));
                    }
                    return Ok((len, Token::Exp));
                } else if let Some(len) = eat(input, "*=") {
                    return Ok((len, Token::MulEq));
                }
                return Ok((1, Token::Mul));
            }
            b'+' => {
                if let Some(len) = eat(input, "+=") {
                    return Ok((len, Token::AddEq));
                }
                return Ok((1, Token::Add));
            }
            b',' => return Ok((1, Token::Comma)),
            b'-' => {
                if let Some(len) = eat(input, "->") {
                    return Ok((len, Token::Arrow));
                } else if let Some(len) = eat(input, "-=") {
                    return Ok((len, Token::MinusEq));
                }
                return Ok((1, Token::Minus));
            }
            b'.' => {
                if let Some(len) = eat(input, "...") {
                    return Ok((len, Token::DotDotDot));
                } else if let Some(len) = eat(input, "..") {
                    return Ok((len, Token::DotDot));
                }
                return Ok((1, Token::Dot));
            }
            b'/' => {
                if eat(input, "//").is_some() {
                    let eol = input.iter().position(|x| *x == b'\n');
                    let len = if let Some(eol) = eol { eol + 1 } else { input.len() };
                    return Ok((len, Token::CommentLine(input_tendril.subtendril(0, len as u32))));
                } else if eat(input, "/*").is_some() {
                    if input.is_empty() {
                        return Err(ParserError::lexer_empty_block_comment().into());
                    }
                    let eol = input.windows(2).skip(2).position(|x| x[0] == b'*' && x[1] == b'/');
                    let len = if let Some(eol) = eol {
                        eol + 4
                    } else {
                        return Err(ParserError::lexer_block_comment_does_not_close_before_eof(
                            String::from_utf8_lossy(&input[0..]),
                        )
                        .into());
                    };
                    return Ok((len, Token::CommentBlock(input_tendril.subtendril(0, len as u32))));
                } else if let Some(len) = eat(input, "/=") {
                    return Ok((len, Token::DivEq));
                }
                return Ok((1, Token::Div));
            }
            b':' => {
                if let Some(len) = eat(input, "::") {
                    return Ok((len, Token::DoubleColon));
                } else {
                    return Ok((1, Token::Colon));
                }
            }
            b';' => return Ok((1, Token::Semicolon)),
            b'<' => {
                if let Some(len) = eat(input, "<=") {
                    return Ok((len, Token::LtEq));
                }
                return Ok((1, Token::Lt));
            }
            b'>' => {
                if let Some(len) = eat(input, ">=") {
                    return Ok((len, Token::GtEq));
                }
                return Ok((1, Token::Gt));
            }
            b'=' => {
                if let Some(len) = eat(input, "==") {
                    return Ok((len, Token::Eq));
                }
                return Ok((1, Token::Assign));
            }
            b'@' => return Ok((1, Token::At)),
            b'[' => return Ok((1, Token::LeftSquare)),
            b']' => return Ok((1, Token::RightSquare)),
            b'{' => return Ok((1, Token::LeftCurly)),
            b'}' => return Ok((1, Token::RightCurly)),
            b'|' => {
                if let Some(len) = eat(input, "||") {
                    return Ok((len, Token::Or));
                }
            }
            _ => (),
        }
        if let Some(ident) = eat_identifier(&input_tendril) {
            return Ok((
                ident.len(),
                match &*ident {
                    x if x.starts_with("aleo1") => Token::AddressLit(ident),
                    "address" => Token::Address,
                    "as" => Token::As,
                    "bool" => Token::Bool,
                    "char" => Token::Char,
                    "circuit" => Token::Circuit,
                    "console" => Token::Console,
                    "const" => Token::Const,
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
                    "import" => Token::Import,
                    "in" => Token::In,
                    "input" => Token::Input,
                    "let" => Token::Let,
                    "mut" => Token::Mut,
                    "return" => Token::Return,
                    "Self" => Token::BigSelf,
                    "self" => Token::LittleSelf,
                    "static" => Token::Static,
                    "true" => Token::True,
                    "type" => Token::Type,
                    "u8" => Token::U8,
                    "u16" => Token::U16,
                    "u32" => Token::U32,
                    "u64" => Token::U64,
                    "u128" => Token::U128,
                    _ => Token::Ident(Symbol::intern(&ident)),
                },
            ));
        }

        Err(ParserError::could_not_lex(String::from_utf8_lossy(&input[0..])).into())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
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

///
/// Returns true if the given string looks like Aleo address.
/// This method DOES NOT check if the address is valid on-chain.
///
pub(crate) fn check_address(address: &str) -> bool {
    // "aleo1" (LOWERCASE_LETTER | ASCII_DIGIT){58}
    if !address.starts_with("aleo1") || address.len() != 63 {
        return false;
    }
    address
        .chars()
        .skip(5)
        .all(|x| x.is_ascii_lowercase() || x.is_ascii_digit())
}
