use crate::{Input, InputValue};
use leo_inputs::{definitions::Definition, InputParserError};

#[derive(Clone, PartialEq, Eq)]
pub struct Record(Vec<Option<InputValue>>);

impl Record {
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Stores record input definitions if the main function input contains the `record` variable.
    pub fn store_definitions(
        &mut self,
        definitions: Vec<Definition>,
        expected_inputs: &Vec<Input>,
    ) -> Result<(), InputParserError> {
        // if the main function does not contain the `record` variable
        // then do not parse record definitions
        if !expected_inputs.contains(&Input::Record) {
            return Ok(());
        }

        let mut record_inputs = vec![];

        // store all definitions
        for definition in definitions {
            let value = InputValue::from_expression(definition.parameter.type_, definition.expression)?;

            // push value to register inputs
            record_inputs.push(Some(value));
        }

        self.0 = record_inputs;

        Ok(())
    }
}
