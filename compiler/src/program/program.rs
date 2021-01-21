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

//! An in memory store to keep track of defined names when constraining a Leo program.

use crate::{value::ConstrainedValue, GroupType};

use leo_asg::Program;
use snarkvm_models::curves::{Field, PrimeField};

use indexmap::IndexMap;
use uuid::Uuid;

pub struct ConstrainedProgram<F: Field + PrimeField, G: GroupType<F>> {
    pub asg: Program,
    identifiers: IndexMap<Uuid, ConstrainedValue<F, G>>,
    pub self_alias: Option<(Uuid, Uuid)>, // current self id -> id that self resolves to
}

pub fn new_scope(outer: &str, inner: &str) -> String {
    format!("{}_{}", outer, inner)
}

pub fn is_in_scope(current_scope: &str, desired_scope: &str) -> bool {
    current_scope.ends_with(desired_scope)
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn new(asg: Program) -> Self {
        Self {
            asg,
            identifiers: IndexMap::new(),
            self_alias: None,
        }
    }

    pub(crate) fn store(&mut self, name: Uuid, value: ConstrainedValue<F, G>) {
        if let Some((from, _)) = &self.self_alias {
            if &name == from {
                panic!("attempted to assign to self alias");
            }
        }
        self.identifiers.insert(name, value);
    }

    pub(crate) fn get(&self, name: &Uuid) -> Option<&ConstrainedValue<F, G>> {
        if let Some((from, to)) = &self.self_alias {
            if name == from {
                return self.identifiers.get(to);
            }
        }
        self.identifiers.get(name)
    }

    pub(crate) fn get_mut(&mut self, name: &Uuid) -> Option<&mut ConstrainedValue<F, G>> {
        if let Some((from, to)) = &self.self_alias {
            if name == from {
                return self.identifiers.get_mut(to);
            }
        }
        self.identifiers.get_mut(name)
    }
}
