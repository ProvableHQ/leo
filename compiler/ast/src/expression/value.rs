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

use tendril::StrTendril;

use super::*;
use crate::{Char, CharValue};

/// A literal expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueExpression {
    // todo: deserialize values here
    /// An address literal, e.g., `aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8`.
    Address(
        #[serde(with = "leo_span::tendril_json")] StrTendril,
        #[serde(with = "leo_span::span_json")] Span,
    ),
    /// A boolean literal, either `true` or `false`.
    Boolean(
        #[serde(with = "leo_span::tendril_json")] StrTendril,
        #[serde(with = "leo_span::span_json")] Span,
    ),
    /// A char literal, e.g., `'a'`, representing a single unicode code point.
    Char(CharValue),
    /// A field literal, e.g., `42field`.
    /// That is, a signed number followed by the keyword `field`.
    Field(
        #[serde(with = "leo_span::tendril_json")] StrTendril,
        #[serde(with = "leo_span::span_json")] Span,
    ),
    /// A group literal, either product or affine.
    /// For example, `42group` or `(12, 52)group`.
    Group(Box<GroupValue>),
    /// A negated non-integer literal, e.g., `-4.2`.
    Implicit(
        #[serde(with = "leo_span::tendril_json")] StrTendril,
        #[serde(with = "leo_span::span_json")] Span,
    ),
    /// An integer literal, e.g., `42`.
    Integer(
        IntegerType,
        #[serde(with = "leo_span::tendril_json")] StrTendril,
        #[serde(with = "leo_span::span_json")] Span,
    ),
    /// A string literal, e.g., `"foobar"`.
    String(Vec<Char>, #[serde(with = "leo_span::span_json")] Span),
}

impl fmt::Display for ValueExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ValueExpression::*;
        match &self {
            Address(address, _) => write!(f, "{}", address),
            Boolean(boolean, _) => write!(f, "{}", boolean),
            Char(character) => write!(f, "{}", character),
            Field(field, _) => write!(f, "{}", field),
            Implicit(implicit, _) => write!(f, "{}", implicit),
            Integer(type_, value, _) => write!(f, "{}{}", value, type_),
            Group(group) => write!(f, "{}", group),
            String(string, _) => {
                for character in string.iter() {
                    write!(f, "{}", character)?;
                }
                Ok(())
            }
        }
    }
}

impl Node for ValueExpression {
    fn span(&self) -> &Span {
        use ValueExpression::*;
        match &self {
            Address(_, span)
            | Boolean(_, span)
            | Field(_, span)
            | Implicit(_, span)
            | Integer(_, _, span)
            | String(_, span) => span,
            Char(character) => &character.span,
            Group(group) => match &**group {
                GroupValue::Single(_, span) => span,
                GroupValue::Tuple(tuple) => &tuple.span,
            },
        }
    }

    fn set_span(&mut self, new_span: Span) {
        use ValueExpression::*;
        match self {
            Address(_, span)
            | Boolean(_, span)
            | Field(_, span)
            | Implicit(_, span)
            | Integer(_, _, span)
            | String(_, span) => *span = new_span,
            Char(character) => character.span = new_span,
            Group(group) => match &mut **group {
                GroupValue::Single(_, span) => *span = new_span,
                GroupValue::Tuple(tuple) => tuple.span = new_span,
            },
        }
    }
}
