// Copyright (C) 2019-2021 Aleo Systems Inc.
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

//! A Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of ast statements and expressions.

use crate::{Circuit, DefinitionStatement, Function, FunctionInput, ImportStatement};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores the Leo program abstract syntax tree.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub name: String,
    pub expected_input: Vec<FunctionInput>,
    pub import_statements: Vec<ImportStatement>,
    pub imports: IndexMap<String, Program>,
    pub circuits: IndexMap<String, Circuit>,
    pub global_consts: IndexMap<String, DefinitionStatement>,
    pub functions: IndexMap<String, Function>,
}

impl AsRef<Program> for Program {
    fn as_ref(&self) -> &Program {
        self
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for import in self.import_statements.iter() {
            import.fmt(f)?;
            writeln!(f,)?;
        }
        writeln!(f,)?;
        for (_, import) in self.imports.iter() {
            import.fmt(f)?;
            writeln!(f,)?;
        }
        writeln!(f,)?;
        for (_, circuit) in self.circuits.iter() {
            circuit.fmt(f)?;
            writeln!(f,)?;
        }
        writeln!(f,)?;
        for (_, function) in self.functions.iter() {
            function.fmt(f)?;
            writeln!(f,)?;
        }
        write!(f, "")
    }
}

impl Program {
    pub fn new(name: String) -> Self {
        Self {
            name,
            expected_input: vec![],
            import_statements: vec![],
            imports: IndexMap::new(),
            circuits: IndexMap::new(),
            global_consts: IndexMap::new(),
            functions: IndexMap::new(),
        }
    }

    pub fn set_core_mapping(&self, mapping: &str) {
        for (_, circuit) in self.circuits.iter() {
            circuit.core_mapping.replace(Some(mapping.to_string()));
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}
