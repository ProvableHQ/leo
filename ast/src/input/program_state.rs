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

/// Processed Program state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProgramState {
    pub state: Definitions,
}

impl TryFrom<ParsedInputFile> for ProgramState {
    type Error = LeoError;
    fn try_from(input: ParsedInputFile) -> Result<Self> {
        let mut state = IndexMap::new();

        for section in input.sections {
            if matches!(section.name, sym::state | sym::record | sym::state_leaf) {
                for definition in section.definitions {
                    state.insert(
                        definition.name.name,
                        InputValue::try_from((definition.type_, definition.value))?,
                    );
                }
            } else {
                return Err(InputError::unexpected_section(
                    &["state", "record", "state_leaf"],
                    section.name,
                    &section.span,
                )
                .into());
            }
        }

        Ok(ProgramState { state })
    }
}
