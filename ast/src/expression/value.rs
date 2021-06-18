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

use tendril::StrTendril;

use super::*;
use crate::{Char, CharValue, GroupTuple};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueExpression {
    // todo: deserialize values here
    Address(#[serde(with = "crate::common::tendril_json")] StrTendril, Span),
    Boolean(#[serde(with = "crate::common::tendril_json")] StrTendril, Span),
    Char(CharValue),
    Field(#[serde(with = "crate::common::tendril_json")] StrTendril, Span),
    Group(Box<GroupValue>),
    Implicit(#[serde(with = "crate::common::tendril_json")] StrTendril, Span),
    Integer(
        IntegerType,
        #[serde(with = "crate::common::tendril_json")] StrTendril,
        Span,
    ),
    String(Vec<Char>, Span),
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
            Integer(value, type_, _) => write!(f, "{}{}", value, type_),
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
                GroupValue::Single(_, span) | GroupValue::Tuple(GroupTuple { span, .. }) => span,
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
                GroupValue::Single(_, span) | GroupValue::Tuple(GroupTuple { span, .. }) => *span = new_span,
            },
        }
    }
}
