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

use crate::{Char, Expression, Node, Span};

use serde::{Deserialize, Serialize};
use std::fmt;
use tendril::StrTendril;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum FormatStringPart {
    Const(#[serde(with = "crate::common::tendril_json")] StrTendril),
    Container,
}

impl FormatStringPart {
    pub fn from_string(string: Vec<Char>) -> Vec<Self> {
        let mut parts = Vec::new();
        let mut in_container = false;
        let mut substring = String::new();
        for (_, character) in string.iter().enumerate() {
            match character {
                Char::Scalar(scalar) => match scalar {
                    '{' if !in_container => {
                        parts.push(FormatStringPart::Const(substring.clone().into()));
                        substring.clear();
                        in_container = true;
                    }
                    '}' if in_container => {
                        in_container = false;
                        parts.push(FormatStringPart::Container);
                    }
                    _ if in_container => {
                        in_container = false;
                    }
                    _ => substring.push(*scalar),
                },
                Char::NonScalar(non_scalar) => {
                    substring.push_str(format!("\\u{{{:x}}}", non_scalar).as_str());
                    in_container = false;
                }
            }
        }

        if !substring.is_empty() {
            parts.push(FormatStringPart::Const(substring.into()));
        }

        parts
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct FormatString {
    pub parts: Vec<FormatStringPart>,
    pub parameters: Vec<Expression>,
    pub span: Span,
}

impl fmt::Display for FormatString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.parts
                .iter()
                .map(|x| match x {
                    FormatStringPart::Const(x) => x.to_string(),
                    FormatStringPart::Container => "{}".to_string(),
                })
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

impl Node for FormatString {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
