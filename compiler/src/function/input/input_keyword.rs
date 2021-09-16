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

use crate::Program;
use leo_asg::{Circuit, CircuitMember, Type};
use leo_errors::{Result, Span};
use snarkvm_ir::Value;

pub const RECORD_VARIABLE_NAME: &str = "record";
pub const REGISTERS_VARIABLE_NAME: &str = "registers";
pub const STATE_VARIABLE_NAME: &str = "state";
pub const STATE_LEAF_VARIABLE_NAME: &str = "state_leaf";

impl<'a> Program<'a> {
    #[allow(clippy::vec_init_then_push)]
    pub fn allocate_input_keyword(
        &mut self,
        span: &Span,
        expected_type: &'a Circuit<'a>,
        input: &leo_ast::Input,
    ) -> Result<Value> {
        // Allocate each input variable as a circuit expression

        let sections = [
            REGISTERS_VARIABLE_NAME,
            RECORD_VARIABLE_NAME,
            STATE_VARIABLE_NAME,
            STATE_LEAF_VARIABLE_NAME,
        ];

        let mut out_variables = vec![];
        for name in sections.iter() {
            let sub_circuit = match expected_type.members.borrow().get(*name) {
                Some(CircuitMember::Variable(Type::Circuit(circuit))) => *circuit,
                _ => panic!("illegal input type definition from asg"),
            };

            let origin = match *name {
                REGISTERS_VARIABLE_NAME => input.get_registers().types(),
                RECORD_VARIABLE_NAME => input.get_record().types(),
                STATE_VARIABLE_NAME => input.get_state().types(),
                STATE_LEAF_VARIABLE_NAME => input.get_state_leaf().types(),
                _ => panic!("illegal input section: {}", name),
            };

            out_variables.push(Value::Tuple(self.allocate_input_section(
                name,
                span,
                sub_circuit,
                origin,
            )?));
        }

        // Return input variable keyword as circuit expression

        Ok(Value::Tuple(out_variables))
    }
}
