use crate::{errors::OutputBytesError, ConstrainedValue, GroupType, REGISTERS_VARIABLE_NAME};
use leo_types::{Parameter, Registers, Span};

use snarkos_models::curves::{Field, PrimeField};

use serde::{Deserialize, Serialize};

/// Serialized program return output.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct OutputBytes(Vec<u8>);

impl OutputBytes {
    pub fn bytes(&self) -> &Vec<u8> {
        &self.0
    }

    pub fn new_from_constrained_value<F: Field + PrimeField, G: GroupType<F>>(
        registers: &Registers,
        value: ConstrainedValue<F, G>,
        span: Span,
    ) -> Result<Self, OutputBytesError> {
        let return_values = match value {
            ConstrainedValue::Return(values) => values,
            value => return Err(OutputBytesError::illegal_return(value.to_string(), span)),
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

            let format = format!("{}: {} = {};\n", name, type_, value,);

            string.push_str(&format);
        }

        let mut bytes: Vec<u8> = vec![];
        bytes.extend_from_slice(string.as_bytes());

        Ok(Self(bytes))
    }
}

impl From<Vec<u8>> for OutputBytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}
