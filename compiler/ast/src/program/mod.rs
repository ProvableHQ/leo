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

//! A Leo program consists of import statements and program scopes.

mod program_id;
pub use program_id::*;

mod program_scope;
pub use program_scope::*;

use leo_span::{Span, Symbol};

use crate::{Module, Stub};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores the Leo program abstract syntax tree.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
    /// A map from module paths to module definitions.
    pub modules: IndexMap<Vec<Symbol>, Module>,
    /// A map from import names to import definitions.
    pub imports: IndexMap<Symbol, (Program, Span)>,
    /// A map from program stub names to program stub scopes.
    pub stubs: IndexMap<Symbol, Stub>,
    /// A map from program names to program scopes.
    pub program_scopes: IndexMap<Symbol, ProgramScope>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (_, module) in self.modules.iter() {
            writeln!(f, "{module}")?;
        }
        for (id, _import) in self.imports.iter() {
            writeln!(f, "import {id}.aleo;")?;
        }
        for (_, stub) in self.stubs.iter() {
            writeln!(f, "{stub}")?;
        }
        for (_, program_scope) in self.program_scopes.iter() {
            writeln!(f, "{program_scope}")?;
        }
        Ok(())
    }
}

impl Default for Program {
    /// Constructs an empty program node.
    fn default() -> Self {
        Self {
            modules: IndexMap::new(),
            imports: IndexMap::new(),
            stubs: IndexMap::new(),
            program_scopes: IndexMap::new(),
        }
    }
}
