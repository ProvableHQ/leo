use crate::InputValue;
use leo_inputs::{definitions::Definition, InputParserError};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
pub struct MainInputs {
    inputs: HashMap<String, Option<InputValue>>,
}

impl MainInputs {
    pub fn new() -> Self {
        Self { inputs: HashMap::new() }
    }

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private inputs.
    pub fn empty(&self) -> Self {
        let mut inputs = self.inputs.clone();

        inputs.iter_mut().for_each(|(_name, value)| {
            *value = None;
        });

        Self { inputs }
    }

    pub fn len(&self) -> usize {
        self.inputs.len()
    }

    /// Parses main input definitions and stores them in `self`.
    pub fn parse(&mut self, definitions: Vec<Definition>) -> Result<(), InputParserError> {
        for definition in definitions {
            let name = definition.parameter.variable.value;
            let value = InputValue::from_expression(definition.parameter.type_, definition.expression)?;

            self.inputs.insert(name, Some(value));
        }

        Ok(())
    }

    /// Returns an `Option` of the main function input at `name`
    pub fn get(&self, name: &String) -> Option<Option<InputValue>> {
        self.inputs.get(name).map(|input| input.clone())
    }
}
