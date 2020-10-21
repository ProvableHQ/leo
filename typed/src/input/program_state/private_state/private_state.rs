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

use crate::{Record, StateLeaf};
use leo_input::{
    sections::{Header, Section},
    InputParserError,
};

#[derive(Clone, PartialEq, Eq, Default)]
pub struct PrivateState {
    record: Record,
    state_leaf: StateLeaf,
}

#[allow(clippy::len_without_is_empty)]
impl PrivateState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private input variables.
    pub fn empty(&self) -> Self {
        let record = self.record.empty();
        let state_leaf = self.state_leaf.empty();

        Self { record, state_leaf }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;

        // add record variable
        if self.record.is_present() {
            len += 1;
        }

        // add state_leaf variable
        if self.state_leaf.is_present() {
            len += 1;
        }

        len
    }

    /// Parse all input variables included in a file and store them in `self`.
    pub fn parse(&mut self, sections: Vec<Section>) -> Result<(), InputParserError> {
        for section in sections {
            match section.header {
                Header::Record(_state) => self.record.parse(section.definitions)?,
                Header::StateLeaf(_state_leaf) => self.state_leaf.parse(section.definitions)?,
                header => return Err(InputParserError::private_section(header)),
            }
        }

        Ok(())
    }

    /// Returns the runtime record input values
    pub fn get_record(&self) -> &Record {
        &self.record
    }

    /// Returns the runtime state leaf input values
    pub fn get_state_leaf(&self) -> &StateLeaf {
        &self.state_leaf
    }
}
