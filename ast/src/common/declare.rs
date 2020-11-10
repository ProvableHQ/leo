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

use leo_grammar::common::Declare as GrammarDeclare;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Declare {
    Const,
    Let,
}

impl<'ast> From<GrammarDeclare> for Declare {
    fn from(declare: GrammarDeclare) -> Self {
        match declare {
            GrammarDeclare::Const(_) => Declare::Const,
            GrammarDeclare::Let(_) => Declare::Let,
        }
    }
}

impl fmt::Display for Declare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Declare::Const => write!(f, "const"),
            Declare::Let => write!(f, "let"),
        }
    }
}
