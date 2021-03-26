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

use crate::{errors::FunctionError, ConstrainedCircuitMember, ConstrainedProgram, ConstrainedValue, GroupType};
use leo_asg::{Circuit, CircuitMember, Type};
use leo_ast::{Identifier, Input, Span};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

pub const RECORD_VARIABLE_NAME: &str = "record";
pub const REGISTERS_VARIABLE_NAME: &str = "registers";
pub const STATE_VARIABLE_NAME: &str = "state";
pub const STATE_LEAF_VARIABLE_NAME: &str = "state_leaf";

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    #[allow(clippy::vec_init_then_push)]
    pub fn allocate_input_keyword<CS: ConstraintSystem<F>>(
        &mut self,
        cs: &mut CS,
        span: &Span,
        expected_type: &'a Circuit<'a>,
        input: &Input,
    ) -> Result<ConstrainedValue<'a, F, G>, FunctionError> {
        // Create an identifier for each input variable

        let registers_name = Identifier {
            name: REGISTERS_VARIABLE_NAME.to_string(),
            span: span.clone(),
        };
        let record_name = Identifier {
            name: RECORD_VARIABLE_NAME.to_string(),
            span: span.clone(),
        };
        let state_name = Identifier {
            name: STATE_VARIABLE_NAME.to_string(),
            span: span.clone(),
        };
        let state_leaf_name = Identifier {
            name: STATE_LEAF_VARIABLE_NAME.to_string(),
            span: span.clone(),
        };

        // Fetch each input variable's definitions

        let registers_values = input.get_registers().values();
        let record_values = input.get_record().values();
        let state_values = input.get_state().values();
        let state_leaf_values = input.get_state_leaf().values();

        // Allocate each input variable as a circuit expression

        let mut sections = Vec::with_capacity(4);

        sections.push((registers_name, registers_values));
        sections.push((record_name, record_values));
        sections.push((state_name, state_values));
        sections.push((state_leaf_name, state_leaf_values));

        let mut members = Vec::with_capacity(sections.len());

        for (name, values) in sections {
            let sub_circuit = match expected_type.members.borrow().get(&name.name) {
                Some(CircuitMember::Variable(Type::Circuit(circuit))) => *circuit,
                _ => panic!("illegal input type definition from asg"),
            };

            let member_name = name.clone();
            let member_value = self.allocate_input_section(cs, name, sub_circuit, values)?;

            let member = ConstrainedCircuitMember(member_name, member_value);

            members.push(member)
        }

        // Return input variable keyword as circuit expression

        Ok(ConstrainedValue::CircuitExpression(expected_type, members))
    }
}
