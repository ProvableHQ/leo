// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{Block, Expression, Node, Statement};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ConditionalStatement {
    pub condition: Expression,
    pub block: Block,
    pub next: Option<Box<Statement>>,
    pub span: Span,
}

impl fmt::Display for ConditionalStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "if ({}) {}", self.condition, self.block)?;
        match self.next.clone() {
            Some(n_or_e) => write!(f, " else {}", n_or_e),
            None => write!(f, ""),
        }
    }
}

impl Node for ConditionalStatement {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
