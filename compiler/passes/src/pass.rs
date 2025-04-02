// Copyright (C) 2019-2025 Provable Inc.
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

use crate::{Assigner, CallGraph, StructGraph, SymbolTable, TypeTable};

use leo_ast::{Ast, NodeBuilder};
use leo_errors::{Result, emitter::Handler};

#[derive(Default)]
pub struct CompilerState {
    pub ast: Ast,
    pub handler: Handler,
    pub type_table: TypeTable,
    pub node_builder: NodeBuilder,
    pub assigner: Assigner,
    pub symbol_table: SymbolTable,
    pub struct_graph: StructGraph,
    pub call_graph: CallGraph,
}

/// A compiler pass.
///
/// Every pass has access to `CompilerState`, and may also specify
/// an `Input` and `Output`.
pub trait Pass {
    type Input;
    type Output;

    const NAME: &str;

    /// Runs the compiler pass.
    fn do_pass(input: Self::Input, state: &mut CompilerState) -> Result<Self::Output>;
}
