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

use crate::{program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::Identifier;
use leo_errors::{CompilerError, LeoError};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

use super::ResolverContext;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub(super) fn resolve_target_access_member<'b, CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        mut context: ResolverContext<'a, 'b, F, G>,
        name: &Identifier,
    ) -> Result<(), LeoError> {
        if context.input.len() != 1 {
            return Err(CompilerError::statement_array_assign_interior_index(&context.span))?;
        }
        match context.input.remove(0) {
            ConstrainedValue::CircuitExpression(_variable, members) => {
                // Modify the circuit variable in place
                let matched_variable = members.iter_mut().find(|member| &member.0 == name);

                match matched_variable {
                    Some(member) => {
                        context.input = vec![&mut member.1];
                        self.resolve_target_access(cs, context)
                    }
                    None => {
                        // Throw an error if the circuit variable does not exist in the circuit
                        Err(CompilerError::statement_undefined_circuit_variable(name, &context.span))?
                    }
                }
            }
            // Throw an error if the circuit definition does not exist in the file
            x => Err(CompilerError::undefined_circuit(x, &context.span))?,
        }
    }
}
