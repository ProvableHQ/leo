// Copyright (C) 2019-2026 Provable Inc.
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

use super::SsaConstPropagationVisitor;

use leo_ast::{AstReconstructor, Constructor, Function, ProgramReconstructor};

impl ProgramReconstructor for SsaConstPropagationVisitor<'_> {
    fn reconstruct_function(&mut self, mut input: Function) -> Function {
        // Reset the constants map for each function.
        // In SSA form, each function has its own scope, so we can clear the map.
        self.constants.clear();
        // Traverse the function body.
        input.block = self.reconstruct_block(input.block).0;
        self.constants.clear();
        input
    }

    fn reconstruct_constructor(&mut self, mut input: Constructor) -> Constructor {
        // Reset the constants map for each constructor.
        self.constants.clear();
        // Traverse the constructor body.
        input.block = self.reconstruct_block(input.block).0;
        self.constants.clear();
        input
    }
}
