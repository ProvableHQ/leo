// Copyright (C) 2019-2025 Provable Inc.
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

use super::DestructuringVisitor;

use leo_ast::{Function, ProgramReconstructor, StatementReconstructor};

impl ProgramReconstructor for DestructuringVisitor<'_> {
    fn reconstruct_function(&mut self, input: Function) -> Function {
        // Set the `is_async` flag before reconstructing the block.
        self.is_async = input.variant.is_async_function();
        // Reconstruct the block.
        let block = self.reconstruct_block(input.block).0;
        // Reset the `is_async` flag.
        self.is_async = false;
        Function { block, ..input }
    }
}
