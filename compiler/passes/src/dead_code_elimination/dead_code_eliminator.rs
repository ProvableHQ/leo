// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use leo_ast::NodeBuilder;
use leo_span::Symbol;

use indexmap::IndexSet;

pub struct DeadCodeEliminator<'a> {
    /// A counter to generate unique node IDs.
    pub(crate) node_builder: &'a NodeBuilder,
    /// The set of used variables in the current function body.
    pub(crate) used_variables: IndexSet<Symbol>,
    /// Whether or not the variables are necessary.
    pub(crate) is_necessary: bool,
}

impl<'a> DeadCodeEliminator<'a> {
    /// Initializes a new `DeadCodeEliminator`.
    pub fn new(node_builder: &'a NodeBuilder) -> Self {
        Self { node_builder, used_variables: Default::default(), is_necessary: false }
    }
}
