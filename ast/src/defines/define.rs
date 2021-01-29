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

use crate::{Expression, Identifier};
use leo_grammar::defines::Define as GrammarDefine;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Define {
    pub name: Identifier,
    pub expression: Expression,
}

impl<'ast> From<GrammarDefine<'ast>> for Define {
    fn from(define: GrammarDefine<'ast>) -> Self {
        Self {
            name: Identifier::from(define.identifier),
            expression: Expression::from(define.expression),
        }
    }
}

impl Define {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "define {} ", self.name)
    }
}

impl fmt::Debug for Define {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
