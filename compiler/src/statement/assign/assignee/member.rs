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

use crate::program::Program;
use leo_asg::{CircuitMember, Identifier, Type};
use leo_errors::{CompilerError, Result};
use snarkvm_ir::{Instruction, Integer, QueryData, Value};

use super::ResolverContext;

impl<'a> Program<'a> {
    pub(super) fn resolve_target_access_member<'b>(
        &mut self,
        mut context: ResolverContext<'a, 'b>,
        name: &Identifier,
    ) -> Result<Value> {
        let input_var = context.input_register;

        let (inner_type, index) = match &context.input_type {
            Type::Circuit(circuit) => {
                let members = circuit.members.borrow();
                let (index, _, member) = members
                    .get_full(name.name.as_ref())
                    .expect("illegal member name in circuit member assignment");
                let inner_type = match member {
                    CircuitMember::Variable(type_) => type_.clone(),
                    CircuitMember::Const(_) | CircuitMember::Function(_) => {
                        return Err(
                            CompilerError::illegal_static_member_assignment(name.name.as_ref(), &name.span).into(),
                        )
                    }
                };
                (inner_type, index)
            }
            _ => panic!("illegal type in circuit member assignment"),
        };

        let out = self.alloc();
        self.emit(Instruction::TupleIndexGet(QueryData {
            destination: out,
            values: vec![Value::Ref(input_var), Value::Integer(Integer::U32(index as u32))],
        }));
        context.input_register = out;
        context.input_type = inner_type.clone();
        let inner = self.resolve_target_access(context)?;
        self.emit(Instruction::TupleIndexStore(QueryData {
            destination: input_var,
            values: vec![Value::Integer(Integer::U32(index as u32)), inner],
        }));
        Ok(Value::Ref(input_var))
    }
}
