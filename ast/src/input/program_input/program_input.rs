// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{InputValue, MainInput, Registers};
use leo_input::{
    sections::{Header, Section},
    InputParserError,
};

#[derive(Clone, PartialEq, Eq, Default)]
pub struct ProgramInput {
    pub main: MainInput,
    registers: Registers,
}

#[allow(clippy::len_without_is_empty)]
impl ProgramInput {
    pub fn new() -> Self {
        Self::default()
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
    #[allow(clippy::ptr_arg)]
    pub fn get(&self, name: &String) -> Option<Option<InputValue>> {
        self.main.get(name)
    }

    /// Returns the runtime register input values
    pub fn get_registers(&self) -> &Registers {
        &self.registers
    }
}
