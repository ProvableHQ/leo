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

use crate::{GroupLiteral, IntegerType};

use super::*;

/// A literal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Literal {
    // todo: deserialize values here
    /// An address literal, e.g., `aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9`.
    Address(String, #[serde(with = "leo_span::span_json")] Span, NodeID),
    /// A boolean literal, either `true` or `false`.
    Boolean(bool, #[serde(with = "leo_span::span_json")] Span, NodeID),
    /// A field literal, e.g., `42field`.
    /// A signed number followed by the keyword `field`.
    Field(String, #[serde(with = "leo_span::span_json")] Span, NodeID),
    /// A group literal, either product or affine.
    /// For example, `42group` or `(12, 52)group`.
    Group(Box<GroupLiteral>),
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
            Self::Group(group) => write!(f, "{group}group"),
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
            | Self::Integer(_, _, span, _)
            | Self::Scalar(_, span, _)
            | Self::String(_, span, _) => *span,
            Self::Group(group) => *group.span(),
        }
    }

    fn set_span(&mut self, new_span: Span) {
        match self {
            Self::Address(_, span, _)
            | Self::Boolean(_, span, _)
            | Self::Field(_, span, _)
            | Self::Integer(_, _, span, _)
            | Self::Scalar(_, span, _)
            | Self::String(_, span, _) => *span = new_span,
            Self::Group(group) => group.set_span(new_span),
        }
    }

    fn id(&self) -> NodeID {
        match &self {
            Self::Address(_, _, id)
            | Self::Boolean(_, _, id)
            | Self::Field(_, _, id)
            | Self::Integer(_, _, _, id)
            | Self::Scalar(_, _, id)
            | Self::String(_, _, id) => *id,
            Self::Group(group) => *group.id(),
        }
    }

    fn set_id(&mut self, id: NodeID) {
        match self {
            Self::Address(_, _, old_id)
            | Self::Boolean(_, _, old_id)
            | Self::Field(_, _, old_id)
            | Self::Integer(_, _, _, old_id)
            | Self::Scalar(_, _, old_id)
            | Self::String(_, _, old_id) => *old_id = id,
            Self::Group(group) => group.set_id(id),
        }
    }
}
