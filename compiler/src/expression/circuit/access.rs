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

//! Enforces a circuit access expression in a compiled Leo program.

use crate::{errors::ExpressionError, program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::{CircuitAccessExpression, Node};

use snarkvm_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_circuit_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expr: &CircuitAccessExpression,
    ) -> Result<ConstrainedValue<F, G>, ExpressionError> {
        if let Some(target) = &expr.target {
            //todo: we can prob pass values by ref here to avoid copying the entire circuit on access
            let target_value = self.enforce_operand(cs, target)?;
            match target_value {
                ConstrainedValue::CircuitExpression(def, members) => {
                    assert!(def.circuit == expr.circuit);
                    if let Some(member) = members.into_iter().find(|x| x.0.name == expr.member.name) {
                        Ok(member.1)
                    } else {
                        Err(ExpressionError::undefined_member_access(
                            expr.circuit.name.borrow().to_string(),
                            expr.member.to_string(),
                            expr.member.span.clone(),
                        ))
                    }
                }
                value => Err(ExpressionError::undefined_circuit(
                    value.to_string(),
                    target.span().cloned().unwrap_or_default(),
                )),
            }
        } else {
            Err(ExpressionError::invalid_static_access(
                expr.member.to_string(),
                expr.member.span.clone(),
            ))
        }
    }
}
