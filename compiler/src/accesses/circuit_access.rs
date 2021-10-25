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

use crate::program::Program;
use leo_asg::{CircuitAccess, CircuitMember};
use leo_errors::{CompilerError, Result};
use snarkvm_ir::{Instruction, Integer, QueryData, Value};

impl<'a> Program<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn enforce_circuit_access(&mut self, expr: &CircuitAccess<'a>) -> Result<Value> {
        let members = expr.circuit.get().members.borrow();
        let target_value = match expr.target.get() {
            Some(target) => self.enforce_expression(target)?,
            None => match members.get(expr.member.name.as_ref()) {
                Some(CircuitMember::Const(value)) => Value::Tuple(vec![self.enforce_expression(value)?]),
                _ => {
                    return Err(CompilerError::expected_circuit_static_const_access(expr.span.as_ref().unwrap()).into())
                }
            },
        };

        let mut index = members
            .get_index_of(expr.member.name.as_ref())
            .expect("missing member from struct");

        if let Some(category) = expr.circuit.get().input_type() {
            index = self.input_index(category, expr.member.name.as_ref());
        }

        let out = self.alloc();
        self.emit(Instruction::TupleIndexGet(QueryData {
            destination: out,
            values: vec![target_value, Value::Integer(Integer::U32(index as u32))],
        }));
        Ok(Value::Ref(out))
    }
}
