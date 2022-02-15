// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{Expression, Identifier, InputValue, Type};
use leo_errors::{LeoError, Result, emitter::Handler};
use leo_span::{sym, Span, Symbol};

use std::path::PathBuf;

use indexmap::IndexMap;

#[derive(Debug, Clone, Default)]
pub struct Input {
    pub program_input: ProgramInput,
    pub program_state: ProgramState,
}

impl Input {
    /// Create an empty struct for future `parse` calls.
    pub fn new() -> Self {
        Self {
            program_input: Default::default(),
            program_state: Default::default(),
        }
    }

    pub fn parse_input(
        &mut self,
        handler: &Handler,
        input_file_path: PathBuf,
        input_file_string: String,
    ) -> Result<&mut Self> {
        
        
        Ok(self)
    }

    pub fn parse_state(
        &mut self,
        handler: &Handler,
        state_file_path: PathBuf,
        state_file_string: String,
    ) -> Result<&mut Self> {
        Ok(self)
    }
}


#[derive(Clone, Debug)]
pub struct ParsedInputFile {
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    pub type_: Type,
    pub name: Identifier,
    pub value: Expression,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Parameter {
    pub variable: Identifier,
    pub type_: Type,
    pub span: Span,
}

impl From<Definition> for Parameter {
    fn from(definition: Definition) -> Self {
        Self {
            variable: definition.name,
            type_: definition.type_,
            span: definition.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: Symbol,
    pub definitions: Vec<Definition>,
    pub is_public: bool,
    pub span: Span,
}

type Definitions = IndexMap<Parameter, InputValue>;

#[derive(Debug, Clone, Default)]
pub struct ProgramState {
    state: Definitions,
    record: Definitions,
    state_leaf: Definitions,
}

impl TryFrom<ParsedInputFile> for ProgramState {
    type Error = LeoError;
    fn try_from(input: ParsedInputFile) -> Result<Self> {
        let mut state = IndexMap::new();
        let mut record = IndexMap::new();
        let mut state_leaf = IndexMap::new();

        for section in input.sections {
            let mut assignments = IndexMap::new();
            for definition in section.definitions {
                assignments.insert(
                    Parameter::from(definition.clone()),
                    InputValue::try_from((definition.type_, definition.value))?,
                );
            }

            match section.name {
                sym::state => {
                    state = assignments;
                }
                sym::record => {
                    record = assignments;
                }
                sym::state_leaf => {
                    state_leaf = assignments;
                }
                _ => todo!("throw an error for illegal section"),
            };
        }

        Ok(ProgramState {
            state,
            record,
            state_leaf,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProgramInput {
    main: Definitions,
    registers: Definitions,
    constants: Definitions,
}

impl TryFrom<ParsedInputFile> for ProgramInput {
    type Error = LeoError;
    fn try_from(input: ParsedInputFile) -> Result<Self> {
        let mut main = IndexMap::new();
        let mut registers = IndexMap::new();
        let mut constants = IndexMap::new();

        for section in input.sections {
            let mut assignments = IndexMap::new();
            for definition in section.definitions {
                assignments.insert(
                    Parameter::from(definition.clone()),
                    InputValue::try_from((definition.type_, definition.value))?,
                );
            }

            match section.name {
                sym::main => {
                    main = assignments;
                }
                sym::registers => {
                    registers = assignments;
                }
                sym::constants => {
                    constants = assignments;
                }
                _ => todo!("throw an error for illegal section"),
            };
        }

        Ok(ProgramInput {
            main,
            registers,
            constants,
        })
    }
}
