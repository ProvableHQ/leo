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

use crate::{Assigner, CallGraph, SymbolTable};

pub struct FunctionInliner<'a> {
    /// The `SymbolTable` of the program.
    pub(crate) symbol_table: &'a SymbolTable,
    /// The call graph for the program.
    pub(crate) call_graph: &'a CallGraph,
    /// An struct used to construct (unique) variable names.
    pub(crate) assigner: Assigner,
}

impl<'a> FunctionInliner<'a> {
    /// Initializes a new `FunctionInliner`.
    pub fn new(symbol_table: &'a SymbolTable, call_graph: &'a CallGraph, assigner: Assigner) -> Self {
        Self {
            symbol_table,
            call_graph,
            assigner,
        }
    }
}
