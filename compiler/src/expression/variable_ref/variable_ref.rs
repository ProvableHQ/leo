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

//! Enforces an identifier expression in a compiled Leo program.

use crate::errors::ExpressionError;
use crate::program::ConstrainedProgram;
use crate::value::ConstrainedValue;
use crate::GroupType;
use leo_asg::VariableRef;

use snarkvm_models::curves::PrimeField;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    /// Enforce a variable expression by getting the resolved value
    pub fn evaluate_ref(&mut self, variable_ref: &VariableRef) -> Result<ConstrainedValue<'a, F, G>, ExpressionError> {
        // Evaluate the identifier name in the current function scope
        let variable = variable_ref.variable.borrow();
        let result_value = if let Some(value) = self.get(variable.id) {
            value.clone()
        } else {
            return Err(ExpressionError::undefined_identifier(variable.name.clone()));
            // todo: probably can be a panic here instead
        };

        Ok(result_value)
    }
}
