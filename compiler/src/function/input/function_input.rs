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

//! Enforces a function input parameter in a compiled Leo program.

use crate::{errors::FunctionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};

use leo_asg::Expression;
use std::sync::Arc;

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn enforce_function_input<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        scope: &str,
        caller_scope: &str,
        function_name: &str,
        input: &Arc<Expression>,
    ) -> Result<ConstrainedValue<F, G>, FunctionError> {
        // Evaluate the function input value as pass by value from the caller or
        // evaluate as an expression in the current function scope
        match input {
            expression => Ok(self.enforce_expression(cs, scope, function_name, expression)?),
        }
    }
}
