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

use crate::{Assigner, SymbolTable, TypeTable};

use leo_ast::{Ast, CallGraph, CompositeGraph, NetworkName, NodeBuilder};
use leo_errors::{Handler, LeoWarning, Result};

use std::collections::HashSet;

/// Contains data shared by many compiler passes.
#[derive(Default)]
pub struct CompilerState {
    /// The Abstract Syntax Tree.
    pub ast: Ast,
    /// The error Handler.
    pub handler: Handler,
    /// Maps node IDs to types.
    pub type_table: TypeTable,
    /// Creates incrementing node IDs.
    pub node_builder: NodeBuilder,
    /// Creates unique symbols and definitions.
    pub assigner: Assigner,
    /// Contains data about the variables and other entities in the program.
    pub symbol_table: SymbolTable,
    /// A graph of which composite refer to each other.
    pub composite_graph: CompositeGraph,
    /// A graph of which functions call each other.
    pub call_graph: CallGraph,
    /// A set of the warnings collected. This is used to make sure we don't emit the same exact warning twice.
    pub warnings: HashSet<LeoWarning>,
    /// Is this a test program?
    pub is_test: bool,
    /// The network.
    pub network: NetworkName,
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
