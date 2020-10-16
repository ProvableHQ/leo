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

use crate::{CircuitError, Function, ResolvedNode};
use leo_static_check::{Attribute, CircuitType, ParameterType, SymbolTable, Type};
use leo_typed::{circuit::Circuit as UnresolvedCircuit, identifier::Identifier, CircuitMember};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A circuit in the resolved syntax tree.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Circuit {
    /// The user-defined type of this circuit.
    pub type_: CircuitType,

    /// The circuit member functions.
    pub functions: HashMap<Identifier, Function>,
}

impl ResolvedNode for Circuit {
    type Error = CircuitError;
    type UnresolvedNode = UnresolvedCircuit;

    ///
    /// Return a new `Circuit` from a given `UnresolvedCircuit`.
    ///
    /// Performs a lookup in the given symbol table if the circuit contains user-defined types.
    ///
    fn resolve(table: &mut SymbolTable, unresolved: Self::UnresolvedNode) -> Result<Self, Self::Error> {
        let identifier = unresolved.circuit_name;
        let type_ = table.get_circuit(&identifier.name).unwrap().clone();

        // Create circuit context
        let mut child_table = SymbolTable::new(Some(Box::new(table.clone())));

        // Create self variable
        let self_key = "self".to_owned();
        let self_variable = ParameterType {
            identifier: identifier.clone(),
            type_: Type::Circuit(identifier.clone()),
            attributes: vec![Attribute::Mutable],
        };
        child_table.insert_variable(self_key, self_variable);

        // Insert circuit functions into symbol table
        for function in type_.functions.iter() {
            let function_key = function.function.identifier.clone();
            let function_type = function.function.clone();

            child_table.insert_function(function_key, function_type);
        }

        // Resolve all circuit functions
        let mut functions = HashMap::new();

        for member in unresolved.members {
            match member {
                CircuitMember::CircuitVariable(_, _, _) => {}
                CircuitMember::CircuitFunction(_, function) => {
                    let identifier = function.identifier.clone();
                    let function_resolved = Function::resolve(&mut child_table.clone(), function)?;

                    functions.insert(identifier, function_resolved);
                }
            }
        }

        Ok(Circuit { type_, functions })
    }
}
