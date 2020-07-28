use crate::{Input, InputValue};
use leo_inputs::{definitions::Definition, InputParserError};

#[derive(Clone, PartialEq, Eq)]
pub struct State(Vec<Option<InputValue>>);

impl State {
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Stores state input definitions if the main function input contains the `state` variable.
    pub fn store_definitions(
        &mut self,
        definitions: Vec<Definition>,
        expected_inputs: &Vec<Input>,
    ) -> Result<(), InputParserError> {
        // if the main function does not contain the `state` variable
        // then do not parse state definitions
        if !expected_inputs.contains(&Input::State) {
            return Ok(());
        }

        let mut state_inputs = vec![];

        // store all registers
        for definition in definitions {
            let value = InputValue::from_expression(definition.parameter.type_, definition.expression)?;

            // push value to register inputs
            state_inputs.push(Some(value));
        }

        self.0 = state_inputs;

        Ok(())
    }
}
