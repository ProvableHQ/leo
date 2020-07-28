use crate::{Input, InputValue};
use leo_inputs::{definitions::Definition, InputParserError};

#[derive(Clone, PartialEq, Eq)]
pub struct StateLeaf(Vec<Option<InputValue>>);

impl StateLeaf {
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Stores state_leaf input definitions if the main function input contains the `state_leaf` variable.
    pub fn store_definitions(
        &mut self,
        definitions: Vec<Definition>,
        expected_inputs: &Vec<Input>,
    ) -> Result<(), InputParserError> {
        // if the main function does not contain the `state_leaf` variable
        // then do not parse state_leaf definitions
        if !expected_inputs.contains(&Input::StateLeaf) {
            return Ok(());
        }

        let mut state_leaf_inputs = vec![];

        // store all definitions
        for definition in definitions {
            let value = InputValue::from_expression(definition.parameter.type_, definition.expression)?;

            // push value to register inputs
            state_leaf_inputs.push(Some(value));
        }

        self.0 = state_leaf_inputs;

        Ok(())
    }
}
