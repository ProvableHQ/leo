use crate::{Input, InputValue, MainInputs, ProgramInputs, ProgramState};
use leo_inputs::{
    files::{File, TableOrSection},
    InputParserError,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Inputs {
    inputs: ProgramInputs,
    state: ProgramState,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            inputs: ProgramInputs::new(),
            state: ProgramState::new(),
        }
    }

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private inputs.
    pub fn empty(&self) -> Self {
        let inputs = self.inputs.empty();
        let state = self.state.empty();

        Self { inputs, state }
    }

    pub fn len(&self) -> usize {
        self.inputs.len() + self.state.len()
    }

    pub fn set_main_inputs(&mut self, inputs: MainInputs) {
        self.inputs.main = inputs;
    }

    pub fn parse_file(&mut self, file: File) -> Result<(), InputParserError> {
        for entry in file.entries.into_iter() {
            match entry {
                TableOrSection::Section(section) => {
                    self.inputs.store_definitions(section)?;
                }
                TableOrSection::Table(table) => {
                    self.state.store_definitions(table)?;
                }
            }
        }

        Ok(())
    }

    /// Returns the main function input value with the given `name`
    pub fn get(&self, name: &String) -> Option<InputValue> {
        self.inputs.get(name)
    }
}
