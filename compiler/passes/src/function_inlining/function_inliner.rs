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

use crate::{Assigner, CallGraph, StaticSingleAssigner, SymbolTable};

use leo_ast::Function;
use leo_span::Symbol;

use indexmap::IndexMap;

pub struct FunctionInliner<'a> {
    /// The call graph for the program.
    pub(crate) call_graph: &'a CallGraph,
    /// A static single assigner used to create unique variable assignments.
    pub(crate) static_single_assigner: StaticSingleAssigner<'a>,
    /// A map of reconstructed functions in the current program scope.
    pub(crate) reconstructed_functions: IndexMap<Symbol, Function>,
}

impl<'a> FunctionInliner<'a> {
    /// Initializes a new `FunctionInliner`.
    pub fn new(symbol_table: &'a SymbolTable, call_graph: &'a CallGraph, assigner: Assigner) -> Self {
        Self {
            call_graph,
            // Note: Since we are using the `StaticSingleAssigner` to create unique variable assignments over `BlockStatement`s, we do not need to pass in
            // an "accurate" symbol table. This assumption only holds if function inlining occurs after flattening.
            // TODO: Refactor out the unique renamer from the static single assigner and use it instead.
            static_single_assigner: StaticSingleAssigner::new(symbol_table, assigner),
            reconstructed_functions: Default::default(),
        }
    }
}
