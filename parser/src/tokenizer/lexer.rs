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

use crate::tokenizer::{FormatStringPart, Token};
use leo_ast::Span;
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
    /// Returns a tuple: [(integer length, integer token)] if an integer can be eaten, otherwise returns [`None`].
    /// An integer can be eaten if its bytes are at the front of the given `input_tendril` string.
    ///
    fn eat_integer(input_tendril: &StrTendril) -> (usize, Option<Token>) {
        if input_tendril.is_empty() {
            return (0, None);
        }
        let input = input_tendril[..].as_bytes();
        if !input[0].is_ascii_digit() {
            return (0, None);
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
        (i, Some(Token::Int(input_tendril.subtendril(0, i as u32))))
    }

    ///
    /// Returns a tuple: [(token length, token)] if the next token can be eaten, otherwise returns [`None`].
    /// The next token can be eaten if the bytes at the front of the given `input_tendril` string can be scanned into a token.
    ///
    pub(crate) fn eat(input_tendril: StrTendril) -> (usize, Option<Token>) {
        if input_tendril.is_empty() {
            return (0, None);
        }
        let input = input_tendril[..].as_bytes();
        match input[0] {
            x if x.is_ascii_whitespace() => return (1, None),
            b'"' => {
                let mut i = 1;
                let mut in_escape = false;
                let mut start = 1usize;
                let mut segments = Vec::new();
                while i < input.len() {
                    if !in_escape {
                        if input[i] == b'"' {
                            break;
                        }
                        if input[i] == b'\\' {
                            in_escape = !in_escape;
                        } else if i < input.len() - 1 && input[i] == b'{' {
                            if i < input.len() - 2 && input[i + 1] == b'{' {
                                i += 2;
                                continue;
                            } else if input[i + 1] != b'}' {
                                i += 1;
                                continue;
                            }
                            if start < i {
                                segments.push(FormatStringPart::Const(
                                    input_tendril.subtendril(start as u32, (i - start) as u32),
                                ));
                            }
                            segments.push(FormatStringPart::Container);
                            start = i + 2;
                            i = start;
                            continue;
                        }
                    } else {
                        in_escape = false;
                    }
                    i += 1;
                }
                if i == input.len() {
                    return (0, None);
                }
                if start < i {
                    segments.push(FormatStringPart::Const(
                        input_tendril.subtendril(start as u32, (i - start) as u32),
                    ));
                }
                return (i + 1, Some(Token::FormatString(segments)));
            }
            b'\'' => {
                if input[1] == b'\'' {
                    return (0, None);
                }

                let mut i = 1;
                let mut in_escape = false;
                let mut character = String::new();
                while i < input.len() {
                    if !in_escape {
                        if input[i] == b'\'' {
                            break;
                        }
                        if input[i] == b'\\' {
                            in_escape = !in_escape;
                        } else {
                            character.push(input[i] as char);
                        }
                    } else {
                        in_escape = false;
                        if input[i] == b'u' {
                            i += 2;
                            let mut j = i;
                            let mut size = 0;
                            while input[j] != b'}' {
                                j += 1;
                                size += 1;
                            }
                            let hex_string_number: String = input_tendril.subtendril(i as u32, size).to_string();
                            if let Ok(hex) = u32::from_str_radix(&hex_string_number, 16) {
                                if let Some(unicode) = std::char::from_u32(hex) {
                                    i = j;
                                    character = unicode.to_string();
                                }
                            } else {
                                return (0, None);
                            }
                        } else {
                            character.push(input[i] as char);
                        }
                    }
                    i += 1;
                }
                if i == input.len() {
                    return (0, None);
                }

                return (i + 1, Some(Token::CharLit(character.into())));
            }
            x if x.is_ascii_digit() => {
                return Self::eat_integer(&input_tendril);
            }
            b'!' => {
                if let Some(len) = eat(input, "!=") {
                    return (len, Some(Token::NotEq));
                }
                return (1, Some(Token::Not));
            }
            b'?' => {
                return (1, Some(Token::Question));
            }
            b'&' => {
                if let Some(len) = eat(input, "&&") {
                    // if let Some(inner_len) = eat(&input[len..], "=") {
                    //     return (len + inner_len, Some(Token::AndEq));
                    // }
                    return (len, Some(Token::And));
                }
                // else if let Some(len) = eat(input, "&=") {
                //     return (len, Some(Token::BitAndEq));
                // }
                // return (1, Some(Token::BitAnd));
            }
            b'(' => return (1, Some(Token::LeftParen)),
            b')' => return (1, Some(Token::RightParen)),
            b'_' => return (1, Some(Token::Underscore)),
            b'*' => {
                if let Some(len) = eat(input, "**") {
                    if let Some(inner_len) = eat(&input[len..], "=") {
                        return (len + inner_len, Some(Token::ExpEq));
                    }
                    return (len, Some(Token::Exp));
                } else if let Some(len) = eat(input, "*=") {
                    return (len, Some(Token::MulEq));
                }
                return (1, Some(Token::Mul));
            }
            b'+' => {
                if let Some(len) = eat(input, "+=") {
                    return (len, Some(Token::AddEq));
                }
                return (1, Some(Token::Add));
            }
            b',' => return (1, Some(Token::Comma)),
            b'-' => {
                if let Some(len) = eat(input, "->") {
                    return (len, Some(Token::Arrow));
                } else if let Some(len) = eat(input, "-=") {
                    return (len, Some(Token::MinusEq));
                }
                return (1, Some(Token::Minus));
            }
            b'.' => {
                if let Some(len) = eat(input, "...") {
                    return (len, Some(Token::DotDotDot));
                } else if let Some(len) = eat(input, "..") {
                    return (len, Some(Token::DotDot));
                }
                return (1, Some(Token::Dot));
            }
            b'/' => {
                if eat(input, "//").is_some() {
                    let eol = input.iter().position(|x| *x == b'\n');
                    let len = if let Some(eol) = eol { eol + 1 } else { input.len() };
                    return (len, Some(Token::CommentLine(input_tendril.subtendril(0, len as u32))));
                } else if eat(input, "/*").is_some() {
                    if input.is_empty() {
                        return (0, None);
                    }
                    let eol = input.windows(2).skip(2).position(|x| x[0] == b'*' && x[1] == b'/');
                    let len = if let Some(eol) = eol { eol + 4 } else { input.len() };
                    return (len, Some(Token::CommentBlock(input_tendril.subtendril(0, len as u32))));
                } else if let Some(len) = eat(input, "/=") {
                    return (len, Some(Token::DivEq));
                }
                return (1, Some(Token::Div));
            }
            b':' => {
                if let Some(len) = eat(input, "::") {
                    return (len, Some(Token::DoubleColon));
                } else {
                    return (1, Some(Token::Colon));
                }
            }
            b';' => return (1, Some(Token::Semicolon)),
            b'<' => {
                if let Some(len) = eat(input, "<=") {
                    return (len, Some(Token::LtEq));
                }
                // else if let Some(len) = eat(input, "<<") {
                //     if let Some(inner_len) = eat(&input[len..], "=") {
                //         return (len + inner_len, Some(Token::ShlEq));
                //     }
                //     return (len, Some(Token::Shl));
                // }
                return (1, Some(Token::Lt));
            }
            b'>' => {
                if let Some(len) = eat(input, ">=") {
                    return (len, Some(Token::GtEq));
                }
                // else if let Some(len) = eat(input, ">>") {
                //     if let Some(inner_len) = eat(&input[len..], "=") {
                //         return (len + inner_len, Some(Token::ShrEq));
                //     } else if let Some(inner_len) = eat(&input[len..], ">") {
                //         if let Some(eq_len) = eat(&input[len + inner_len..], "=") {
                //             return (len + inner_len + eq_len, Some(Token::ShrSignedEq));
                //         }
                //         return (len + inner_len, Some(Token::ShrSigned));
                //     }
                //     return (len, Some(Token::Shr));
                // }
                return (1, Some(Token::Gt));
            }
            b'=' => {
                if let Some(len) = eat(input, "==") {
                    return (len, Some(Token::Eq));
                }
                return (1, Some(Token::Assign));
            }
            b'@' => return (1, Some(Token::At)),
            b'[' => return (1, Some(Token::LeftSquare)),
            b']' => return (1, Some(Token::RightSquare)),
            b'{' => return (1, Some(Token::LeftCurly)),
            b'}' => return (1, Some(Token::RightCurly)),
            b'|' => {
                if let Some(len) = eat(input, "||") {
                    // if let Some(inner_len) = eat(&input[len..], "=") {
                    //     return (len + inner_len, Some(Token::OrEq));
                    // }
                    return (len, Some(Token::Or));
                }
                // else if let Some(len) = eat(input, "|=") {
                //     return (len, Some(Token::BitOrEq));
                // }
                // return (1, Some(Token::BitOr));
            }
            // b'^' => {
            //     if let Some(len) = eat(input, "^=") {
            //         return (len, Some(Token::BitXorEq));
            //     }
            //     return (1, Some(Token::BitXor));
            // }
            // b'~' => return (1, Some(Token::BitNot)),
            // b'%' => {
            //     if let Some(len) = eat(input, "%=") {
            //         return (len, Some(Token::ModEq));
            //     }
            //     return (1, Some(Token::Mod));
            // }
            _ => (),
        }
        if let Some(ident) = eat_identifier(&input_tendril) {
            return (
                ident.len(),
                Some(match &*ident {
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
                    "string" => Token::String,
                    "true" => Token::True,
                    "u8" => Token::U8,
                    "u16" => Token::U16,
                    "u32" => Token::U32,
                    "u64" => Token::U64,
                    "u128" => Token::U128,
                    _ => Token::Ident(ident),
                }),
            );
        }

        (0, None)
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
