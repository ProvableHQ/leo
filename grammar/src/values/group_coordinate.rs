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

use crate::{ast::Rule, values::NumberValue, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::group_coordinate))]
pub enum GroupCoordinate<'ast> {
    Number(NumberValue<'ast>),
    SignHigh(SignHigh<'ast>),
    SignLow(SignLow<'ast>),
    Inferred(Inferred<'ast>),
}

impl<'ast> GroupCoordinate<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            GroupCoordinate::Number(number) => &number.span(),
            GroupCoordinate::SignHigh(sign_high) => &sign_high.span,
            GroupCoordinate::SignLow(sign_low) => &sign_low.span,
            GroupCoordinate::Inferred(inferred) => &inferred.span,
        }
    }
}

impl<'ast> fmt::Display for GroupCoordinate<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GroupCoordinate::Number(number) => write!(f, "{}", number),
            GroupCoordinate::SignHigh(_) => write!(f, "+"),
            GroupCoordinate::SignLow(_) => write!(f, "-"),
            GroupCoordinate::Inferred(_) => write!(f, "_"),
        }
    }
}

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::sign_high))]
pub struct SignHigh<'ast> {
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::sign_low))]
pub struct SignLow<'ast> {
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::inferred))]
pub struct Inferred<'ast> {
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}
