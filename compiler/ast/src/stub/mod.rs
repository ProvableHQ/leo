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

//! A stub contains function templates as well as definitions for mappings, structs, records, and constants.

pub mod function_stub;
pub use function_stub::*;

use crate::{Composite, ConstDeclaration, Identifier, Mapping, NodeID, ProgramId};
use leo_span::{Span, Symbol};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores the Leo stub abstract syntax tree.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Stub {
    /// A vector of imported programs.
    pub imports: Vec<ProgramId>,
    /// The stub id
    pub stub_id: ProgramId,
    /// A vector of const definitions.
    pub consts: Vec<(Symbol, ConstDeclaration)>,
    /// A vector of struct definitions.
    pub structs: Vec<(Symbol, Composite)>,
    /// A vector of mapping definitions.
    pub mappings: Vec<(Symbol, Mapping)>,
    /// A vector of function stub definitions.
    pub functions: Vec<(Symbol, FunctionStub)>,
    /// The span associated with the stub.
    pub span: Span,
}

impl Default for Stub {
    /// Constructs an empty program stub
    fn default() -> Self {
        Self {
            imports: Vec::new(),
            stub_id: ProgramId {
                name: Identifier::new(Symbol::intern(""), NodeID::default()),
                network: Identifier::new(Symbol::intern(""), NodeID::default()),
            },
            consts: Vec::new(),
            structs: Vec::new(),
            mappings: Vec::new(),
            functions: Vec::new(),
            span: Span::default(),
        }
    }
}

impl fmt::Display for Stub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "stub {} {{", self.stub_id)?;
        for import in self.imports.iter() {
            writeln!(f, "    import {import}")?;
        }
        for (_, mapping) in self.mappings.iter() {
            writeln!(f, "    {mapping}")?;
        }
        for (_, struct_) in self.structs.iter() {
            writeln!(f, "    {struct_}")?;
        }
        for (_, function) in self.functions.iter() {
            writeln!(f, "    {function}")?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}
