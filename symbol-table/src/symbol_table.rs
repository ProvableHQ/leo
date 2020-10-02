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

use crate::{CircuitType, FunctionType, ResolvedNode, SymbolTableError, VariableType};
use leo_typed::{Circuit, Function, Identifier, Program as UnresolvedProgram};

use leo_imports::ImportParser;
use std::collections::HashMap;

/// A abstract data type that tracks the current bindings of identifier
/// names to types in a Leo program.
///
/// A symbol table has access to all function and circuit names in its
/// parent's symbol table.
/// A symbol table cannot access names in its child's symbol table.
/// Children cannot access names in another sibling's symbol table.
#[derive(Clone)]
pub struct SymbolTable {
    /// Maps variable name -> variable type.
    variables: HashMap<String, VariableType>,

    /// Maps circuit name -> circuit type.
    circuits: HashMap<String, CircuitType>,

    ///Maps function name -> function type.
    functions: HashMap<String, FunctionType>,

    /// The parent of this symbol table.
    parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    ///
    /// Creates a new symbol table with a given parent symbol table.
    ///
    pub fn new(parent: Option<Box<SymbolTable>>) -> Self {
        SymbolTable {
            variables: HashMap::new(),
            circuits: HashMap::new(),
            functions: HashMap::new(),
            parent,
        }
    }

    ///
    /// Insert a variable into the symbol table from a given name and variable type.
    ///
    /// If the symbol table did not have this name present, `None` is returned.
    /// If the symbol table did have this name present, the variable type is updated, and the old
    /// variable type is returned.
    ///
    pub fn insert_variable(&mut self, name: String, variable_type: VariableType) -> Option<VariableType> {
        self.variables.insert(name, variable_type)
    }

    ///
    /// Insert a circuit definition into the symbol table from a given circuit identifier and
    /// circuit type.
    ///
    /// If the symbol table did not have this name present, `None` is returned.
    /// If the symbol table did have this name present, the circuit type is updated, and the old
    /// circuit type is returned.
    ///
    pub fn insert_circuit(&mut self, identifier: Identifier, circuit_type: CircuitType) -> Option<CircuitType> {
        self.circuits.insert(identifier.name, circuit_type)
    }

    ///
    /// Insert a function definition into the symbol table from a given identifier and
    /// function type.
    ///
    /// If the symbol table did not have this name present, `None` is returned.
    /// If the symbol table did have this name present, the function type is updated, and the old
    /// function type is returned.
    ///
    pub fn insert_function(&mut self, identifier: Identifier, function_type: FunctionType) -> Option<FunctionType> {
        self.functions.insert(identifier.name, function_type)
    }

    ///
    /// Returns a reference to the variable type corresponding to the name.
    ///
    /// If the symbol table did not have this name present, then `None` is returned.
    ///
    pub fn get_variable(&self, name: &String) -> Option<&VariableType> {
        // Lookup variable name in symbol table.
        match self.variables.get(name) {
            Some(variable) => Some(variable),
            None => None,
        }
    }

    ///
    /// Returns a reference to the circuit type corresponding to the name.
    ///
    /// If the symbol table did not have this name present, then the parent symbol table is checked.
    /// If there is no parent symbol table, then `None` is returned.
    ///
    pub fn get_circuit(&self, name: &String) -> Option<&CircuitType> {
        // Lookup name in symbol table.
        match self.circuits.get(name) {
            Some(circuit) => Some(circuit),
            None => {
                // Lookup name in parent symbol table.
                match &self.parent {
                    Some(parent) => parent.get_circuit(name),
                    None => None,
                }
            }
        }
    }

    ///
    /// Returns a reference to the function type corresponding to the name.
    ///
    /// If the symbol table did not have this name present, then the parent symbol table is checked.
    /// If there is no parent symbol table, then `None` is returned.
    ///
    pub fn get_function(&self, key: &String) -> Option<&FunctionType> {
        // Lookup name in symbol table.
        match self.functions.get(key) {
            Some(circuit) => Some(circuit),
            None => {
                // Lookup name in parent symbol table
                match &self.parent {
                    Some(parent) => parent.get_function(key),
                    None => None,
                }
            }
        }
    }

    ///
    /// Inserts all imported identifiers for a given list of imported programs.
    ///
    /// No type resolution performed at this step.
    ///
    pub fn insert_imports(&mut self, _imports: ImportParser) {}

