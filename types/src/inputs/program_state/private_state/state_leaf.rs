use crate::{Input, InputValue};
use leo_inputs::{definitions::Definition, InputParserError};

use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
pub struct StateLeaf {
    is_present: bool,
    values: HashMap<String, Option<InputValue>>,
}

impl StateLeaf {
    pub fn new() -> Self {
        Self {
            is_present: false,
            values: HashMap::new(),
        }
    }

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private inputs.
    pub fn empty(&self) -> Self {
        let is_present = self.is_present;
        let mut values = self.values.clone();

        values.iter_mut().for_each(|(_name, value)| {
            *value = None;
        });

        Self { is_present, values }
    }

    /// Returns `true` if the `state_leaf` variable is passed as input to the main function
    pub fn is_present(&self) -> bool {
        self.is_present
    }

    /// Stores state leaf input definitions.
    /// This function is called if the main function input contains the `state_leaf` variable.
    pub fn store_definitions(&mut self, definitions: Vec<Definition>) -> Result<(), InputParserError> {
        self.is_present = true;
        // // if the main function does not contain the `state_leaf` variable
        // // then do not parse state_leaf definitions
        // if !expected_inputs.contains(&Input::StateLeaf) {
        //     return Ok(());
        // }
        //
        // let mut state_leaf_inputs = vec![];
        //
        // // store all definitions
        // for definition in definitions {
        //     let value = InputValue::from_expression(definition.parameter.type_, definition.expression)?;
        //
        //     // push value to register inputs
        //     state_leaf_inputs.push(Some(value));
        // }
        //
        // self.0 = state_leaf_inputs;

        Ok(())
    }
}
