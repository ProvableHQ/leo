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

//! A Leo program scope consists of const, composite, function, and mapping definitions.

use crate::{
    Composite,
    ConstDeclaration,
    Constructor,
    Function,
    Indent,
    Interface,
    Mapping,
    ProgramId,
    StorageVariable,
};

use leo_span::{Span, Symbol};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores the Leo program scope abstract syntax tree.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProgramScope {
    /// The program id of the program scope.
    pub program_id: ProgramId,
    /// The interface this program inherits from, if any
    pub parent: Option<Symbol>,
    /// A vector of const definitions.
    pub consts: Vec<(Symbol, ConstDeclaration)>,
    /// A vector of composite definitions.
    pub composites: Vec<(Symbol, Composite)>,
    /// A vector of mapping definitions.
    pub mappings: Vec<(Symbol, Mapping)>,
    /// A vector of storage variable definitions.
    pub storage_variables: Vec<(Symbol, StorageVariable)>,
    /// A vector of function definitions.
    pub functions: Vec<(Symbol, Function)>,
    /// A vector of interface definitions.
    pub interfaces: Vec<(Symbol, Interface)>,
    /// An optional constructor.
    pub constructor: Option<Constructor>,
    /// The span associated with the program scope.
    pub span: Span,
}

impl fmt::Display for ProgramScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "program {} {{", self.program_id)?;
        for (_, const_decl) in self.consts.iter() {
            writeln!(f, "{};", Indent(const_decl))?;
        }
        if let Some(constructor) = &self.constructor {
            writeln!(f, "{}", Indent(constructor))?;
        }
        for (_, composite_) in self.composites.iter() {
            writeln!(f, "{}", Indent(composite_))?;
        }
        for (_, mapping) in self.mappings.iter() {
            writeln!(f, "{};", Indent(mapping))?;
        }
        for (_, function) in self.functions.iter().filter(|f| f.1.variant.is_entry()) {
            writeln!(f, "{}", Indent(function))?;
        }
        writeln!(f, "}}")?;

        for (_, function) in self.functions.iter().filter(|f| !f.1.variant.is_entry()) {
            writeln!(f, "{}", Indent(function))?;
        }
        Ok(())
    }
}
