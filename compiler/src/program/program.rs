// Copyright (C) 2019-2021 Aleo Systems Inc.
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

//! An in memory store to keep track of defined names when constraining a Leo program.

use crate::{value::ConstrainedValue, GroupType};

use leo_asg::Program;
use snarkvm_fields::PrimeField;

use indexmap::IndexMap;

pub struct ConstrainedProgram<'a, F: PrimeField, G: GroupType<F>> {
    pub asg: Program<'a>,
    identifiers: IndexMap<u32, ConstrainedValue<'a, F, G>>,
}

impl<'a, F: PrimeField, G: GroupType<F>> ConstrainedProgram<'a, F, G> {
    pub fn new(asg: Program<'a>) -> Self {
        Self {
            asg,
            identifiers: IndexMap::new(),
        }
    }

    pub(crate) fn store(&mut self, id: u32, value: ConstrainedValue<'a, F, G>) {
        self.identifiers.insert(id, value);
    }

    pub(crate) fn get(&self, id: u32) -> Option<&ConstrainedValue<'a, F, G>> {
        self.identifiers.get(&id)
    }

    pub(crate) fn get_mut(&mut self, id: u32) -> Option<&mut ConstrainedValue<'a, F, G>> {
        self.identifiers.get_mut(&id)
    }
}
