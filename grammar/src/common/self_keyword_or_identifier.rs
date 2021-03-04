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

use crate::ast::Rule;
use crate::common::Identifier;
use crate::common::SelfKeyword;

use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::self_keyword_or_identifier))]
pub enum SelfKeywordOrIdentifier<'ast> {
    SelfKeyword(SelfKeyword<'ast>),
    Identifier(Identifier<'ast>),
}

impl<'ast> fmt::Display for SelfKeywordOrIdentifier<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SelfKeywordOrIdentifier::SelfKeyword(self_keyword) => write!(f, "{}", self_keyword),
            SelfKeywordOrIdentifier::Identifier(identifier) => write!(f, "{}", identifier),
        }
    }
}
