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
    types::functions::{FunctionInputType, FunctionOutputType},
    SymbolTable,
    TypeError,
};
use leo_ast::{Function, Identifier};

use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Stores function definition details.
///
/// This type should be added to the function symbol table for a resolved syntax tree.
/// This is a user-defined type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FunctionType {
    /// The name of the function definition.
    pub identifier: Identifier,

    /// The function inputs.
    pub inputs: Vec<FunctionInputType>,

    /// The function output.
    pub output: FunctionOutputType,
}

impl FunctionType {
    ///
    /// Return a new `FunctionType` from a given `Function` definition.
    ///
    /// Performs a lookup in the given symbol table if the function definition contains
    /// user-defined types.
    ///
    pub fn new(table: &SymbolTable, unresolved: Function) -> Result<Self, TypeError> {
        let mut inputs_resolved = Vec::with_capacity(unresolved.input.len());

        // Type check function inputs
        for input in unresolved.input {
            let input = FunctionInputType::new(table, input)?;
            inputs_resolved.push(input);
        }

        // Type check function output
        let output = FunctionOutputType::new(table, unresolved.output, unresolved.span)?;

        Ok(FunctionType {
            identifier: unresolved.identifier,
            inputs: inputs_resolved,
            output,
        })
    }

    ///
    /// Return a new `FunctionType` from a given `Function` definition.
    ///
    /// Performs a lookup in the given symbol table if the function definition contains
    /// user-defined types.
    ///
    /// If the function definition contains the `Self` keyword, then the given circuit identifier
    /// is used as the type.
    ///
    pub fn from_circuit(
        table: &SymbolTable,
        circuit_name: Identifier,
        unresolved_function: Function,
    ) -> Result<Self, TypeError> {
        let function_identifier = unresolved_function.identifier;
        let mut inputs = Vec::with_capacity(unresolved_function.input.len());

        // Type check function inputs.
        for unresolved_input in unresolved_function.input {
            let input = FunctionInputType::new_from_circuit(table, unresolved_input, circuit_name.clone())?;
            inputs.push(input);
        }

        // Type check function output.
        let output = FunctionOutputType::new_from_circuit(
            table,
            circuit_name,
            unresolved_function.output,
            unresolved_function.span,
        )?;

        Ok(FunctionType {
            identifier: function_identifier,
            inputs,
            output,
        })
    }

    ///
    /// Resolve a function definition and insert it into the given symbol table.
    ///
    pub fn insert_definition(table: &mut SymbolTable, unresolved_function: Function) -> Result<(), TypeError> {
        // Get the identifier of the function.
        let function_identifier = unresolved_function.identifier.clone();

        // Resolve the function definition into a function type.
        let function = Self::new(table, unresolved_function)?;

        // Insert (function_identifier -> function_type) as a (key -> value) pair in the symbol table.
        table.insert_function_type(function_identifier, function);

        Ok(())
    }
}

impl PartialEq for FunctionType {
    fn eq(&self, other: &Self) -> bool {
        self.identifier.eq(&other.identifier)
    }
}

impl Eq for FunctionType {}

impl Hash for FunctionType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
    }
}
