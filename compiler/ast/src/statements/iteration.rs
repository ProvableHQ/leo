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

use crate::{Block, Expression, Identifier, Node};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A bounded `for` loop statement `for variable in start .. =? stop block`.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct IterationStatement {
    /// The binding / variable to introduce in the body `block`.
    pub variable: Identifier,
    /// The start of the iteration.
    pub start: Expression,
    /// The end of the iteration, possibly `inclusive`.
    pub stop: Expression,
    /// Whether `stop` is inclusive or not.
    /// Signified with `=` when parsing.
    pub inclusive: bool,
    /// The block to run on each iteration.
    pub block: Block,
    /// The span from `for` to `block`.
    pub span: Span,
}

impl fmt::Display for IterationStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let eq = if self.inclusive { "=" } else { "" };
        write!(
            f,
            "for {} in {}..{}{} {}",
            self.variable, self.start, eq, self.stop, self.block
        )
    }
}

impl Node for IterationStatement {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
