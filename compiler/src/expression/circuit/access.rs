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

use crate::{program::ConstrainedProgram, value::ConstrainedValue, GroupType};
use leo_asg::{CircuitAccessExpression, Node};
use leo_errors::{new_backtrace, CompilerError, Result};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_circuit_access<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expr: &CircuitAccessExpression<'a>,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        if let Some(target) = expr.target.get() {
            //todo: we can prob pass values by ref here to avoid copying the entire circuit on access
            let target_value = self.enforce_expression(cs, target)?;
            match target_value {
                ConstrainedValue::CircuitExpression(def, members) => {
                    assert!(def == expr.circuit.get());
                    if let Some(member) = members.into_iter().find(|x| x.0.name == expr.member.name) {
                        Ok(member.1)
                    } else {
                        return Err(CompilerError::undefined_circuit_member_access(
                            expr.circuit.get().name.borrow(),
                            &expr.member.name,
                            &expr.member.span,
                            new_backtrace(),
                        )
                        .into());
                    }
                }
                value => {
                    return Err(CompilerError::undefined_circuit(
                        value,
                        &target.span().cloned().unwrap_or_default(),
                        new_backtrace(),
                    )
                    .into());
                }
            }
        } else {
            Err(CompilerError::invalid_circuit_static_member_access(
                &expr.member.name,
                &expr.member.span,
                new_backtrace(),
            )
            .into())
        }
    }
}
