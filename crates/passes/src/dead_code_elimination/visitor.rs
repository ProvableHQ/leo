// Copyright (C) 2019-2026 Provable Inc.
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

use crate::{CompilerState, expression_can_be_discarded};

use leo_ast::Expression;
use leo_span::Symbol;

use indexmap::IndexSet;

pub struct DeadCodeEliminatingVisitor<'a> {
    pub state: &'a mut CompilerState,

    /// The set of used variables in the current function body.
    pub used_variables: IndexSet<Symbol>,

    /// The name of the program currently being processed.
    pub unit_name: Symbol,
}

impl DeadCodeEliminatingVisitor<'_> {
    pub fn can_discard(&self, expr: &Expression) -> bool {
        expression_can_be_discarded(expr, self.state)
    }
}
