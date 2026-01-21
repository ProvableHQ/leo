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

//! A stub contains function templates as well as definitions for mappings, composites, records, and constants.

pub mod function_stub;
pub use function_stub::*;

use crate::{Composite, ConstDeclaration, Identifier, Indent, Mapping, NodeID, Program, ProgramId};
use indexmap::IndexSet;
use leo_span::{Span, Symbol};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Stub {
    /// A dependency that is a Leo program parsed into an AST.
    FromLeo {
        program: Program,
        parents: IndexSet<Symbol>, // These are the names of all the programs that import this dependency.
    },
    /// A dependency that is an Aleo program.
    FromAleo {
        program: AleoProgram,
        parents: IndexSet<Symbol>, // These are the names of all the programs that import this dependency.
    },
}

impl Stub {
    /// Returns the programs that this stub imports.
    pub fn imports(&self) -> Box<dyn Iterator<Item = &Symbol> + '_> {
        match self {
            Stub::FromLeo { program, .. } => Box::new(program.imports.keys()),
            Stub::FromAleo { program, .. } => Box::new(program.imports.iter().map(|id| &id.name.name)),
        }
    }

    /// Inserts the given program name as a parent for this stub, implying that the stub is
    /// imported by this parent.
    pub fn add_parent(&mut self, parent: Symbol) {
        match self {
            Stub::FromLeo { parents, .. } | Stub::FromAleo { parents, .. } => {
                parents.insert(parent);
            }
        }
    }
}

impl fmt::Display for Stub {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stub::FromLeo { program, .. } => write!(f, "{program}"),
            Stub::FromAleo { program, .. } => write!(f, "{program}"),
        }
    }
}

impl From<Program> for Stub {
    fn from(program: Program) -> Self {
        Stub::FromLeo { program, parents: IndexSet::new() }
    }
}

impl From<AleoProgram> for Stub {
    fn from(program: AleoProgram) -> Self {
        Stub::FromAleo { program, parents: IndexSet::new() }
    }
}

/// Stores the Leo stub abstract syntax tree.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AleoProgram {
    /// A vector of imported programs.
    pub imports: Vec<ProgramId>,
    /// The stub id
    pub stub_id: ProgramId,
    /// A vector of const definitions.
    pub consts: Vec<(Symbol, ConstDeclaration)>,
    /// A vector of composite definitions.
    pub composites: Vec<(Symbol, Composite)>,
    /// A vector of mapping definitions.
    pub mappings: Vec<(Symbol, Mapping)>,
    /// A vector of function stub definitions.
    pub functions: Vec<(Symbol, FunctionStub)>,
    /// The span associated with the stub.
    pub span: Span,
}

impl Default for AleoProgram {
    /// Constructs an empty program stub
    fn default() -> Self {
        Self {
            imports: Vec::new(),
            stub_id: ProgramId {
                name: Identifier::new(Symbol::intern(""), NodeID::default()),
                network: Identifier::new(Symbol::intern(""), NodeID::default()),
            },
            consts: Vec::new(),
            composites: Vec::new(),
            mappings: Vec::new(),
            functions: Vec::new(),
            span: Span::default(),
        }
    }
}

impl fmt::Display for AleoProgram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "stub {} {{", self.stub_id)?;
        for import in self.imports.iter() {
            writeln!(f, "    import {import};")?;
        }
        for (_, mapping) in self.mappings.iter() {
            writeln!(f, "{};", Indent(mapping))?;
        }
        for (_, composite) in self.composites.iter() {
            writeln!(f, "{}", Indent(composite))?;
        }
        for (_, function) in self.functions.iter() {
            writeln!(f, "{}", Indent(function))?;
        }
        write!(f, "}}")
    }
}
