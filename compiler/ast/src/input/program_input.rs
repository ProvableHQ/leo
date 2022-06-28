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

use crate::ParamMode;

use super::*;

/// Processed Program input.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProgramInput {
    pub constants: Definitions,
    pub private: Definitions,
    pub public: Definitions,
    pub registers: Definitions,
}

impl TryFrom<InputAst> for ProgramInput {
    type Error = LeoError;
    fn try_from(input: InputAst) -> Result<Self> {
        let mut constants = IndexMap::new();
        let mut private = IndexMap::new();
        let mut public = IndexMap::new();
        let mut registers = IndexMap::new();

        for section in input.sections {
            match section.name {
                sym::main => {
                    for definition in section.definitions.into_iter() {
                        match definition.mode {
                            ParamMode::Const => {
                                constants.insert(
                                    definition.name.name,
                                    InputValue::try_from((definition.type_, definition.value))?,
                                );
                            }
                            ParamMode::Private => {
                                private.insert(
                                    definition.name.name,
                                    InputValue::try_from((definition.type_, definition.value))?,
                                );
                            }
                            ParamMode::Public => {
                                public.insert(
                                    definition.name.name,
                                    InputValue::try_from((definition.type_, definition.value))?,
                                );
                            }
                        }
                    }
                }
                sym::registers => {
                    for definition in section.definitions.into_iter() {
                        registers.insert(
                            definition.name.name,
                            InputValue::try_from((definition.type_, definition.value))?,
                        );
                    }
                }
                _ => {
                    return Err(
                        InputError::unexpected_section(&["main", "registers"], section.name, section.span).into(),
                    )
                }
            }
        }

        Ok(ProgramInput {
            constants,
            private,
            public,
            registers,
        })
    }
}
