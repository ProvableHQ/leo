use crate::{InputValue, MainInput, Registers};
use leo_input::{
    sections::{Header, Section},
    InputParserError,
};

#[derive(Clone, PartialEq, Eq)]
pub struct ProgramInput {
    pub main: MainInput,
    registers: Registers,
}

impl ProgramInput {
    pub fn new() -> Self {
        Self {
            main: MainInput::new(),
            registers: Registers::new(),
        }
    }

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private input values.
    pub fn empty(&self) -> Self {
        let main = self.main.empty();
        let registers = self.registers.empty();

        Self { main, registers }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;

        // add main input variables
        len += self.main.len();

        // add registers
        if self.registers.is_present() {
            len += 1;
        }

        len
    }

    /// Parse each input included in a file and store them in `self`.
    pub fn parse(&mut self, section: Section) -> Result<(), InputParserError> {
        match section.header {
            Header::Main(_main) => self.main.parse(section.definitions),
            Header::Registers(_registers) => self.registers.parse(section.definitions),
            header => Err(InputParserError::input_section_header(header)),
        }
    }

    /// Returns the main function input value with the given `name`
    pub fn get(&self, name: &String) -> Option<Option<InputValue>> {
        self.main.get(name)
    }

    /// Returns the runtime register input values
    pub fn get_registers(&self) -> &Registers {
        &self.registers
    }
}
