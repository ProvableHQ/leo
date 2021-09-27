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

pub mod output_file;
use std::fmt;

pub use self::output_file::*;

pub mod output_bytes;
pub use self::output_bytes::*;

use crate::REGISTERS_VARIABLE_NAME;
use indexmap::IndexMap;
use leo_errors::{CompilerError, Result, Span};
use snarkvm_eval::{ConstrainedValue, GroupType, PrimeField};

use serde::{Deserialize, Serialize};
use snarkvm_ir::Input;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct OutputRegister {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Output {
    pub registers: IndexMap<String, OutputRegister>,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[{}]", REGISTERS_VARIABLE_NAME)?;
        // format: "token_id: u64 = 1u64;"
        for (name, register) in self.registers.iter() {
            match register.type_.as_str() {
                "char" => writeln!(f, "{}: {} = '{}';", name, register.type_, register.value)?,
                _ => writeln!(f, "{}: {} = {};", name, register.type_, register.value)?,
            }
        }
        Ok(())
    }
}

#[allow(clippy::from_over_into)]
impl Into<OutputBytes> for Output {
    fn into(self) -> OutputBytes {
        OutputBytes::from(self.to_string().into_bytes())
    }
}

impl Output {
    pub fn new<F: PrimeField, G: GroupType<F>>(
        registers: &[Input],
        value: ConstrainedValue<F, G>,
        span: &Span,
    ) -> Result<Self> {
        let return_values = match value {
            ConstrainedValue::Tuple(values) => values,
            value => vec![value],
        };

        // Return an error if we do not have enough return registers
        if registers.len() < return_values.len() {
            return Err(CompilerError::output_not_enough_registers(span).into());
        }

        let mut out = IndexMap::new();

        for (input, value) in registers.iter().zip(return_values.into_iter()) {
            if !value.matches_input_type(&input.type_) {
                return Err(CompilerError::output_mismatched_types(&input.type_, value.to_string(), span).into());
            }

            let value = value.to_string();

            out.insert(
                input.name.clone(),
                OutputRegister {
                    type_: input.type_.to_string(),
                    value,
                },
            );
        }

        Ok(Output { registers: out })
    }
}
