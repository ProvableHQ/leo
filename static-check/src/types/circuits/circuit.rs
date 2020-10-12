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

use crate::{
    types::circuits::{CircuitFunctionType, CircuitVariableType},
    Attribute,
    FunctionType,
    SymbolTable,
    Type,
    TypeError,
};
use leo_typed::{Circuit, CircuitMember, Identifier};

use serde::{Deserialize, Serialize};

/// Stores circuit definition details.
///
/// This type should be added to the circuit symbol table for a resolved syntax tree.
/// This is a user-defined type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CircuitType {
    /// The name of the circuit definition.
    pub identifier: Identifier,

    /// The circuit variables.
    pub variables: Vec<CircuitVariableType>,

    /// The circuit functions.
    pub functions: Vec<CircuitFunctionType>,
}

impl CircuitType {
    ///
    /// Return a new `CircuitType` from a given `Circuit` definition.
    ///
    /// Performs a lookup in the given symbol table if the circuit definition contains
    /// user-defined types.
    ///
    pub fn new(table: &SymbolTable, unresolved: Circuit) -> Result<Self, TypeError> {
        let circuit_identifier = unresolved.circuit_name;
        let mut variables = vec![];
        let mut functions = vec![];

        // Resolve the type of every circuit member.
        for member in unresolved.members {
            match member {
                CircuitMember::CircuitVariable(is_mutable, variable_identifier, type_) => {
                    // Resolve the type of the circuit member variable.
                    let type_ = Type::new_from_circuit(
                        table,
                        type_,
                        circuit_identifier.clone(),
                        circuit_identifier.span.clone(),
                    )?;

                    // Check if the circuit member variable is mutable.
                    let attributes = if is_mutable { vec![Attribute::Mutable] } else { vec![] };

                    // Create a new circuit variable type.
                    let variable = CircuitVariableType {
                        identifier: variable_identifier,
                        type_,
                        attributes,
                    };

                    // Store the circuit variable type.
                    variables.push(variable);
                }
                CircuitMember::CircuitFunction(is_static, function) => {
                    // Resolve the type of the circuit member function.
                    let function_type = FunctionType::from_circuit(table, circuit_identifier.clone(), function)?;

                    // Check if the circuit member function is static.
                    let attributes = if is_static { vec![Attribute::Static] } else { vec![] };

                    // Create a new circuit function type.
                    let function = CircuitFunctionType {
                        function: function_type,
                        attributes,
                    };

                    // Store the circuit function type.
                    functions.push(function);
                }
            }
        }

        // Return a new circuit type.
        Ok(CircuitType {
            identifier: circuit_identifier.clone(),
            variables,
            functions,
        })
    }

    ///
    /// Returns the type of a circuit member.
    ///
    /// If the member is a circuit variable, then the type of the variable is returned.
    /// If the member is a circuit function, then the return type of the function is returned.
    ///
    pub fn member_type(&self, identifier: &Identifier) -> Result<&Type, TypeError> {
        // Check if the circuit member is a circuit variable.
        let matched_variable = self
            .variables
            .iter()
            .find(|variable| variable.identifier.eq(identifier));

        match matched_variable {
            Some(variable) => Ok(&variable.type_),
            None => {
                // Check if the circuit member is a circuit function.
                let matched_function = self
                    .functions
                    .iter()
                    .find(|function| function.function.identifier.eq(identifier));

                match matched_function {
                    Some(function) => Ok(&function.function.output.type_),
                    None => Err(TypeError::undefined_circuit_member(identifier.clone())),
                }
            }
        }
    }
}
