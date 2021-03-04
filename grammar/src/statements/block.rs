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

use crate::{ast::Rule, statements::Statement, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::block))]
pub struct Block<'ast> {
    pub statements: Vec<Statement<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for Block<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{")?;
        if self.statements.is_empty() {
            writeln!(f, "\t")?;
        } else {
            self.statements
                .iter()
                .try_for_each(|statement| writeln!(f, "\t{}", statement))?;
        }
        write!(f, "}}")
    }
}
