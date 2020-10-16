// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{Circuit, Function, ProgramError, ResolvedNode, TestFunction};
use leo_static_check::SymbolTable;
use leo_typed::{Identifier, Program as UnresolvedProgram};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static MAIN_FUNCTION_NAME: &str = "main";

/// The root of the resolved syntax tree.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Program {
    // pub imports: Vec<Import>,
    pub circuits: HashMap<Identifier, Circuit>,
    pub functions: HashMap<Identifier, Function>,
    pub tests: HashMap<Identifier, TestFunction>,
}

impl ResolvedNode for Program {
    type Error = ProgramError;
    type UnresolvedNode = UnresolvedProgram;

    ///
    /// Returns a `Program` given an `UnresolvedProgram` AST.
    ///
    /// At each AST node:
    ///    1. Resolve all child AST nodes.
    ///    2. Resolve current AST node.
    ///
    /// Performs a lookup in the given symbol table if the function contains user-defined types.
    ///
    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        let mut circuits = HashMap::new();
        let mut functions = HashMap::new();
        let mut tests = HashMap::new();

        // TODO: Resolve import statements

        // Resolve circuit definitions
        for (identifier, circuit) in unresolved.circuits {
            let resolved_circuit = Circuit::resolve(table, circuit)?;

            circuits.insert(identifier, resolved_circuit);
        }

        // Resolve function statements
        for (identifier, function) in unresolved.functions {
            let mut child_table = SymbolTable::new(Some(Box::new(table.clone())));
            let resolved_function = Function::resolve(&mut child_table, function)?;

            functions.insert(identifier, resolved_function);
        }

        // Resolve tests
        for (identifier, test) in unresolved.tests {
            let mut child_table = SymbolTable::new(Some(Box::new(table.clone())));
            let resolved_test = TestFunction::resolve(&mut child_table, test)?;

            tests.insert(identifier, resolved_test);
        }

        // Look for main function
        // let main = unresolved.functions.into_iter().find(|(identifier, _)| {
        //     identifier.name.eq(MAIN_FUNCTION_NAME)
        // });
        //
        // //TODO: return no main function error
        // let program = match main {
        //     Some((_identifier, function)) => ,
        //     None => unimplemented!("ERROR: main function not found"),
        // }

        Ok(Program {
            circuits,
            functions,
            tests,
        })
    }
}
