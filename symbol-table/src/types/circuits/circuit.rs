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

use crate::{types::circuits::CircuitVariableType, FunctionType, SymbolTable, Type, TypeError};
use leo_ast::{Circuit, CircuitMember, Identifier, InputValue, Parameter, Span};

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

/// Stores circuit definition details.
///
/// This type should be added to the circuit symbol table for a resolved syntax tree.
/// This is a user-defined type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitType {
    /// The name of the circuit definition.
    pub identifier: Identifier,

    /// The circuit variables.
    pub variables: Vec<CircuitVariableType>,

    /// The circuit functions.
    pub functions: Vec<FunctionType>,
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
                CircuitMember::CircuitVariable(variable_identifier, type_) => {
                    // Resolve the type of the circuit member variable.
                    let type_ = Type::new_from_circuit(
                        table,
                        type_,
                        circuit_identifier.clone(),
                        circuit_identifier.span.clone(),
                    )?;

                    // Create a new circuit variable type.
                    let variable = CircuitVariableType {
                        identifier: variable_identifier,
                        type_,
                        attribute: None,
                    };

                    // Store the circuit variable type.
                    variables.push(variable);
                }
                CircuitMember::CircuitFunction(function) => {
                    // Resolve the type of the circuit member function.
                    let function_type = FunctionType::from_circuit(table, circuit_identifier.clone(), function)?;

                    // Store the circuit function type.
                    functions.push(function_type);
                }
            }
        }

        // Return a new circuit type.
        Ok(CircuitType {
            identifier: circuit_identifier,
            variables,
            functions,
        })
    }

    ///
    /// Returns the function type of a circuit member given an identifier.
    ///
    pub fn member_function_type(&self, identifier: &Identifier) -> Option<&FunctionType> {
        self.functions
            .iter()
            .find(|function| function.identifier.eq(identifier))
    }

    ///
    /// Returns the type of a circuit member.
    ///
    /// If the member is a circuit variable, then the type of the variable is returned.
    /// If the member is a circuit function, then the return type of the function is returned.
    ///
    pub fn member_type(&self, identifier: &Identifier) -> Result<Type, TypeError> {
        // Check if the circuit member is a circuit variable.
        let matched_variable = self
            .variables
            .iter()
            .find(|variable| variable.identifier.eq(identifier));

        match matched_variable {
            Some(variable) => Ok(variable.type_.to_owned()),
            None => {
                // Check if the circuit member is a circuit function.
                let matched_function = self.member_function_type(identifier);

                match matched_function {
                    Some(function) => Ok(Type::Function(function.identifier.to_owned())),
                    None => Err(TypeError::undefined_circuit_member(identifier.clone())),
                }
            }
        }
    }

    ///
    /// Returns a new `CircuitType` from a given `Input` struct.
    ///
    pub fn from_input_section(
        table: &SymbolTable,
        name: String,
        section: HashMap<Parameter, Option<InputValue>>,
    ) -> Result<Self, TypeError> {
        // Create a new `CircuitVariableType` for each section pair.
        let mut variables = Vec::new();

        for (parameter, _option) in section.into_iter() {
            let variable = CircuitVariableType {
                identifier: parameter.variable,
                type_: Type::new(table, parameter.type_, Span::default())?,
                attribute: None,
            };

            variables.push(variable);
        }

        // Create a new `Identifier` for the input section.
        let identifier = Identifier::new(name);

        // Return a new `CircuitType` with the given name.
        Ok(Self {
            identifier,
            variables,
            functions: Vec::new(),
        })
    }
}

impl PartialEq for CircuitType {
    fn eq(&self, other: &Self) -> bool {
        self.identifier.eq(&other.identifier)
    }
}

impl Eq for CircuitType {}

impl Hash for CircuitType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
    }
}
