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

use super::*;

/// Processed Program input.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProgramInput {
    pub main: Definitions,
    pub registers: Definitions,
    pub constants: Definitions,
}

impl TryFrom<ParsedInputFile> for ProgramInput {
    type Error = LeoError;
    fn try_from(input: ParsedInputFile) -> Result<Self> {
        let mut main = IndexMap::new();
        let mut registers = IndexMap::new();
        let mut constants = IndexMap::new();

        for section in input.sections {
            let target = match section.name {
                sym::main => &mut main,
                sym::registers => &mut registers,
                sym::constants => &mut constants,
                _ => {
                    return Err(InputError::unexpected_section(
                        &["main", "registers", "constants"],
                        section.name,
                        &section.span,
                    )
                    .into())
                }
            };

            for definition in section.definitions {
                target.insert(
                    Parameter::from(definition.clone()),
                    InputValue::try_from((definition.type_, definition.value))?,
                );
            }
        }

        Ok(ProgramInput {
            main,
            registers,
            constants,
        })
    }
}
