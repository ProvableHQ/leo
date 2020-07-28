use crate::{Input, MainInputs, Registers};
use leo_inputs::{
    sections::{Header, Section},
    InputParserError,
};

#[derive(Clone, PartialEq, Eq)]
pub struct ProgramInputs {
    pub main: MainInputs,
    pub registers: Registers,
}

impl ProgramInputs {
    pub fn new() -> Self {
        Self {
            main: MainInputs::new(),
            registers: Registers::new(),
        }
    }

    pub fn store_definitions(
        &mut self,
        section: Section,
        expected_inputs: &Vec<Input>,
    ) -> Result<(), InputParserError> {
        match section.header {
            Header::Main(_main) => self.main.store_definitions(section.definitions, &expected_inputs),
            Header::Registers(_registers) => self.registers.store_definitions(section.definitions, &expected_inputs),
            header => Err(InputParserError::input_section_header(header)),
        }
    }
}
