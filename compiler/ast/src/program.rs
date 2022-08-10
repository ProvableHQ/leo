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

use crate::{Circuit, Function, FunctionInput, Identifier};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores the Leo program abstract syntax tree.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
    /// The name of the program.
    pub name: String,
    /// The network of the program.
    pub network: String,
    /// Expected main function inputs.
    /// Empty after parsing.
    pub expected_input: Vec<FunctionInput>,
    /// A map from import names to import definitions.
    pub imports: IndexMap<Identifier, Program>,
    /// A map from circuit names to circuit definitions.
    pub circuits: IndexMap<Identifier, Circuit>,
    /// A map from function names to function definitions.
    pub functions: IndexMap<Identifier, Function>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (id, _import) in self.imports.iter() {
            writeln!(f, "import {}.leo;", id)?;
        }
        for (_, function) in self.functions.iter() {
            function.fmt(f)?;
            writeln!(f,)?;
        }
        for (_, circuit) in self.circuits.iter() {
            circuit.fmt(f)?;
            writeln!(f,)?;
        }
        write!(f, "")
    }
}

impl Default for Program {
    /// Constructs an empty program node.
    fn default() -> Self {
        Self {
            name: String::new(),
            network: String::new(),
            expected_input: vec![],
            imports: IndexMap::new(),
            functions: IndexMap::new(),
            circuits: IndexMap::new(),
        }
    }
}
