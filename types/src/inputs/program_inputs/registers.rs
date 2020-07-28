use crate::{Input, InputValue};
use leo_inputs::{definitions::Definition, InputParserError};

#[derive(Clone, PartialEq, Eq)]
pub struct Registers(Vec<Option<InputValue>>);

impl Registers {
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Stores register input definitions if the main function input contains the `registers` variable.
    pub fn store_definitions(
        &mut self,
        definitions: Vec<Definition>,
        expected_inputs: &Vec<Input>,
    ) -> Result<(), InputParserError> {
        // if the main function does not contain the `registers` variable
        // then do not parse registers
        if !expected_inputs.contains(&Input::Registers) {
            return Ok(());
        }

        let mut register_inputs = vec![];

        // store all registers
        for definition in definitions {
            let value = InputValue::from_expression(definition.parameter.type_, definition.expression)?;

            // push value to register inputs
            register_inputs.push(Some(value));
        }

        self.0 = register_inputs;

        Ok(())
    }
}
