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

use crate::{Alias, Circuit, CircuitMember, DefinitionStatement, Function, FunctionInput, Identifier, ImportStatement};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores the Leo program abstract syntax tree.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub name: String,
    pub expected_input: Vec<FunctionInput>,
    pub import_statements: Vec<ImportStatement>,
    #[serde(with = "crate::common::imported_modules")]
    pub imports: IndexMap<Vec<String>, Program>,
    pub aliases: IndexMap<Identifier, Alias>,
    pub circuits: IndexMap<Identifier, Circuit>,
    #[serde(with = "crate::common::global_consts_json")]
    pub global_consts: IndexMap<Vec<Identifier>, DefinitionStatement>,
    pub functions: IndexMap<Identifier, Function>,
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
        for (_, alias) in self.aliases.iter() {
            alias.fmt(f)?;
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
            aliases: IndexMap::new(),
            circuits: IndexMap::new(),
            global_consts: IndexMap::new(),
            functions: IndexMap::new(),
        }
    }

    pub fn set_core_mapping(&self) {
        for (_, circuit) in self.circuits.iter() {
            for member in circuit.members.iter() {
                if let CircuitMember::CircuitFunction(function) = member {
                    let mapping = match function.identifier.name.as_ref() {
                        "to_bits_le" => Some(format!("{}_to_bits_le", circuit.circuit_name.name.to_string())),
                        "from_bits_le" => Some(format!("{}_from_bits_le", circuit.circuit_name.name.to_string())),
                        "to_bytes_le" => Some(format!("{}_to_bytes_le", circuit.circuit_name.name.to_string())),
                        "from_bytes_le" => Some(format!("{}_from_bytes_le", circuit.circuit_name.name.to_string())),
                        _ => Some(function.identifier.name.to_string()),
                    };

                    function.core_mapping.replace(mapping);
                }
            }
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
