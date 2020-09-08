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
    access::Access,
    ast::Rule,
    common::{Identifier, SelfKeyword},
    functions::InputKeyword,
    SpanDef,
};

use crate::types::SelfType;
use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::expression_postfix))]
pub struct PostfixExpression<'ast> {
    pub name: KeywordOrIdentifier<'ast>,
    pub accesses: Vec<Access<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::keyword_or_identifier))]
pub enum KeywordOrIdentifier<'ast> {
    SelfKeyword(SelfKeyword<'ast>),
    SelfType(SelfType<'ast>),
    Input(InputKeyword<'ast>),
    Identifier(Identifier<'ast>),
}
