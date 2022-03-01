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

//! A Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of ast statements and expressions.

use crate::{Alias, Circuit, CircuitMember, DefinitionStatement, Function, FunctionInput, Identifier, ImportStatement};

use leo_span::{sym, Symbol};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores the Leo program abstract syntax tree.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
    /// The name of the program.
    /// Empty after parsing.
    pub name: String,
    /// Expected main function inputs.
    /// Empty after parsing.
    pub expected_input: Vec<FunctionInput>,
    /// The collected import statements.
    pub import_statements: Vec<ImportStatement>,
    #[serde(with = "crate::common::imported_modules")]
    /// A map from paths to injected programs.
    pub imports: IndexMap<Vec<Symbol>, Program>,
    /// A map from alias names to type aliases.
    pub aliases: IndexMap<Identifier, Alias>,
    /// A map from circuit names to circuit definitions.
    pub circuits: IndexMap<Identifier, Circuit>,
    /// A map from constant names to their definitions.
    #[serde(with = "crate::common::global_consts_json")]
    pub global_consts: IndexMap<Vec<Identifier>, DefinitionStatement>,
    /// A map from function names to their definitions.
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
    /// Constructs an empty program with `name`.
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

    /// Handles all internal annotations like `@CoreFunction` and `@AlwaysConst`.
    pub fn handle_internal_annotations(&mut self) {
        self.circuits
            .iter_mut()
            .flat_map(|(_, circuit)| &mut circuit.members)
            .filter_map(|member| {
                if let CircuitMember::CircuitFunction(function) = member {
                    Some(function)
                } else {
                    None
                }
            })
            .for_each(|function| {
                function.annotations.retain(|name, core_map| {
                    match *name {
                        sym::CoreFunction => {
                            let new = core_map.arguments.get(0).copied().or(Some(function.identifier.name));
                            function.core_mapping.replace(new);
                            false
                        }
                        sym::AlwaysConst => {
                            function.const_ = true;
                            false
                        }
                        // Could still be a valid annotation.
                        // Carry on and let ASG handle.
                        _ => true,
                    }
                })
            });
    }

    /// Extract the name of the program.
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    /// Sets the name of the program.
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}
