// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{InputValue, MainInput, ProgramInput, ProgramState, Record, Registers, State, StateLeaf};
use leo_input::{
    files::{File, TableOrSection},
    InputParserError,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Input {
    name: String,
    program_input: ProgramInput,
    program_state: ProgramState,
}

impl Input {
    pub fn new() -> Self {
        Self {
            name: "default".to_owned(),
            program_input: ProgramInput::new(),
            program_state: ProgramState::new(),
        }
    }

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private input variables.
    pub fn empty(&self) -> Self {
        let input = self.program_input.empty();
        let state = self.program_state.empty();

        Self {
            name: self.name.clone(),
            program_input: input,
            program_state: state,
        }
    }

    /// Returns the number of input variables to pass into the `main` program function
    pub fn len(&self) -> usize {
        self.program_input.len() + self.program_state.len()
    }

    /// Manually set the input variables to the `main` program function
    pub fn set_main_input(&mut self, input: MainInput) {
        self.program_input.main = input;
    }

    /// Parse all input variables included in a file and store them in `self`.
    pub fn parse_input(&mut self, file: File) -> Result<(), InputParserError> {
        for entry in file.entries.into_iter() {
            match entry {
                TableOrSection::Section(section) => {
                    self.program_input.parse(section)?;
                }
                TableOrSection::Table(table) => return Err(InputParserError::table(table)),
            }
        }

        Ok(())
    }

    /// Parse all state variables included in a file and store them in `self`.
    pub fn parse_state(&mut self, file: File) -> Result<(), InputParserError> {
        for entry in file.entries.into_iter() {
            match entry {
                TableOrSection::Section(section) => return Err(InputParserError::section(section.header)),
                TableOrSection::Table(table) => {
                    self.program_state.parse(table)?;
                }
            }
        }

        Ok(())
    }

    /// Returns the main function input value with the given `name`
    pub fn get(&self, name: &String) -> Option<Option<InputValue>> {
        self.program_input.get(name)
    }

    /// Returns the runtime register input values
    pub fn get_registers(&self) -> &Registers {
        self.program_input.get_registers()
    }

    /// Returns the runtime record input values
    pub fn get_record(&self) -> &Record {
        self.program_state.get_record()
    }

    /// Returns the runtime state input values
    pub fn get_state(&self) -> &State {
        self.program_state.get_state()
    }

    /// Returns the runtime state leaf input values
    pub fn get_state_leaf(&self) -> &StateLeaf {
        self.program_state.get_state_leaf()
    }
}