    ///
    /// Checks for duplicate circuit names given a hashmap of unresolved circuits.
    ///
    /// If a circuit name has no duplicates, then it is inserted into the symbol table.
    /// Variables defined later in the unresolved program cannot have the same name.
    ///
    pub fn check_duplicate_circuits(
        &mut self,
        circuits: &HashMap<Identifier, Circuit>,
    ) -> Result<(), SymbolTableError> {
        // Iterate over circuit names and definitions.
        for (identifier, circuit) in circuits.iter() {
            // Attempt to insert the circuit name into the symbol table.
            let duplicate = self.insert_variable(identifier.to_string(), VariableType::from(circuit.clone()));

            // Check that the circuit name is unique.
            if duplicate.is_some() {
                return Err(SymbolTableError::duplicate_circuit(
                    identifier.clone(),
                    circuit.circuit_name.span.clone(),
                ));
            }
        }

        Ok(())
    }

    ///
    /// Checks for duplicate function names given a hashmap of unresolved functions.
    ///
    /// If a function name has no duplicates, then it is inserted into the symbol table.
    /// Variables defined later in the unresolved program cannot have the same name.
    ///
    pub fn check_duplicate_functions(
        &mut self,
        functions: &HashMap<Identifier, Function>,
    ) -> Result<(), SymbolTableError> {
        // Iterate over function names and definitions.
        for (identifier, function) in functions.iter() {
            // Attempt to insert the function name into the symbol table.
            let duplicate = self.insert_variable(identifier.to_string(), VariableType::from(function.clone()));

            // Check that the function name is unique.
            if duplicate.is_some() {
                return Err(SymbolTableError::duplicate_function(
                    identifier.clone(),
                    function.identifier.span.clone(),
                ));
            }
        }

        Ok(())
    }

    ///
    /// Checks for unknown types in a circuit given a hashmap of unresolved circuits.
    ///
    /// If a circuit definition only contains known types, then it is inserted into the
    /// symbol table. Variables defined later in the unresolved program can lookup the definition
    /// and refer to its expected types
    ///
    pub fn check_unknown_types_circuits(
        &mut self,
        circuits: &HashMap<Identifier, Circuit>,
    ) -> Result<(), SymbolTableError> {
        // Iterate over circuit names and definitions.
        for (_, circuit) in circuits.iter() {
            // Get the identifier of the unresolved circuit.
            let identifier = circuit.circuit_name.clone();

            // Resolve unknown types in the unresolved circuit definition.
            let circuit_type = CircuitType::resolve(self, circuit.clone())?;

            // Attempt to insert the circuit definition into the symbol table.
            self.insert_circuit(identifier, circuit_type);
        }

        Ok(())
    }

    ///
    /// Checks for unknown types in a function given a hashmap of unresolved functions.
    ///
    /// If a function definition only contains known types, then it is inserted into the
    /// symbol table. Variables defined later in the unresolved program can lookup the definition
    /// and refer to its expected types
    ///
    pub fn check_unknown_types_functions(
        &mut self,
        functions: &HashMap<Identifier, Function>,
    ) -> Result<(), SymbolTableError> {
        // Iterate over function names and definitions.
        for (_, function) in functions.iter() {
            // Get the identifier of the unresolved function.
            let identifier = function.identifier.clone();

            // Resolve unknown types in the unresolved function definition.
            let function_type = FunctionType::resolve(self, function.clone())?;

            // Attempt to insert the function definition into the symbol table.
            self.insert_function(identifier, function_type);
        }

        Ok(())
    }

    ///
    /// Checks for duplicate circuit and function names given an unresolved program.
    ///
    /// If a circuit or function name has no duplicates, then it is inserted into the symbol table.
    /// Variables defined later in the unresolved program cannot have the same name.
    ///
    pub fn pass_one(&mut self, program: &UnresolvedProgram) -> Result<(), SymbolTableError> {
        // Check unresolved program circuit names.
        self.check_duplicate_circuits(&program.circuits)?;

        // Check unresolved program function names.
        self.check_duplicate_functions(&program.functions)?;

        Ok(())
    }

    ///
    /// Checks for unknown types in circuit and function definitions given an unresolved program.
    ///
    /// If a circuit or function definition only contains known types, then it is inserted into the
    /// symbol table. Variables defined later in the unresolved program can lookup the definition and
    /// refer to its expected types.
    ///
    pub fn pass_two(&mut self, program: &UnresolvedProgram) -> Result<(), SymbolTableError> {
        // Check unresolved program circuit definitions.
        self.check_unknown_types_circuits(&program.circuits)?;

        // Check unresolved program function definitions.
        self.check_unknown_types_functions(&program.functions)?;

        Ok(())
    }
}
