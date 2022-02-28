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
use leo_errors::AstError;

/// Input data which includes [`ProgramInput`] and [`ProgramState`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Input {
    pub program_input: ProgramInput,
    pub program_state: ProgramState,
}

/// A raw unprocessed input or state file data. Used for future conversion
/// into [`ProgramInput`] or [`ProgramState`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParsedInputFile {
    pub sections: Vec<Section>,
}

impl Input {
    /// Serializes the ast into a JSON string.
    pub fn to_json_string(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self).map_err(|e| AstError::failed_to_convert_ast_to_json_string(&e))?)
    }
}
