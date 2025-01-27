// Copyright (C) 2019-2025 Aleo Systems Inc.
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

use crate::IntegerType;

use super::*;

/// A literal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Literal {
    // todo: deserialize values here
    /// An address literal, e.g., `aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9` or `hello.aleo`.
    Address(String, #[serde(with = "leo_span::span_json")] Span, NodeID),
    /// A boolean literal, either `true` or `false`.
    Boolean(bool, #[serde(with = "leo_span::span_json")] Span, NodeID),
    /// A field literal, e.g., `42field`.
    /// A signed number followed by the keyword `field`.
    Field(String, #[serde(with = "leo_span::span_json")] Span, NodeID),
    /// A group literal, eg `42group`.
    Group(String, #[serde(with = "leo_span::span_json")] Span, NodeID),
    /// An integer literal, e.g., `42`.
    Integer(IntegerType, String, #[serde(with = "leo_span::span_json")] Span, NodeID),
    /// A scalar literal, e.g. `1scalar`.
    /// An unsigned number followed by the keyword `scalar`.
    Scalar(String, #[serde(with = "leo_span::span_json")] Span, NodeID),
    /// A string literal, e.g., `"foobar"`.
    String(String, #[serde(with = "leo_span::span_json")] Span, NodeID),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Address(address, _, _) => write!(f, "{address}"),
            Self::Boolean(boolean, _, _) => write!(f, "{boolean}"),
            Self::Field(field, _, _) => write!(f, "{field}field"),
            Self::Group(group, _, _) => write!(f, "{group}group"),
            Self::Integer(type_, value, _, _) => write!(f, "{value}{type_}"),
            Self::Scalar(scalar, _, _) => write!(f, "{scalar}scalar"),
            Self::String(string, _, _) => write!(f, "\"{string}\""),
        }
    }
}

impl Node for Literal {
    fn span(&self) -> Span {
        match &self {
            Self::Address(_, span, _)
            | Self::Boolean(_, span, _)
            | Self::Field(_, span, _)
            | Self::Group(_, span, _)
            | Self::Integer(_, _, span, _)
            | Self::Scalar(_, span, _)
            | Self::String(_, span, _) => *span,
        }
    }

    fn set_span(&mut self, new_span: Span) {
        match self {
            Self::Address(_, span, _)
            | Self::Boolean(_, span, _)
            | Self::Field(_, span, _)
            | Self::Integer(_, _, span, _)
            | Self::Group(_, span, _)
            | Self::Scalar(_, span, _)
            | Self::String(_, span, _) => *span = new_span,
        }
    }

    fn id(&self) -> NodeID {
        match &self {
            Self::Address(_, _, id)
            | Self::Boolean(_, _, id)
            | Self::Field(_, _, id)
            | Self::Group(_, _, id)
            | Self::Integer(_, _, _, id)
            | Self::Scalar(_, _, id)
            | Self::String(_, _, id) => *id,
        }
    }

    fn set_id(&mut self, id: NodeID) {
        match self {
            Self::Address(_, _, old_id)
            | Self::Boolean(_, _, old_id)
            | Self::Field(_, _, old_id)
            | Self::Group(_, _, old_id)
            | Self::Integer(_, _, _, old_id)
            | Self::Scalar(_, _, old_id)
            | Self::String(_, _, old_id) => *old_id = id,
        }
    }
}

struct DisplayDecimal<'a>(&'a Literal);

impl Literal {
    /// For displaying a literal as decimal, regardless of the radix in which it was parsed.
    ///
    /// In particular this is useful for outputting .aleo files.
    pub fn display_decimal(&self) -> impl '_ + fmt::Display {
        DisplayDecimal(self)
    }
}

impl fmt::Display for DisplayDecimal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Literal::Address(address, _, _) => write!(f, "{address}"),
            Literal::Boolean(boolean, _, _) => write!(f, "{boolean}"),
            Literal::Field(field, _, _) => write!(f, "{field}field"),
            Literal::Group(group, _, _) => write!(f, "{group}group"),
            Literal::Integer(type_, value, _, _) => {
                if !value.starts_with("0x")
                    && !value.starts_with("-0x")
                    && !value.starts_with("0b")
                    && !value.starts_with("-0b")
                    && !value.starts_with("0o")
                    && !value.starts_with("-0o")
                {
                    // It's already decimal.
                    return write!(f, "{value}{type_}");
                }
                let string = value.replace('_', "");
                if value.starts_with('-') {
                    let v = i128::from_str_by_radix(&string).expect("Failed to parse integer?");
                    write!(f, "{v}{type_}")
                } else {
                    let v = u128::from_str_by_radix(&string).expect("Failed to parse integer?");
                    write!(f, "{v}{type_}")
                }
            }
            Literal::Scalar(scalar, _, _) => write!(f, "{scalar}scalar"),
            Literal::String(string, _, _) => write!(f, "\"{string}\""),
        }
    }
}

/// This trait allows to parse integer literals of any type generically.
///
/// The literal may optionally start with a `-` and/or `0x` or `0o` or 0b`.
pub trait FromStrRadix: Sized {
    fn from_str_by_radix(src: &str) -> Result<Self, std::num::ParseIntError>;
}

macro_rules! implement_from_str_radix {
    ($($ty:ident)*) => {
        $(
            impl FromStrRadix for $ty {
                fn from_str_by_radix(src: &str) -> Result<Self, std::num::ParseIntError> {
                    if let Some(stripped) = src.strip_prefix("0x") {
                        Self::from_str_radix(stripped, 16)
                    } else if let Some(stripped) = src.strip_prefix("0o") {
                        Self::from_str_radix(stripped, 8)
                    } else if let Some(stripped) = src.strip_prefix("0b") {
                        Self::from_str_radix(stripped, 2)
                    } else if let Some(stripped) = src.strip_prefix("-0x") {
                        // We have to remove the 0x prefix and put back in a - to use
                        // std's parsing. Alternatively we could jump through
                        // a few hoops to avoid allocating.
                        let mut s = String::new();
                        s.push('-');
                        s.push_str(stripped);
                        Self::from_str_radix(&s, 16)
                    } else if let Some(stripped) = src.strip_prefix("-0o") {
                        // Ditto.
                        let mut s = String::new();
                        s.push('-');
                        s.push_str(stripped);
                        Self::from_str_radix(&s, 8)
                    } else if let Some(stripped) = src.strip_prefix("-0b") {
                        // Ditto.
                        let mut s = String::new();
                        s.push('-');
                        s.push_str(stripped);
                        Self::from_str_radix(&s, 2)
                    } else {
                        Self::from_str_radix(src, 10)
                    }
                }
            }
        )*
    };
}

implement_from_str_radix! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 }
