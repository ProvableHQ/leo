// Copyright (C) 2019-2023 Aleo Systems Inc.
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

/// A single section in an input or a state file.
/// An example of a section would be: `[main]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub name: Symbol,
    pub definitions: Vec<Definition>,
    pub span: Span,
}

impl Section {
    pub fn name(&self) -> String {
        self.name.to_string()
    }
}
