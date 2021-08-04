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

//! Enforces a circuit expression in a compiled Leo program.

use crate::{
    program::ConstrainedProgram,
    value::{ConstrainedCircuitMember, ConstrainedValue},
    GroupType,
};
use leo_asg::{CircuitInitExpression, CircuitMember};
use leo_errors::{new_backtrace, CompilerError, Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn enforce_circuit<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        expr: &CircuitInitExpression<'a>,
        span: &Span,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        let circuit = expr.circuit.get();
        let members = circuit.members.borrow();

        let mut resolved_members = Vec::with_capacity(members.len());

        // type checking is already done in asg
        for (name, inner) in expr.values.iter() {
            let target = members
                .get(name.name.as_ref())
                .expect("illegal name in asg circuit init expression");
            match target {
                CircuitMember::Variable(_type_) => {
                    let variable_value = self.enforce_expression(cs, inner.get())?;
                    resolved_members.push(ConstrainedCircuitMember(name.clone(), variable_value));
                }
                _ => {
                    return Err(CompilerError::expected_circuit_member(name, span, new_backtrace()).into());
                }
            }
        }

        let value = ConstrainedValue::CircuitExpression(circuit, resolved_members);
        Ok(value)
    }
}
