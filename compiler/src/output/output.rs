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

use crate::{errors::OutputBytesError, ConstrainedValue, GroupType, REGISTERS_VARIABLE_NAME};
use leo_typed::{Parameter, Registers, Span};

use snarkos_models::curves::{Field, PrimeField};

use serde::{Deserialize, Serialize};
use snarkos_models::gadgets::r1cs::Index;

/// Serialized program return output.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Output {
    bytes: Vec<u8>,
    indices: Vec<usize>,
}
impl Output {
    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn indices(&self) -> Vec<usize> {
        self.indices.clone()
    }

    pub fn new<F: Field + PrimeField, G: GroupType<F>>(
        registers: &Registers,
        value: ConstrainedValue<F, G>,
        constraint_system_indices: Vec<Index>,
        span: Span,
    ) -> Result<Self, OutputBytesError> {
        let return_values = match value {
            ConstrainedValue::Tuple(values) => values,
            value => vec![value],
        };
        let register_hashmap = registers.values();

        // Create vector of parameter values in alphabetical order
        let mut register_values = register_hashmap
            .into_iter()
            .map(|register| register.0)
            .collect::<Vec<Parameter>>();

        register_values.sort_by(|a, b| a.variable.name.cmp(&b.variable.name));

        // Return an error if we do not have enough return registers
        if register_values.len() < return_values.len() {
            return Err(OutputBytesError::not_enough_registers(span));
        }

        // Manually construct result string
        let mut string = String::new();
        let header = format!("[{}]\n", REGISTERS_VARIABLE_NAME);

        string.push_str(&header);

        // format: "token_id: u64 = 1u64;"
        for (parameter, value) in register_values.into_iter().zip(return_values.into_iter()) {
            let name = parameter.variable.name;
            let type_ = parameter.type_;
            let value = value.to_string();

            let format = format!("{}: {} = {};\n", name, type_, value);

            string.push_str(&format);
        }

        let mut bytes: Vec<u8> = vec![];
        bytes.extend_from_slice(string.as_bytes());

        // Serialize constraint system indices.
        let indices = constraint_system_indices
            .iter()
            .map(|index| match index {
                Index::Input(index) => index.clone(),
                Index::Aux(index) => index.clone(),
            })
            .collect::<Vec<_>>();

        Ok(Self { bytes, indices })
    }
}
