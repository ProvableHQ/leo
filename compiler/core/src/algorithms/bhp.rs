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

use leo_ast::{Circuit, CircuitMember, Function, Identifier};
use leo_span::sym;

pub struct BHP256(Circuit);

impl BHP256 {
    pub fn new() -> Self {
        Self(Circuit {
            circuit_name: Identifier::new(sym::bhp256),
            members: vec![
                CircuitMember::CircuitFunction(Box::new(
                    Function {
                        identifier: Identifier {},
                        input: vec![],
                        output: Type::Address,
                        core_mapping: Cell::new(None),
                        block: Block {},
                        span: Default::default()
                    }
                ))
            ]
        })
    }
}

pub struct BHP512(Circuit);