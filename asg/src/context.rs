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

use std::cell::Cell;
use std::unimplemented;

use typed_arena::Arena;

use crate::ArenaNode;

pub struct AsgContextInner<'a> {
    pub arena: &'a Arena<ArenaNode<'a>>,
    pub next_id: Cell<u32>,
}

impl<'a> AsgContextInner<'a> {
    pub fn new(arena: &'a Arena<ArenaNode<'a>>) -> &'a Self {
        match arena.alloc(ArenaNode::Inner(AsgContextInner {
            arena,
            next_id: Cell::new(0),
        })) {
            ArenaNode::Inner(x) => x,
            _ => unimplemented!(),
        }
    }

    pub fn get_id(&self) -> u32 {
        let next_id = self.next_id.get();
        self.next_id.replace(next_id + 1);
        next_id
    }
}

pub type AsgContext<'a> = &'a AsgContextInner<'a>;
