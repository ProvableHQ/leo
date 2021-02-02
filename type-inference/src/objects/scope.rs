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

use crate::{ScopeError, VariableTable};
use leo_symbol_table::{FunctionInputType, Type};

/// A structure for tracking the types of defined variables in a block of code.
#[derive(Clone, Default)]
pub struct Scope {
    pub loop_variables: VariableTable,
    pub variables: VariableTable,
}

impl Scope {
    ///
    /// Returns a new `Scope` from an optional given `Scope`.
    ///
    /// The new scope will contain the variables of the optional given `Scope`.
    ///
    pub fn new(parent: Option<Scope>) -> Self {
        match parent {
            Some(scope) => scope,
            None => Self::default(),
        }
    }

    ///
    /// Inserts a variable name -> type mapping into the loop variable table.
    ///
    pub fn insert_loop_variable(&mut self, name: String, type_: Type) -> Option<Type> {
        self.loop_variables.insert(name, type_)
    }

    ///
    /// Inserts a variable name -> type mapping into the variable table.
    ///
    pub fn insert_variable(&mut self, name: String, type_: Type) -> Option<Type> {
        self.variables.insert(name, type_)
    }

    ///
    /// Returns a reference to the type corresponding to the loop variable name.
    ///
    pub fn get_loop_variable(&self, name: &str) -> Option<&Type> {
        self.loop_variables.get(name)
    }

    ///
    /// Returns a reference to the type corresponding to the variable name.
    ///
    /// Checks loop variables first, then non-loop variables.
    ///
    pub fn get_variable(&self, name: &str) -> Option<&Type> {
        match self.get_loop_variable(name) {
            Some(loop_variable_type) => Some(loop_variable_type),
            None => self.variables.get(name),
        }
    }

    ///
    /// Inserts a vector of function input types into the `Scope` variable table.
    ///
    pub fn insert_function_inputs(&mut self, function_inputs: &[FunctionInputType]) -> Result<(), ScopeError> {
        self.variables
            .insert_function_inputs(function_inputs)
            .map_err(ScopeError::VariableTableError)
    }
}
