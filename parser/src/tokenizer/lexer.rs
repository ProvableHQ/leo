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

use crate::tokenizer::{FormattedStringPart, Token};
use leo_ast::Span;
use serde::{Deserialize, Serialize};

use std::fmt;

fn eat<'a>(input: &'a [u8], wanted: &str) -> Option<&'a [u8]> {
    let wanted = wanted.as_bytes();
    if input.len() < wanted.len() {
        return None;
    }
    if &input[0..wanted.len()] == wanted {
        return Some(&input[wanted.len()..]);
    }
    None
}

fn eat_identifier(input: &[u8]) -> Option<(&[u8], &[u8])> {
    if input.is_empty() {
        return None;
    }
    if !input[0].is_ascii_alphabetic() && input[0] != b'_' {
        return None;
    }
    let mut i = 1usize;
    while i < input.len() {
        if !input[i].is_ascii_alphanumeric() && input[i] != b'_' {
            break;
        }
        i += 1;
    }
    Some((&input[0..i], &input[i..]))
}

impl Token {
    fn gobble_int(input: &[u8]) -> (&[u8], Option<Token>) {
        if input.is_empty() {
            return (input, None);
        }
        if !input[0].is_ascii_digit() {
            return (input, None);
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
        (
            &input[i..],
            Some(Token::Int(String::from_utf8(input[0..i].to_vec()).unwrap_or_default())),
        )
    }

    pub(crate) fn gobble(input: &[u8]) -> (&[u8], Option<Token>) {
        if input.is_empty() {
            return (input, None);
        }
        match input[0] {
            x if x.is_ascii_whitespace() => return (&input[1..], None),
            b'"' => {
                let mut i = 1;
                let mut in_escape = false;
                let mut start = 1usize;
                let mut segments = vec![];
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
                                segments.push(FormattedStringPart::Const(
                                    String::from_utf8_lossy(&input[start..i]).to_string(),
                                ));
                            }
                            segments.push(FormattedStringPart::Container);
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
                    return (input, None);
                }
                if start < i {
                    segments.push(FormattedStringPart::Const(
                        String::from_utf8_lossy(&input[start..i]).to_string(),
                    ));
                }
                return (&input[(i + 1)..], Some(Token::FormattedString(segments)));
            }
            x if x.is_ascii_digit() => {
                return Self::gobble_int(input);
            }
            b'!' => {
                if let Some(input) = eat(input, "!=") {
                    return (input, Some(Token::NotEq));
                }
                return (&input[1..], Some(Token::Not));
            }
            b'?' => {
                return (&input[1..], Some(Token::Question));
            }
            b'&' => {
                if let Some(input) = eat(input, "&&") {
                    if let Some(input) = eat(input, "=") {
                        return (input, Some(Token::AndEq));
                    }
                    return (input, Some(Token::And));
                } else if let Some(input) = eat(input, "&=") {
                    return (input, Some(Token::BitAndEq));
                }
                return (&input[1..], Some(Token::BitAnd));
            }
            b'(' => return (&input[1..], Some(Token::LeftParen)),
            b')' => return (&input[1..], Some(Token::RightParen)),
            b'*' => {
                if let Some(input) = eat(input, "**") {
                    if let Some(input) = eat(input, "=") {
                        return (input, Some(Token::ExpEq));
                    }
                    return (input, Some(Token::Exp));
                } else if let Some(input) = eat(input, "*=") {
                    return (input, Some(Token::MulEq));
                }
                return (&input[1..], Some(Token::Mul));
            }
            b'+' => {
                if let Some(input) = eat(input, "+=") {
                    return (input, Some(Token::AddEq));
                }
                return (&input[1..], Some(Token::Add));
            }
            b',' => return (&input[1..], Some(Token::Comma)),
            b'-' => {
                if let Some(input) = eat(input, "->") {
                    return (input, Some(Token::Arrow));
                } else if let Some(input) = eat(input, "-=") {
                    return (input, Some(Token::MinusEq));
                }
                return (&input[1..], Some(Token::Minus));
            }
            b'.' => {
                if let Some(input) = eat(input, "...") {
                    return (input, Some(Token::DotDotDot));
                } else if let Some(input) = eat(input, "..") {
                    return (input, Some(Token::DotDot));
                }
                return (&input[1..], Some(Token::Dot));
            }
            b'/' => {
                if eat(input, "//").is_some() {
                    let eol = input.iter().position(|x| *x == b'\n');
                    let (input, comment) = if let Some(eol) = eol {
                        (&input[(eol + 1)..], &input[..eol])
                    } else {
                        (&input[input.len()..input.len()], &input[..])
                    };
                    return (
                        input,
                        Some(Token::CommentLine(String::from_utf8_lossy(comment).to_string())),
                    );
                } else if eat(input, "/*").is_some() {
                    if input.is_empty() {
                        return (input, None);
                    }
                    let eol = input.windows(2).skip(2).position(|x| x[0] == b'*' && x[1] == b'/');
                    let (input, comment) = if let Some(eol) = eol {
                        (&input[(eol + 4)..], &input[..eol + 4])
                    } else {
                        (&input[input.len()..input.len()], &input[..])
                    };
                    return (
                        input,
                        Some(Token::CommentBlock(String::from_utf8_lossy(comment).to_string())),
                    );
                } else if let Some(input) = eat(input, "/=") {
                    return (input, Some(Token::DivEq));
                }
                return (&input[1..], Some(Token::Div));
            }
            b':' => {
                if let Some(input) = eat(input, "::") {
                    return (input, Some(Token::DoubleColon));
                } else {
                    return (&input[1..], Some(Token::Colon));
                }
            }
            b';' => return (&input[1..], Some(Token::Semicolon)),
            b'<' => {
                if let Some(input) = eat(input, "<=") {
                    return (input, Some(Token::LtEq));
                } else if let Some(input) = eat(input, "<<") {
                    if let Some(input) = eat(input, "=") {
                        return (input, Some(Token::ShlEq));
                    }
                    return (input, Some(Token::Shl));
                }
                return (&input[1..], Some(Token::Lt));
            }
            b'>' => {
                if let Some(input) = eat(input, ">=") {
                    return (input, Some(Token::GtEq));
                } else if let Some(input) = eat(input, ">>") {
                    if let Some(input) = eat(input, "=") {
                        return (input, Some(Token::ShrEq));
                    } else if let Some(input) = eat(input, ">") {
                        if let Some(input) = eat(input, "=") {
                            return (input, Some(Token::ShrSignedEq));
                        }
                        return (input, Some(Token::ShrSigned));
                    }
                    return (input, Some(Token::Shr));
                }
                return (&input[1..], Some(Token::Gt));
            }
            b'=' => {
                if let Some(input) = eat(input, "==") {
                    return (input, Some(Token::Eq));
                }
                return (&input[1..], Some(Token::Assign));
            }
            b'@' => return (&input[1..], Some(Token::At)),
            b'[' => return (&input[1..], Some(Token::LeftSquare)),
            b']' => return (&input[1..], Some(Token::RightSquare)),
            b'{' => return (&input[1..], Some(Token::LeftCurly)),
            b'}' => return (&input[1..], Some(Token::RightCurly)),
            b'|' => {
                if let Some(input) = eat(input, "||") {
                    if let Some(input) = eat(input, "=") {
                        return (input, Some(Token::OrEq));
                    }
                    return (input, Some(Token::Or));
                } else if let Some(input) = eat(input, "|=") {
                    return (input, Some(Token::BitOrEq));
                }
                return (&input[1..], Some(Token::BitOr));
            }
            b'^' => {
                if let Some(input) = eat(input, "^=") {
                    return (input, Some(Token::BitXorEq));
                }
                return (&input[1..], Some(Token::BitXor));
            }
            b'~' => return (&input[1..], Some(Token::BitNot)),
            b'%' => {
                if let Some(input) = eat(input, "%=") {
                    return (input, Some(Token::ModEq));
                }
                return (&input[1..], Some(Token::Mod));
            }
            _ => (),
        }
        if let Some((ident, input)) = eat_identifier(input) {
            let ident = String::from_utf8_lossy(ident).to_string();
            return (
                input,
                Some(match &*ident {
                    x if x.starts_with("aleo1")
                        && x.chars().skip(5).all(|x| x.is_ascii_lowercase() || x.is_ascii_digit()) =>
                    {
                        Token::AddressLit(x.to_string())
                    }
                    "address" => Token::Address,
                    "as" => Token::As,
                    "bool" => Token::Bool,
                    "circuit" => Token::Circuit,
                    "const" => Token::Const,
                    "else" => Token::Else,
                    "false" => Token::False,
                    "field" => Token::Field,
                    "for" => Token::For,
                    "function" => Token::Function,
                    "group" => Token::Group,
                    "i128" => Token::I128,
                    "i64" => Token::I64,
                    "i32" => Token::I32,
                    "i16" => Token::I16,
                    "i8" => Token::I8,
                    "if" => Token::If,
                    "import" => Token::Import,
                    "in" => Token::In,
                    "input" => Token::Input,
                    "let" => Token::Let,
                    "mut" => Token::Mut,
                    "return" => Token::Return,
                    "static" => Token::Static,
                    "string" => Token::Str,
                    "true" => Token::True,
                    "u128" => Token::U128,
                    "u64" => Token::U64,
                    "u32" => Token::U32,
                    "u16" => Token::U16,
                    "u8" => Token::U8,
                    "Self" => Token::BigSelf,
                    "self" => Token::LittleSelf,
                    "console" => Token::Console,
                    _ => Token::Ident(ident),
                }),
            );
        }

        (input, None)
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

pub(crate) fn validate_address(address: &str) -> bool {
    // "aleo1" (LOWERCASE_LETTER | ASCII_DIGIT){58}
    if !address.starts_with("aleo1") || address.len() != 63 {
        return false;
    }
    address
        .chars()
        .skip(5)
        .all(|x| x.is_ascii_lowercase() || x.is_ascii_digit())
}
