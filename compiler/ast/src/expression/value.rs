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

use super::*;

/// A literal expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueExpression {
    // todo: deserialize values here
    /// An address literal, e.g., `aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9`.
    Address(String, #[serde(with = "leo_span::span_json")] Span),
    /// A boolean literal, either `true` or `false`.
    Boolean(String, #[serde(with = "leo_span::span_json")] Span),
    /// A field literal, e.g., `42field`.
    /// A signed number followed by the keyword `field`.
    Field(String, #[serde(with = "leo_span::span_json")] Span),
    /// A group literal, either product or affine.
    /// For example, `42group` or `(12, 52)group`.
    Group(Box<GroupValue>),
    /// An integer literal, e.g., `42`.
    Integer(IntegerType, String, #[serde(with = "leo_span::span_json")] Span),
    /// A scalar literal, e.g. `1scalar`.
    /// An unsigned number followed by the keyword `scalar`.
    Scalar(String, #[serde(with = "leo_span::span_json")] Span),
    /// A string literal, e.g., `"foobar"`.
    String(String, #[serde(with = "leo_span::span_json")] Span),
}

impl fmt::Display for ValueExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ValueExpression::*;
        match &self {
            Address(address, _) => write!(f, "{}", address),
            Boolean(boolean, _) => write!(f, "{}", boolean),
            Field(field, _) => write!(f, "{}", field),
            Group(group) => write!(f, "{}", group),
            Integer(type_, value, _) => write!(f, "{}{}", value, type_),
            Scalar(scalar, _) => write!(f, "{}", scalar),
            String(string, _) => write!(f, "{}", string),
        }
    }
}

impl Node for ValueExpression {
    fn span(&self) -> Span {
        use ValueExpression::*;
        match &self {
            Address(_, span)
            | Boolean(_, span)
            | Field(_, span)
            | Integer(_, _, span)
            | Scalar(_, span)
            | String(_, span) => *span,
            Group(group) => match &**group {
                GroupValue::Single(_, span) => *span,
                GroupValue::Tuple(tuple) => tuple.span,
            },
        }
    }

    fn set_span(&mut self, new_span: Span) {
        use ValueExpression::*;
        match self {
            Address(_, span)
            | Boolean(_, span)
            | Field(_, span)
            | Integer(_, _, span)
            | Scalar(_, span)
            | String(_, span) => *span = new_span,
            Group(group) => match &mut **group {
                GroupValue::Single(_, span) => *span = new_span,
                GroupValue::Tuple(tuple) => tuple.span = new_span,
            },
        }
    }
}
