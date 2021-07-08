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
use std::{collections::BTreeMap, fmt};

pub use self::output_file::*;

pub mod output_bytes;
pub use self::output_bytes::*;

use crate::{errors::OutputBytesError, Char, CharType, ConstrainedValue, GroupType, REGISTERS_VARIABLE_NAME};
use leo_asg::Program;
use leo_ast::{Parameter, Registers, Span};

use snarkvm_fields::PrimeField;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OutputRegister {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Output {
    pub registers: BTreeMap<String, OutputRegister>,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[{}]", REGISTERS_VARIABLE_NAME)?;
        // format: "token_id: u64 = 1u64;"
        for (name, register) in self.registers.iter() {
            writeln!(f, "{}: {} = {};", name, register.type_, register.value)?;
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

fn char_to_output_string<F: PrimeField>(character: &Char<F>, quote: bool) -> String {
    let mut string = String::new();
    if quote {
        string.push('\'');
    }

    match character.character {
        CharType::Scalar(scalar) => {
            if scalar.is_alphanumeric() {
                string.push(scalar);
            } else {
                string.push_str(format!("{}", char::escape_default(scalar)).as_str());
            }
        }
        CharType::NonScalar(non_scalar) => string.push_str(format!("\\u{{{:x}}}", non_scalar).as_str()),
    }
    if quote {
        string.push('\'');
    }
    string
}

impl Output {
    pub fn new<'a, F: PrimeField, G: GroupType<F>>(
        program: &Program<'a>,
        registers: &Registers,
        value: ConstrainedValue<'a, F, G>,
        span: &Span,
    ) -> Result<Self, OutputBytesError> {
        let return_values = match value {
            ConstrainedValue::Tuple(tuple) => tuple,
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

        let mut registers = BTreeMap::new();

        for (parameter, value) in register_values.into_iter().zip(return_values.into_iter()) {
            let name = parameter.variable.name;

            // Check register type == return value type.
            let register_type = program.scope.resolve_ast_type(&parameter.type_)?;
            let return_value_type = value.to_type(span)?;

            if !register_type.is_assignable_from(&return_value_type) {
                return Err(OutputBytesError::mismatched_output_types(
                    &register_type,
                    &return_value_type,
                    span,
                ));
            }

            let value = match value {
                ConstrainedValue::Char(c) => char_to_output_string(&c, true),
                ConstrainedValue::Array(array) => {
                    let mut string = String::new();
                    string.push('"');
                    for e in array.iter() {
                        if let ConstrainedValue::Char(c) = e {
                            string.push_str(char_to_output_string(c, false).as_str());
                        } else {
                            string.push_str(e.to_string().as_str());
                        }
                    }
                    string.push('"');
                    string
                }
                ConstrainedValue::Tuple(tuple) => {
                    let values = tuple
                        .iter()
                        .map(|e| {
                            if let ConstrainedValue::Char(c) = e {
                                char_to_output_string(c, true)
                            } else {
                                e.to_string()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ");

                    format!("({})", values)
                }
                ConstrainedValue::Field(field) => format!("{:?}", field),
                ConstrainedValue::Group(group) => format!("{:?}", group),
                _ => value.to_string(),
            };

            registers.insert(name.to_string(), OutputRegister {
                type_: register_type.to_string(),
                value,
            });
        }

        Ok(Output { registers })
    }
}
