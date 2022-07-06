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

use std::collections::HashMap;

pub struct CodeGenerator<'a> {
    _handler: &'a Handler,
    /// A counter to track the next available register.
    pub(crate) next_register: u64,
    /// Reference to the current function.
    pub(crate) current_function: Option<&'a Function>,
    /// Mapping of variables to registers.
    pub(crate) variable_mapping: HashMap<&'a Symbol, String>,
    /// Mapping of composite names to type (`circuit` or `register`).
    pub(crate) composite_mapping: HashMap<&'a Symbol, String>,
}

impl<'a> CodeGenerator<'a> {
    /// Initializes a new `CodeGenerator`.
    pub fn new(handler: &'a Handler) -> Self {
        Self {
            _handler: handler,
            next_register: 0,
            current_function: None,
            variable_mapping: HashMap::new(),
            composite_mapping: HashMap::new(),
        }
    }
}
