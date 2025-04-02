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

use crate::CompilerState;

use leo_ast::{Function, Program, ProgramId, Variant};
use leo_span::Symbol;

use indexmap::IndexMap;

pub struct CodeGeneratingVisitor<'a> {
    pub state: &'a CompilerState,
    /// A counter to track the next available register.
    pub next_register: u64,
    /// Reference to the current function.
    pub current_function: Option<&'a Function>,
    /// Mapping of variables to registers.
    pub variable_mapping: IndexMap<Symbol, String>,
    /// Mapping of composite names to a tuple containing metadata associated with the name.
    /// The first element of the tuple indicate whether the composite is a record or not.
    /// The second element of the tuple is a string modifier used for code generation.
    pub composite_mapping: IndexMap<Symbol, (bool, String)>,
    /// Mapping of global identifiers to their associated names.
    pub global_mapping: IndexMap<Symbol, String>,
    /// The variant of the function we are currently traversing.
    pub variant: Option<Variant>,
    /// A reference to program. This is needed to look up external programs.
    pub program: &'a Program,
    /// The program ID of the current program.
    pub program_id: Option<ProgramId>,
    /// A reference to the finalize caller.
    pub finalize_caller: Option<Symbol>,
    /// A counter to track the next available label.
    pub next_label: u64,
    /// The depth of the current conditional block.
    pub conditional_depth: u64,
}
