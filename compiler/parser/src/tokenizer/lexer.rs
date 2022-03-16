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

use std::{fmt, iter::Peekable};

///
/// Returns a new `StrTendril` string if an identifier can be eaten, otherwise returns [`None`].
/// An identifier can be eaten if its bytes are at the front of the given `input_tendril` string.
///
fn eat_identifier(input: &mut Peekable<impl Iterator<Item = char>>) -> Option<String> {
    match input.peek() {
        None => return None,
        Some(c) if !c.is_ascii_alphabetic() => return None,
        _ => {}
    }

    let mut ident = String::new();
    while let Some(c) = input.next_if(|c| c.is_ascii_alphanumeric() || c == &'_') {
        ident.push(c);
    }
    Some(ident)
}

impl Token {
    fn eat_unicode_char(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<(usize, Char)> {
        let mut unicode = String::new();
        // Account for the chars '\' and 'u'.
        let mut len = 2;

        if input.next_if_eq(&'{').is_some() {
            len += 1;
        } else if let Some(c) = input.next() {
            return Err(ParserError::lexer_unopened_escaped_unicode_char(c).into());
        } else {
            return Err(ParserError::lexer_empty_input_tendril().into());
        }

        while let Some(c) = input.next_if(|c| c != &'}') {
            len += 1;
            unicode.push(c);
        }

        if input.next_if_eq(&'}').is_some() {
            len += 1;
        } else {
            return Err(ParserError::lexer_unclosed_escaped_unicode_char(unicode).into());
        }

        // Max of 6 digits.
        // Minimum of 1 digit.
        if unicode.len() > 6 || unicode.is_empty() {
            return Err(ParserError::lexer_invalid_escaped_unicode_length(unicode).into());
        }

        if let Ok(hex) = u32::from_str_radix(&unicode, 16) {
            if let Some(character) = std::char::from_u32(hex) {
                // scalar
                Ok((len, Char::Scalar(character)))
            } else if hex <= 0x10FFFF {
                Ok((len, Char::NonScalar(hex)))
            } else {
                Err(ParserError::lexer_invalid_character_exceeded_max_value(unicode).into())
            }
        } else {
            Err(ParserError::lexer_expected_valid_hex_char(unicode).into())
        }
    }

    fn eat_hex_char(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<(usize, Char)> {
        let mut hex = String::new();
        // Account for the chars '\' and 'x'.
        let mut len = 2;

        // First hex character.
        if let Some(c) = input.next_if(|c| c != &'\'') {
            len += 1;
            hex.push(c);
        } else if let Some(c) = input.next() {
            return Err(ParserError::lexer_expected_valid_hex_char(c).into());
        } else {
            return Err(ParserError::lexer_empty_input_tendril().into());
        }

        // Second hex character.
        if let Some(c) = input.next_if(|c| c != &'\'') {
            len += 1;
            hex.push(c);
        } else if let Some(c) = input.next() {
            return Err(ParserError::lexer_expected_valid_hex_char(c).into());
        } else {
            return Err(ParserError::lexer_empty_input_tendril().into());
        }

        if let Ok(ascii_number) = u8::from_str_radix(&hex, 16) {
            // According to RFC, we allow only values less than 128.
            if ascii_number > 127 {
                return Err(ParserError::lexer_expected_valid_hex_char(hex).into());
            }

            Ok((len, Char::Scalar(ascii_number as char)))
        } else {
            Err(ParserError::lexer_expected_valid_hex_char(hex).into())
        }
    }

    fn eat_escaped_char(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<(usize, Char)> {
        match input.next() {
            None => Err(ParserError::lexer_empty_input_tendril().into()),
            // Length of 2 to account the '\'.
            Some('0') => Ok((2, Char::Scalar(0 as char))),
            Some('t') => Ok((2, Char::Scalar(9 as char))),
            Some('n') => Ok((2, Char::Scalar(10 as char))),
            Some('r') => Ok((2, Char::Scalar(13 as char))),
            Some('\"') => Ok((2, Char::Scalar(34 as char))),
            Some('\'') => Ok((2, Char::Scalar(39 as char))),
            Some('\\') => Ok((2, Char::Scalar(92 as char))),
            Some('u') => Self::eat_unicode_char(input),
            Some('x') => Self::eat_hex_char(input),
            Some(c) => Err(ParserError::lexer_expected_valid_escaped_char(c).into()),
        }
    }

    ///
    /// Returns a `char` if a character can be eaten, otherwise returns [`None`].
    ///
    fn eat_char(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<(usize, Char)> {
        match input.next() {
            None => Err(ParserError::lexer_empty_input_tendril().into()),
            Some('\\') => Self::eat_escaped_char(input),
            Some(c) => Ok((c.len_utf8(), Char::Scalar(c))),
        }
    }

    ///
    /// Returns a tuple: [(integer length, integer token)] if an integer can be eaten, otherwise returns [`None`].
    /// An integer can be eaten if its bytes are at the front of the given `input_tendril` string.
    ///
    fn eat_integer(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<(usize, Token)> {
        if input.peek().is_none() {
            return Err(ParserError::lexer_empty_input_tendril().into());
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

    /// Returns the number of bytes in an utf-8 encoding that starts with this byte.
    fn _utf8_byte_count(byte: u8) -> usize {
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
    pub(crate) fn eat(input_tendril: &str) -> Result<(usize, Token)> {
        if input_tendril.is_empty() {
            return Err(ParserError::lexer_empty_input_tendril().into());
        }

        let mut input = input_tendril.chars().peekable();

        match input.peek() {
            Some(x) if x.is_ascii_whitespace() => {
                input.next();
                return Ok((1, Token::WhiteSpace));
            }
            Some('"') => {
                let mut string: Vec<leo_ast::Char> = Vec::new();
                input.next();

                let mut len = 0;
                while let Some(c) = input.peek() {
                    if c == &'"' {
                        break;
                    }
                    let (char_len, character) = Self::eat_char(&mut input)?;
                    len += char_len;
                    string.push(character.into());
                }

                if input.next_if_eq(&'"').is_some() {
                    return Ok((len + 2, Token::StringLit(string)));
                }

                return Err(ParserError::lexer_string_not_closed(string).into());
            }
            Some('\'') => {
                input.next();

                let (len, character) = Self::eat_char(&mut input)?;

                if input.next_if_eq(&'\'').is_some() {
                    input.next();
                    return Ok((len + 2, Token::CharLit(character)));
                } else if let Some(c) = input.next() {
                    return Err(ParserError::lexer_string_not_closed(c).into());
                } else {
                    return Err(ParserError::lexer_empty_input_tendril().into());
                }
            }
            Some(x) if x.is_ascii_digit() => {
                return Self::eat_integer(&mut input);
            }
            Some('!') => {
                input.next();
                if input.next_if_eq(&'=').is_some() {
                    return Ok((2, Token::NotEq));
                }
                return Ok((1, Token::Not));
            }
            Some('?') => {
                input.next();
                return Ok((1, Token::Question));
            }
            Some('&') => {
                input.next();
                if input.next_if_eq(&'&').is_some() {
                    return Ok((2, Token::And));
                }
                return Ok((1, Token::Ampersand));
            }
            Some('(') => {
                input.next();
                return Ok((1, Token::LeftParen));
            }
            Some(')') => {
                input.next();
                return Ok((1, Token::RightParen));
            }
            Some('_') => {
                input.next();
                return Ok((1, Token::Underscore));
            }
            Some('*') => {
                input.next();
                if input.next_if_eq(&'*').is_some() {
                    if input.next_if_eq(&'=').is_some() {
                        return Ok((3, Token::ExpEq));
                    }
                    return Ok((2, Token::Exp));
                } else if input.next_if_eq(&'=').is_some() {
                    return Ok((2, Token::MulEq));
                }
                return Ok((1, Token::Mul));
            }
            Some('+') => {
                input.next();
                if input.next_if_eq(&'=').is_some() {
                    return Ok((2, Token::AddEq));
                }
                return Ok((1, Token::Add));
            }
            Some(',') => {
                input.next();
                return Ok((1, Token::Comma));
            }
            Some('-') => {
                input.next();
                if input.next_if_eq(&'>').is_some() {
                    return Ok((2, Token::Arrow));
                } else if input.next_if_eq(&'=').is_some() {
                    return Ok((2, Token::MinusEq));
                }
                return Ok((1, Token::Minus));
            }
            Some('.') => {
                input.next();
                if input.next_if_eq(&'.').is_some() {
                    if input.next_if_eq(&'.').is_some() {
                        return Ok((3, Token::DotDotDot));
                    } else {
                        return Ok((2, Token::DotDot));
                    }
                }
                return Ok((1, Token::Dot));
            }
            Some(c) if c == &'/' => {
                input.next();
                if input.next_if_eq(&'/').is_some() {
                    let mut comment = String::from("//");

                    while let Some(c) = input.next_if(|c| c != &'\n') {
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
                } else if input.next_if_eq(&'=').is_some() {
                    return Ok((2, Token::DivEq));
                }
                return Ok((1, Token::Div));
            }
            Some(':') => {
                input.next();
                if input.next_if_eq(&':').is_some() {
                    return Ok((2, Token::DoubleColon));
                } else {
                    return Ok((1, Token::Colon));
                }
            }
            Some(';') => {
                input.next();
                return Ok((1, Token::Semicolon));
            }
            Some('<') => {
                input.next();
                if input.next_if_eq(&'=').is_some() {
                    return Ok((2, Token::LtEq));
                }
                return Ok((1, Token::Lt));
            }
            Some('>') => {
                input.next();
                if input.next_if_eq(&'=').is_some() {
                    return Ok((2, Token::GtEq));
                }
                return Ok((1, Token::Gt));
            }
            Some('=') => {
                input.next();
                if input.next_if_eq(&'=').is_some() {
                    return Ok((2, Token::Eq));
                }
                return Ok((1, Token::Assign));
            }
            Some('@') => {
                input.next();
                return Ok((1, Token::At));
            }
            Some('[') => {
                input.next();
                return Ok((1, Token::LeftSquare));
            }
            Some(']') => {
                input.next();
                return Ok((1, Token::RightSquare));
            }
            Some('{') => {
                input.next();
                return Ok((1, Token::LeftCurly));
            }
            Some('}') => {
                input.next();
                return Ok((1, Token::RightCurly));
            }
            Some('|') => {
                input.next();
                if input.next_if_eq(&'|').is_some() {
                    return Ok((2, Token::Or));
                } else if let Some(found) = input.next() {
                    return Err(ParserError::lexer_expected_but_found(found, '|').into());
                } else {
                    return Err(ParserError::lexer_empty_input_tendril().into());
                }
            }
            _ => (),
        }
        if let Some(ident) = eat_identifier(&mut input) {
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

        Err(ParserError::could_not_lex(input.collect::<String>()).into())
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
