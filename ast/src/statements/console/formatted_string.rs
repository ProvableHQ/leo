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

use crate::{Expression, Node, Span};

use serde::{Deserialize, Serialize};
use std::fmt;
use tendril::StrTendril;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum FormatStringPart {
    Const(#[serde(with = "crate::common::tendril_json")] StrTendril),
    Container,
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
                    FormatStringPart::Const(x) => x,
                    FormatStringPart::Container => "{}",
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
