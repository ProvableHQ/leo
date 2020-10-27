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

use crate::VariableTableError;
use leo_static_check::{FunctionInputType, Type};
use std::collections::HashMap;

/// Mapping of variable names to types
#[derive(Clone)]
pub struct VariableTable(pub HashMap<String, Type>);

impl VariableTable {
    ///
    /// Insert a name -> type pair into the variable table.
    ///
    /// If the variable table did not have this key present, [`None`] is returned.
    ///
    /// If the variable table did have this key present, the type is updated, and the old
    /// type is returned.
    ///
    pub fn insert(&mut self, name: String, type_: Type) -> Option<Type> {
        self.0.insert(name, type_)
    }

    ///
    /// Returns a reference to the type corresponding to the name.
    ///
    /// If the variable table did not have this key present, throw an undefined variable error
    /// using the given span.
    ///
    pub fn get(&self, name: &String) -> Option<&Type> {
        self.0.get(name)
    }

    ///
    /// Inserts a vector of function input types into the variable table.
    ///
    pub fn insert_function_inputs(
        &mut self,
        function_inputs: &Vec<FunctionInputType>,
    ) -> Result<(), VariableTableError> {
        for input in function_inputs {
            let input_name = input.identifier().name.clone();
            let input_type = input.type_();

            // TODO (collinc97) throw an error for duplicate function input names.
            self.insert(input_name, input_type);
        }
        Ok(())
    }
}

impl Default for VariableTable {
    fn default() -> Self {
        Self(HashMap::new())
    }
}
