// Copyright (C) 2019-2024 Aleo Systems Inc.
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


pub mod program_scope;
pub use program_scope::*;

pub mod comment;
pub use comment::*;

use leo_span::{Span, Symbol};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::Stub;

/// Stores the Leo program abstract syntax tree.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Program {
   
    /// A map from import names to import definitions.
    pub imports: IndexMap<Symbol, (Program, Span, Vec<Comment>, Comment)>,
    /// A map from program stub names to program stub scopes.
    pub stubs: IndexMap<Symbol, (Stub, usize)>,
    /// A map from program names to program scopes.
    pub program_scopes: IndexMap<Symbol, (ProgramScope, Vec<Comment>)>,
    
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (id, import) in self.imports.iter() {
            for comment in &import.2 {
                comment.fmt(f)?;
            }
            if import.3 == Comment::None {
                writeln!(f, "import {id}.aleo;")?;
            }else {
                write!(f, "import {id}.aleo; ")?;
            }
            import.3.fmt(f)?;
        }
        for (_, stub) in self.stubs.iter() {
            stub.0.fmt(f)?;
            writeln!(f,)?;
        }
        for (_, program_scope) in self.program_scopes.iter() {
            for comment in &program_scope.1 {
                comment.fmt(f)?;
            }
            program_scope.0.fmt(f)?;
        }
        Ok(())
    }
}

impl Default for Program {
    /// Constructs an empty program node.
    fn default() -> Self {
        Self { imports: IndexMap::new(), stubs: IndexMap::new(), program_scopes: IndexMap::new() }
    }
}
