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

//! Enforces an identifier expression in a compiled Leo program.

use crate::{
    errors::ExpressionError,
    program::{new_scope, ConstrainedProgram},
    value::ConstrainedValue,
    Address,
    GroupType,
};
use leo_ast::{Identifier, Type};

use snarkos_models::curves::{Field, PrimeField};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    /// Enforce a variable expression by getting the resolved value
    pub fn evaluate_identifier(
        &mut self,
        file_scope: &str,
        function_scope: &str,
        expected_type: Option<Type>,
        unresolved_identifier: Identifier,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        // Evaluate the identifier name in the current function scope
        let variable_name = new_scope(function_scope, &unresolved_identifier.name);
        let identifier_name = new_scope(file_scope, &unresolved_identifier.name);

        let mut result_value = if let Some(value) = self.get(&variable_name) {
            // Reassigning variable to another variable
            value.clone()
        } else if let Some(value) = self.get(&identifier_name) {
            // Check global scope (function and circuit names)
            value.clone()
        } else if let Some(value) = self.get(&unresolved_identifier.name) {
            // Check imported file scope
            value.clone()
        } else if expected_type == Some(Type::Address) {
            // If we expect an address type, try to return an address
            let address = Address::constant(unresolved_identifier.name, &unresolved_identifier.span)?;

            return Ok(ConstrainedValue::Address(address));
        } else {
            return Err(ExpressionError::undefined_identifier(unresolved_identifier));
        };

        result_value.resolve_type(expected_type, &unresolved_identifier.span)?;

        Ok(result_value)
    }
}
