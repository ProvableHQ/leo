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

use crate::{ConstrainedValue, GroupType, REGISTERS_VARIABLE_NAME};
use leo_asg::Program;
use leo_ast::{Parameter, Registers};
use leo_errors::{CompilerError, LeoError, Span};

use snarkvm_fields::PrimeField;

use serde::{Deserialize, Serialize};

/// Serialized program return output.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct OutputBytes(Vec<u8>);

impl OutputBytes {
    pub fn bytes(&self) -> &Vec<u8> {
        &self.0
    }

    pub fn new_from_constrained_value<'a, F: PrimeField, G: GroupType<F>>(
        program: &Program<'a>,
        registers: &Registers,
        value: ConstrainedValue<'a, F, G>,

        span: &Span,
    ) -> Result<Self, LeoError> {
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
            return Err(LeoError::from(CompilerError::not_enough_registers(span));)
        }

        // Manually construct result string
        let mut string = String::new();
        let header = format!("[{}]\n", REGISTERS_VARIABLE_NAME);

        string.push_str(&header);

        // format: "token_id: u64 = 1u64;"
        for (parameter, value) in register_values.into_iter().zip(return_values.into_iter()) {
            let name = parameter.variable.name;

            // Check register type == return value type.
            let register_type = program.scope.resolve_ast_type(&parameter.type_)?;
            let return_value_type = value.to_type(span)?;

            if !register_type.is_assignable_from(&return_value_type) {
                return Err(LeoError::from(CompilerError::mismatched_output_types(
                    &register_type,
                    &return_value_type,
                    span,
                )));
            }

            let value = value.to_string();

            let format = format!("{}: {} = {};\n", name, register_type, value,);

            string.push_str(&format);
        }

        let bytes = string.into_bytes();

        Ok(Self(bytes))
    }
}

impl From<Vec<u8>> for OutputBytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}
