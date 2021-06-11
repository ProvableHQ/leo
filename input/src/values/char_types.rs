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

use crate::{
    ast::{span_into_string, Rule},
    errors::InputParserError,
};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::basic_char))]
pub struct BasicChar<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::escaped_char))]
pub struct EscapedChar<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::hex_char))]
pub struct HexChar<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::unicode_char))]
pub struct UnicodeChar<'ast> {
    #[pest_ast(outer(with(span_into_string)))]
    pub value: String,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::char_types))]
pub enum CharTypes<'ast> {
    Basic(BasicChar<'ast>),
    Escaped(EscapedChar<'ast>),
    Hex(HexChar<'ast>),
    Unicode(UnicodeChar<'ast>),
}

impl<'ast> CharTypes<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            CharTypes::Basic(value) => &value.span,
            CharTypes::Escaped(value) => &value.span,
            CharTypes::Hex(value) => &value.span,
            CharTypes::Unicode(value) => &value.span,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Char {
    Scalar(char),
    NonScalar(u32),
}

impl<'ast> CharTypes<'ast> {
    pub fn inner(self) -> Result<Char, InputParserError> {
        match self {
            Self::Basic(character) => {
                if let Some(character) = character.value.chars().next() {
                    return Ok(Char::Scalar(character));
                }

                Err(InputParserError::invalid_char(character.value, &character.span))
            }
            Self::Escaped(character) => {
                if let Some(inner) = character.value.chars().nth(1) {
                    return match inner {
                        '0' => Ok(Char::Scalar(0 as char)),
                        't' => Ok(Char::Scalar(9 as char)),
                        'n' => Ok(Char::Scalar(10 as char)),
                        'r' => Ok(Char::Scalar(13 as char)),
                        '\"' => Ok(Char::Scalar(34 as char)),
                        '\'' => Ok(Char::Scalar(39 as char)),
                        '\\' => Ok(Char::Scalar(92 as char)),
                        _ => Err(InputParserError::invalid_char(character.value, &character.span)),
                    };
                }

                Err(InputParserError::invalid_char(character.value, &character.span))
            }
            Self::Hex(character) => {
                let hex_string_number = character.value[2..character.value.len()].to_string();
                if let Ok(number) = u8::from_str_radix(&hex_string_number, 16) {
                    if number < 127 {
                        return Ok(Char::Scalar(number as char));
                    }
                }

                Err(InputParserError::invalid_char(character.value, &character.span))
            }
            Self::Unicode(character) => {
                let unicode_string_number = character.value[3..=character.value.len() - 2].to_string();
                if let Ok(hex) = u32::from_str_radix(&unicode_string_number, 16) {
                    if let Some(unicode) = std::char::from_u32(hex) {
                        return Ok(Char::Scalar(unicode));
                    } else if hex <= 0x10FFFF {
                        return Ok(Char::NonScalar(hex));
                    }
                }

                Err(InputParserError::invalid_char(character.value, &character.span))
            }
        }
    }
}
