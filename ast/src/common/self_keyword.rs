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

use crate::{Node, Span};
use leo_grammar::common::SelfKeyword as GrammarSelfKeyword;

use serde::{Deserialize, Serialize};
use std::fmt;

/// The `self` keyword can view circuit values inside of a circuit function.
/// Circuit values cannot be modified. To modify values use the `mut self` [MutSelfKeyword].
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SelfKeyword {
    pub span: Span,
}

impl<'ast> From<GrammarSelfKeyword<'ast>> for SelfKeyword {
    fn from(grammar: GrammarSelfKeyword<'ast>) -> Self {
        Self {
            span: Span::from(grammar.span),
        }
    }
}

impl fmt::Display for SelfKeyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "self")
    }
}

impl Node for SelfKeyword {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
