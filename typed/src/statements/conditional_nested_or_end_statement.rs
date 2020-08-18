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

use crate::{ConditionalStatement, Statement};
use leo_ast::statements::ConditionalNestedOrEndStatement as AstConditionalNestedOrEndStatement;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionalNestedOrEndStatement {
    Nested(Box<ConditionalStatement>),
    End(Vec<Statement>),
}

impl<'ast> From<AstConditionalNestedOrEndStatement<'ast>> for ConditionalNestedOrEndStatement {
    fn from(statement: AstConditionalNestedOrEndStatement<'ast>) -> Self {
        match statement {
            AstConditionalNestedOrEndStatement::Nested(nested) => {
                ConditionalNestedOrEndStatement::Nested(Box::new(ConditionalStatement::from(*nested)))
            }
            AstConditionalNestedOrEndStatement::End(statements) => ConditionalNestedOrEndStatement::End(
                statements
                    .into_iter()
                    .map(|statement| Statement::from(statement))
                    .collect(),
            ),
        }
    }
}

impl fmt::Display for ConditionalNestedOrEndStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConditionalNestedOrEndStatement::Nested(ref nested) => write!(f, "else {}", nested),
            ConditionalNestedOrEndStatement::End(ref statements) => {
                write!(f, "else {{\n")?;
                for statement in statements.iter() {
                    write!(f, "\t\t{}\n", statement)?;
                }
                write!(f, "\t}}")
            }
        }
    }
}
