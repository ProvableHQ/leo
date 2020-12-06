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

use crate::{Block, ConditionalStatement};
use leo_grammar::statements::ConditionalNestedOrEndStatement as GrammarConditionalNestedOrEndStatement;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionalNestedOrEndStatement {
    Nested(Box<ConditionalStatement>),
    End(Block),
}

impl<'ast> From<GrammarConditionalNestedOrEndStatement<'ast>> for ConditionalNestedOrEndStatement {
    fn from(statement: GrammarConditionalNestedOrEndStatement<'ast>) -> Self {
        match statement {
            GrammarConditionalNestedOrEndStatement::Nested(nested) => {
                ConditionalNestedOrEndStatement::Nested(Box::new(ConditionalStatement::from(*nested)))
            }
            GrammarConditionalNestedOrEndStatement::End(block) => {
                ConditionalNestedOrEndStatement::End(Block::from(block))
            }
        }
    }
}

impl fmt::Display for ConditionalNestedOrEndStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConditionalNestedOrEndStatement::Nested(ref nested) => write!(f, "else {}", nested),
            ConditionalNestedOrEndStatement::End(ref block) => write!(f, "else {}", block),
        }
    }
}
