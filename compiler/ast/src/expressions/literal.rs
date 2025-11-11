// Copyright (C) 2019-2025 Provable Inc.
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Literal {
    pub span: Span,
    pub id: NodeID,
    pub variant: LiteralVariant,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum LiteralVariant {
    /// An address literal, e.g., `aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9` or `hello.aleo`.
    Address(String),
    /// A boolean literal, either `true` or `false`.
    Boolean(bool),
    /// A field literal, e.g., `42field`.
    /// A signed number followed by the keyword `field`.
    Field(String),
    /// A group literal, eg `42group`.
    Group(String),
    /// An integer literal, e.g., `42u32`.
    Integer(IntegerType, String),
    /// A literal `None` for optional types.
    None,
    /// A scalar literal, e.g. `1scalar`.
    /// An unsigned number followed by the keyword `scalar`.
    Scalar(String),
    /// An unsuffixed literal, e.g. `42` (without a type suffix)
    Unsuffixed(String),
    /// A string literal, e.g., `"foobar"`.
    String(String),
}

impl fmt::Display for LiteralVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Address(address) => write!(f, "{address}"),
            Self::Boolean(boolean) => write!(f, "{boolean}"),
            Self::Field(field) => write!(f, "{field}field"),
            Self::Group(group) => write!(f, "{group}group"),
            Self::Integer(type_, value) => write!(f, "{value}{type_}"),
            Self::None => write!(f, "none"),
            Self::Scalar(scalar) => write!(f, "{scalar}scalar"),
            Self::Unsuffixed(value) => write!(f, "{value}"),
            Self::String(string) => write!(f, "\"{string}\""),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.variant.fmt(f)
    }
}

crate::simple_node_impl!(Literal);

struct DisplayDecimal<'a>(&'a Literal);

impl Literal {
    pub fn string(s: String, span: Span, id: NodeID) -> Self {
        Literal { variant: LiteralVariant::String(s), span, id }
    }

    pub fn field(s: String, span: Span, id: NodeID) -> Self {
        Literal { variant: LiteralVariant::Field(s), span, id }
    }

    pub fn group(s: String, span: Span, id: NodeID) -> Self {
        Literal { variant: LiteralVariant::Group(s), span, id }
    }

    pub fn address(s: String, span: Span, id: NodeID) -> Self {
        Literal { variant: LiteralVariant::Address(s), span, id }
    }

    pub fn scalar(s: String, span: Span, id: NodeID) -> Self {
        Literal { variant: LiteralVariant::Scalar(s), span, id }
    }

    pub fn boolean(s: bool, span: Span, id: NodeID) -> Self {
        Literal { variant: LiteralVariant::Boolean(s), span, id }
    }

    pub fn integer(integer_type: IntegerType, s: String, span: Span, id: NodeID) -> Self {
        Literal { variant: LiteralVariant::Integer(integer_type, s), span, id }
    }

    pub fn unsuffixed(s: String, span: Span, id: NodeID) -> Self {
        Literal { variant: LiteralVariant::Unsuffixed(s), span, id }
    }

    pub fn none(span: Span, id: NodeID) -> Self {
        Literal { variant: LiteralVariant::None, span, id }
    }

    /// For displaying a literal as decimal, regardless of the radix in which it was parsed.
    ///
    /// In particular this is useful for outputting .aleo files.
    pub fn display_decimal(&self) -> impl '_ + fmt::Display {
        DisplayDecimal(self)
    }

    /// For an integer literal, parse it and cast it to a u32.
    pub fn as_u32(&self) -> Option<u32> {
        if let LiteralVariant::Integer(_, s) = &self.variant {
            u32::from_str_by_radix(&s.replace("_", "")).ok()
        } else {
            None
        }
    }
}

impl fmt::Display for DisplayDecimal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // This function is duplicated in `interpreter/src/cursor.rs`,
        // but there's not really a great place to put a common implementation
        // right now.
        fn prepare_snarkvm_string(s: &str, suffix: &str) -> String {
            // If there's a `-`, separate it from the rest of the string.
            let (neg, rest) = s.strip_prefix("-").map(|rest| ("-", rest)).unwrap_or(("", s));
            // Remove leading zeros.
            let mut rest = rest.trim_start_matches('0');
            if rest.is_empty() {
                rest = "0";
            }
            format!("{neg}{rest}{suffix}")
        }

        match &self.0.variant {
            LiteralVariant::Address(address) => write!(f, "{address}"),
            LiteralVariant::Boolean(boolean) => write!(f, "{boolean}"),
            LiteralVariant::Field(field) => write!(f, "{}", prepare_snarkvm_string(field, "field")),
            LiteralVariant::Group(group) => write!(f, "{}", prepare_snarkvm_string(group, "group")),
            LiteralVariant::Integer(type_, value) => {
                let string = value.replace('_', "");
                if value.starts_with('-') {
                    let v = i128::from_str_by_radix(&string).expect("Failed to parse integer?");
                    write!(f, "{v}{type_}")
                } else {
                    let v = u128::from_str_by_radix(&string).expect("Failed to parse integer?");
                    write!(f, "{v}{type_}")
                }
            }
            LiteralVariant::None => write!(f, "none"),
            LiteralVariant::Scalar(scalar) => write!(f, "{}", prepare_snarkvm_string(scalar, "scalar")),
            LiteralVariant::Unsuffixed(value) => write!(f, "{value}"),
            LiteralVariant::String(string) => write!(f, "\"{string}\""),
        }
    }
}

impl From<Literal> for Expression {
    fn from(value: Literal) -> Self {
        Expression::Literal(value)
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
