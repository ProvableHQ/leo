// Copyright (C) 2019-2020 Aleo Systems Inc.
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
    ast::Rule,
    values::{NegativeNumber, PositiveNumber},
};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::value_number))]
pub enum NumberValue<'ast> {
    Negative(NegativeNumber<'ast>),
    Positive(PositiveNumber<'ast>),
}

impl<'ast> NumberValue<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            NumberValue::Negative(number) => &number.span,
            NumberValue::Positive(number) => &number.span,
        }
    }
}

impl<'ast> fmt::Display for NumberValue<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumberValue::Negative(number) => write!(f, "{}", number),
            NumberValue::Positive(number) => write!(f, "{}", number),
        }
    }
}
