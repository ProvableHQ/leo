// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use leo_ast::Function;
use leo_errors::emitter::Handler;
use leo_span::Symbol;

use indexmap::IndexMap;

pub struct CodeGenerator<'a> {
    _handler: &'a Handler,
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
    /// Are we traversing a program function?
    /// A "program function" is a function that can be invoked by a user or another program.
    pub(crate) is_program_function: bool,
}

impl<'a> CodeGenerator<'a> {
    /// Initializes a new `CodeGenerator`.
    pub fn new(handler: &'a Handler) -> Self {
        Self {
            _handler: handler,
            next_register: 0,
            current_function: None,
            variable_mapping: IndexMap::new(),
            composite_mapping: IndexMap::new(),
            is_program_function: false,
        }
    }
}
