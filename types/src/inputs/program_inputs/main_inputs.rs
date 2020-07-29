use crate::{Input, InputValue};
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

    pub fn len(&self) -> usize {
        self.inputs.len()
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

    /// Parses main input definitions and stores them in `self`.
    pub fn parse(&mut self, definitions: Vec<Definition>) -> Result<(), InputParserError> {
        for definition in definitions {
            let name = definition.parameter.variable.value;
            let value = InputValue::from_expression(definition.parameter.type_, definition.expression)?;

            self.inputs.insert(name, Some(value));
        }

        Ok(())
    }

    /// Returns main function input at `index`
    pub fn get(&self, name: &String) -> Option<InputValue> {
        self.inputs.get(name).unwrap().clone()
    }
}
