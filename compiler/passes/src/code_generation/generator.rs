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

use crate::{CallGraph, StructGraph, SymbolTable, TypeTable};

use leo_ast::{Function, Program, ProgramId};
use leo_span::Symbol;

use indexmap::IndexMap;

pub struct CodeGenerator<'a> {
    /// The symbol table for the program.
    pub(crate) symbol_table: &'a SymbolTable,
    /// A mapping between expressions and their types.
    pub(crate) type_table: &'a TypeTable,
    /// The struct dependency graph for the program.
    pub(crate) struct_graph: &'a StructGraph,
    /// The call graph for the program.
    pub(crate) _call_graph: &'a CallGraph,
    /// A counter to track the next available register.
    pub(crate) next_register: u64,
    /// Reference to the current function.
    pub(crate) current_function: Option<&'a Function>,
    /// Mapping of variables to registers.
    pub(crate) variable_mapping: IndexMap<&'a Symbol, String>,
    /// Mapping of composite names to a tuple containing metadata associated with the name.
    /// The first element of the tuple indicate whether the composite is a record or not.
    /// The second element of the tuple is a string modifier used for code generation.
    pub(crate) composite_mapping: IndexMap<&'a Symbol, (bool, String)>,
    /// Mapping of global identifiers to their associated names.
    pub(crate) global_mapping: IndexMap<&'a Symbol, String>,
    /// Are we traversing a transition function?
    pub(crate) is_transition_function: bool,
    /// Are we traversing a finalize block?
    pub(crate) in_finalize: bool,
    // TODO (@d0cd): There are a temporary solution to be compatible with futures introduced in Aleo instructions.
    // The registers containing futures produced in the current transition.
    pub(crate) futures: Vec<(String, String)>,
    // A reference to program. This is needed to look up external programs.
    pub(crate) program: &'a Program,
    // The program ID of the current program.
    pub(crate) program_id: Option<ProgramId>,
}

impl<'a> CodeGenerator<'a> {
    /// Initializes a new `CodeGenerator`.
    pub fn new(
        symbol_table: &'a SymbolTable,
        type_table: &'a TypeTable,
        struct_graph: &'a StructGraph,
        _call_graph: &'a CallGraph,
        program: &'a Program,
    ) -> Self {
        // Initialize variable mapping.
        Self {
            symbol_table,
            type_table,
            struct_graph,
            _call_graph,
            next_register: 0,
            current_function: None,
            variable_mapping: IndexMap::new(),
            composite_mapping: IndexMap::new(),
            global_mapping: IndexMap::new(),
            is_transition_function: false,
            in_finalize: false,
            futures: Vec::new(),
            program,
            program_id: None,
        }
    }
}
