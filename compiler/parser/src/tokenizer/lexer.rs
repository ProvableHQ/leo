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

use crate::tokenizer::Token;
use leo_errors::{ParserError, Result};
use leo_span::{Span, Symbol};

use serde::{Deserialize, Serialize};
use std::{
    fmt,
    iter::{from_fn, Peekable},
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

/// Ensure that `string` contains no Unicode Bidirectional Override code points.
fn ensure_no_bidi_override(string: &str) -> Result<()> {
    if string.chars().any(is_bidi_override) {
        return Err(ParserError::lexer_bidi_override().into());
    }
    Ok(())
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

        // Note that it is still impossible to have a number that starts with an `_` because eat_integer is only called when the first character is a digit.
        while let Some(c) = input.next_if(|c| c.is_ascii_digit() || *c == '_') {
            if c == '0' && matches!(input.peek(), Some('x')) {
                int.push(c);
                int.push(input.next().unwrap());
                return Err(ParserError::lexer_hex_number_provided(int).into());
            }

            int.push(c);
        }

        Ok((int.len(), Token::Integer(int)))
    }

    /// Returns a tuple: [(token length, token)] if the next token can be eaten, otherwise returns an error.
    /// The next token can be eaten if the bytes at the front of the given `input` string can be scanned into a token.
    pub(crate) fn eat(input: &str) -> Result<(usize, Token)> {
        if input.is_empty() {
            return Err(ParserError::lexer_empty_input().into());
        }

        let input_str = input;
        let mut input = input.chars().peekable();

        // Returns one token matching one character.
        let match_one = |input: &mut Peekable<_>, token| {
            input.next();
            Ok((1, token))
        };

        // Returns one token matching one or two characters.
        // If the `second` character matches, return the `second_token` that represents two characters.
        // Otherwise, return the `first_token` that matches the one character.
        let match_two = |input: &mut Peekable<_>, first_token, second_char, second_token| {
            input.next();
            Ok(if input.next_if_eq(&second_char).is_some() { (2, second_token) } else { (1, first_token) })
        };

        // Returns one token matching one or two characters.
        // If the `second_char` character matches, return the `second_token` that represents two characters.
        // If the `third_char` character matches, return the `third_token` that represents two characters.
        // Otherwise, return the `first_token` that matches the one character.
        let match_three = |input: &mut Peekable<_>, first_token, second_char, second_token, third_char, third_token| {
            input.next();
            Ok(if input.next_if_eq(&second_char).is_some() {
                (2, second_token)
            } else if input.next_if_eq(&third_char).is_some() {
                (2, third_token)
            } else {
                (1, first_token)
            })
        };

        // Returns one token matching one, two, or three characters.
        // The `fourth_token` expects both the `third_char` and `fourth_char` to be present.
        // See the example with the different combinations for Mul, MulAssign, Pow, PowAssign below.
        let match_four = |
            input: &mut Peekable<_>,
            first_token, // Mul '*'
            second_char, // '='
            second_token, // MulAssign '*='
            third_char, // '*'
            third_token, // Pow '**'
            fourth_char, // '='
            fourth_token // PowAssign '**='
        | {
            input.next();
            Ok(if input.next_if_eq(&second_char).is_some() {
                // '*='
                (2, second_token)
            } else if input.next_if_eq(&third_char).is_some() {
                if input.next_if_eq(&fourth_char).is_some() {
                    // '**='
                    return Ok((3, fourth_token))
                }
                // '**'
                (2, third_token)
            } else {
                // '*'
                (1, first_token)
            })
        };

        match *input.peek().ok_or_else(ParserError::lexer_empty_input)? {
            x if x.is_ascii_whitespace() => return match_one(&mut input, Token::WhiteSpace),
            '"' => {
                // Find end string quotation mark.
                // Instead of checking each `char` and pushing, we can avoid reallocations.
                // This works because the code 34 of double quote cannot appear as a byte
                // in middle of a multi-byte UTF-8 encoding of a character,
                // because those bytes all have the high bit set to 1;
                // in UTF-8, the byte 34 can only appear as the single-byte encoding of double quote.
                let rest = &input_str[1..];
                let string = match rest.as_bytes().iter().position(|c| *c == b'"') {
                    None => return Err(ParserError::lexer_string_not_closed(rest).into()),
                    Some(idx) => rest[..idx].to_owned(),
                };

                ensure_no_bidi_override(&string)?;

                // + 2 to account for parsing quotation marks.
                return Ok((string.len() + 2, Token::StaticString(string)));
            }

            x if x.is_ascii_digit() => return Self::eat_integer(&mut input),
            '!' => return match_two(&mut input, Token::Not, '=', Token::NotEq),
            '?' => return match_one(&mut input, Token::Question),
            '&' => {
                return match_four(
                    &mut input,
                    Token::BitAnd,
                    '=',
                    Token::BitAndAssign,
                    '&',
                    Token::And,
                    '=',
                    Token::AndAssign,
                );
            }
            '(' => return match_one(&mut input, Token::LeftParen),
            ')' => return match_one(&mut input, Token::RightParen),
            '_' => return match_one(&mut input, Token::Underscore),
            '*' => {
                return match_four(
                    &mut input,
                    Token::Mul,
                    '=',
                    Token::MulAssign,
                    '*',
                    Token::Pow,
                    '=',
                    Token::PowAssign,
                );
            }
            '+' => return match_two(&mut input, Token::Add, '=', Token::AddAssign),
            ',' => return match_one(&mut input, Token::Comma),
            '-' => return match_three(&mut input, Token::Sub, '=', Token::SubAssign, '>', Token::Arrow),
            '.' => return match_two(&mut input, Token::Dot, '.', Token::DotDot),
            '/' => {
                input.next();
                if input.next_if_eq(&'/').is_some() {
                    // Find the end of the comment line.
                    // This works because the code 10 of line feed cannot appear as a byte
                    // in middle of a multi-byte UTF-8 encoding of a character,
                    // because those bytes all have the high bit set to 1;
                    // in UTF-8, the byte 10 can only appear as the single-byte encoding of line feed.
                    let comment = match input_str.as_bytes().iter().position(|c| *c == b'\n') {
                        None => input_str,
                        Some(idx) => &input_str[..idx + 1],
                    };

                    ensure_no_bidi_override(comment)?;
                    return Ok((comment.len(), Token::CommentLine(comment.to_owned())));
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

                    ensure_no_bidi_override(&comment)?;

                    if !ended {
                        return Err(ParserError::lexer_block_comment_does_not_close_before_eof(comment).into());
                    }
                    return Ok((comment.len(), Token::CommentBlock(comment)));
                } else if input.next_if_eq(&'=').is_some() {
                    // '/='
                    return Ok((2, Token::DivAssign));
                }
                // '/'
                return Ok((1, Token::Div));
            }
            '%' => return match_two(&mut input, Token::Rem, '=', Token::RemAssign),
            ':' => return match_two(&mut input, Token::Colon, ':', Token::DoubleColon),
            ';' => return match_one(&mut input, Token::Semicolon),
            '<' => return match_four(&mut input, Token::Lt, '=', Token::LtEq, '<', Token::Shl, '=', Token::ShlAssign),
            '>' => return match_four(&mut input, Token::Gt, '=', Token::GtEq, '>', Token::Shr, '=', Token::ShrAssign),
            '=' => return match_three(&mut input, Token::Assign, '=', Token::Eq, '>', Token::BigArrow),
            '[' => return match_one(&mut input, Token::LeftSquare),
            ']' => return match_one(&mut input, Token::RightSquare),
            '{' => return match_one(&mut input, Token::LeftCurly),
            '}' => return match_one(&mut input, Token::RightCurly),
            '|' => {
                return match_four(
                    &mut input,
                    Token::BitOr,
                    '=',
                    Token::BitOrAssign,
                    '|',
                    Token::Or,
                    '=',
                    Token::OrAssign,
                );
            }
            '^' => return match_two(&mut input, Token::BitXor, '=', Token::BitXorAssign),
            '@' => return Ok((1, Token::At)),
            _ => (),
        }
        if let Some(identifier) = eat_identifier(&mut input) {
            return Ok((
                identifier.len(),
                // todo: match on symbols instead of hard-coded &str's
                match &*identifier {
                    x if x.starts_with("aleo1") => Token::AddressLit(identifier),
                    "address" => Token::Address,
                    "as" => Token::As,
                    "assert" => Token::Assert,
                    "assert_eq" => Token::AssertEq,
                    "assert_neq" => Token::AssertNeq,
                    "block" => Token::Block,
                    "bool" => Token::Bool,
                    "console" => Token::Console,
                    "const" => Token::Const,
                    "constant" => Token::Constant,
                    "else" => Token::Else,
                    "false" => Token::False,
                    "field" => Token::Field,
                    "finalize" => Token::Finalize,
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
                    "inline" => Token::Inline,
                    "let" => Token::Let,
                    "leo" => Token::Leo,
                    "mapping" => Token::Mapping,
                    "private" => Token::Private,
                    "program" => Token::Program,
                    "public" => Token::Public,
                    "record" => Token::Record,
                    "return" => Token::Return,
                    "scalar" => Token::Scalar,
                    "signature" => Token::Signature,
                    "self" => Token::SelfLower,
                    "string" => Token::String,
                    "struct" => Token::Struct,
                    "then" => Token::Then,
                    "transition" => Token::Transition,
                    "true" => Token::True,
                    "u8" => Token::U8,
                    "u16" => Token::U16,
                    "u32" => Token::U32,
                    "u64" => Token::U64,
                    "u128" => Token::U128,
                    _ => Token::Identifier(Symbol::intern(&identifier)),
                },
            ));
        }

        Err(ParserError::could_not_lex(input.take_while(|c| *c != ';' && !c.is_whitespace()).collect::<String>())
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
        Self { token: Token::Question, span: Span::dummy() }
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
