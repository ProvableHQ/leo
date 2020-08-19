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

use crate::{PrivateState, PublicState, Record, State, StateLeaf};
use leo_input::{
    tables::{Table, Visibility},
    InputParserError,
};

#[derive(Clone, PartialEq, Eq)]
pub struct ProgramState {
    public: PublicState,
    private: PrivateState,
}

impl ProgramState {
    pub fn new() -> Self {
        Self {
            public: PublicState::new(),
            private: PrivateState::new(),
        }
    }

    /// Returns an empty version of this struct with `None` values.
    /// Called during constraint synthesis to provide private input variables.
    pub fn empty(&self) -> Self {
        let public = self.public.empty();
        let private = self.private.empty();

        Self { public, private }
    }

    pub fn len(&self) -> usize {
        self.public.len() + self.private.len()
    }

    /// Parse all input variables included in a file and store them in `self`.
    pub fn parse(&mut self, table: Table) -> Result<(), InputParserError> {
        match table.visibility {
            Visibility::Private(_private) => self.private.parse(table.sections),
            Visibility::Public(_public) => self.public.parse(table.sections),
        }
    }

    /// Returns the runtime record input values
    pub fn get_record(&self) -> &Record {
        self.private.get_record()
    }

    /// Returns the runtime state input values
    pub fn get_state(&self) -> &State {
        self.public.get_state()
    }

    /// Returns the runtime state leaf input values
    pub fn get_state_leaf(&self) -> &StateLeaf {
        self.private.get_state_leaf()
    }
}
