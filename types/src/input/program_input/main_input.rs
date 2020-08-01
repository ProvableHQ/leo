use crate::InputValue;
use leo_input::{definitions::Definition, InputParserError};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
pub struct MainInput {
    input: HashMap<String, Option<InputValue>>,
}

impl MainInput {
    pub fn new() -> Self {
        Self { input: HashMap::new() }
    }

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private input variables.
    pub fn empty(&self) -> Self {
        let mut input = self.input.clone();

        input.iter_mut().for_each(|(_name, value)| {
            *value = None;
        });

        Self { input }
    }

    pub fn len(&self) -> usize {
        self.input.len()
    }

    pub fn insert(&mut self, key: String, value: Option<InputValue>) {
        self.input.insert(key, value);
    }

    /// Parses main input definitions and stores them in `self`.
    pub fn parse(&mut self, definitions: Vec<Definition>) -> Result<(), InputParserError> {
        for definition in definitions {
            let name = definition.parameter.variable.value;
            let value = InputValue::from_expression(definition.parameter.type_, definition.expression)?;

            self.insert(name, Some(value));
        }

        Ok(())
    }

    /// Returns an `Option` of the main function input at `name`
    pub fn get(&self, name: &String) -> Option<Option<InputValue>> {
        self.input.get(name).map(|input| input.clone())
    }
}
