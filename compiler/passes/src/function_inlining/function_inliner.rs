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

use crate::{Assigner, AssignmentRenamer, CallGraph, TypeTable};

use leo_ast::{Function, NodeBuilder};
use leo_span::Symbol;

pub struct FunctionInliner<'a> {
    /// A counter used to create unique NodeIDs.
    pub(crate) node_builder: &'a NodeBuilder,
    /// The call graph for the program.
    pub(crate) call_graph: &'a CallGraph,
    /// A wrapper around an Assigner used to create unique variable assignments.
    pub(crate) assignment_renamer: AssignmentRenamer<'a>,
    /// A mapping between node IDs and their types.
    pub(crate) type_table: &'a TypeTable,
    /// A map of reconstructed functions in the current program scope.
    pub(crate) reconstructed_functions: Vec<(Symbol, Function)>,
}

impl<'a> FunctionInliner<'a> {
    /// Initializes a new `FunctionInliner`.
    pub fn new(
        node_builder: &'a NodeBuilder,
        call_graph: &'a CallGraph,
        assigner: &'a Assigner,
        type_table: &'a TypeTable,
    ) -> Self {
        Self {
            node_builder,
            call_graph,
            assignment_renamer: AssignmentRenamer::new(assigner),
            reconstructed_functions: Default::default(),
            type_table,
        }
    }
}
