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

use crate::State;
use leo_input::{
    sections::{Header, Section},
    InputParserError,
};

#[derive(Clone, PartialEq, Eq, Default)]
pub struct PublicState {
    state: State,
}

#[allow(clippy::len_without_is_empty)]
impl PublicState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private input variables.
    pub fn empty(&self) -> Self {
        let state = self.state.empty();

        Self { state }
    }

    pub fn len(&self) -> usize {
        if self.state.is_present() {
            1usize
        } else {
            0usize
        }
    }

    /// Parse all input variables included in a file and store them in `self`.
    pub fn parse(&mut self, sections: Vec<Section>) -> Result<(), InputParserError> {
        for section in sections {
            match section.header {
                Header::State(_state) => self.state.parse(section.definitions)?,
                header => return Err(InputParserError::public_section(header)),
            }
        }

        Ok(())
    }

    /// Returns the runtime state input values
    pub fn get_state(&self) -> &State {
        &self.state
    }
}
