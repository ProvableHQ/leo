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
            let target = match section.name {
                sym::state => &mut state,
                sym::record => &mut record,
                sym::state_leaf => &mut state_leaf,
                _ => todo!("throw an error for illegal section"),
            };

            for definition in section.definitions {
                target.insert(
                    Parameter::from(definition.clone()),
                    InputValue::try_from((definition.type_, definition.value))?,
                );
            }
        }

        Ok(ProgramState {
            state,
            record,
            state_leaf,
        })
    }
}
