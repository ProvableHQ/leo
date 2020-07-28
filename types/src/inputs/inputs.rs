use crate::{Input, InputValue, ProgramInputs, ProgramState};
use leo_inputs::{
    files::{File, TableOrSection},
    InputParserError,
};

#[derive(Clone)]
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

    pub fn get_inputs(&self) -> Vec<Option<InputValue>> {
        self.inputs.main.0.clone()
    }

    pub fn set_inputs(&mut self, inputs: Vec<Option<InputValue>>) {
        self.inputs.main.0 = inputs;
    }

    pub fn set_inputs_size(&mut self, size: usize) {
        self.inputs.main.0 = vec![None; size];
    }

    pub fn parse_program_input_file(
        &mut self,
        file: File,
        expected_inputs: Vec<Input>,
    ) -> Result<(), InputParserError> {
        for entry in file.entries.into_iter() {
            match entry {
                TableOrSection::Section(section) => {
                    self.inputs.store_definitions(section, &expected_inputs)?;
                }
                TableOrSection::Table(table) => {
                    self.state.store_definitions(table, &expected_inputs)?;
                }
            }
        }

        Ok(())
    }
}
