// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::program::Program;
use leo_asg::{CircuitInitExpression, CircuitMember};
use leo_errors::{CompilerError, Result, Span};
use snarkvm_ir::Value;

impl<'a> Program<'a> {
    pub fn enforce_circuit(&mut self, expr: &CircuitInitExpression<'a>, span: &Span) -> Result<Value> {
        let circuit = expr.circuit.get();
        let members = circuit.members.borrow();

        let member_var_len = members
            .values()
            .filter(|x| matches!(x, CircuitMember::Variable(_)))
            .count();

        let mut resolved_members = vec![None; member_var_len];

        // type checking is already done in asg
        for (name, inner) in expr.values.iter() {
            let (index, _, target) = members
                .get_full(name.name.as_ref())
                .expect("illegal name in asg circuit init expression");
            match target {
                CircuitMember::Variable(_type_) => {
                    let variable_value = self.enforce_expression(inner.get())?;
                    resolved_members[index] = Some(variable_value);
                }
                _ => return Err(CompilerError::expected_circuit_member(name, span).into()),
            }
        }

        Ok(Value::Tuple(
            resolved_members
                .into_iter()
                .map(|x| x.expect("missing circuit field"))
                .collect(),
        ))
    }
}
