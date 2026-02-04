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

use super::CommonSubexpressionEliminatingVisitor;

use leo_ast::{AstReconstructor, Constructor, Function, Module, ProgramReconstructor};

impl ProgramReconstructor for CommonSubexpressionEliminatingVisitor<'_> {
    fn reconstruct_program_scope(&mut self, mut input: leo_ast::ProgramScope) -> leo_ast::ProgramScope {
        input.functions = input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect();
        input.constructor = input.constructor.map(|c| self.reconstruct_constructor(c));
        input
    }

    fn reconstruct_function(&mut self, mut input: Function) -> Function {
        input.block = self.reconstruct_block(input.block).0;
        input
    }

    fn reconstruct_constructor(&mut self, mut input: Constructor) -> Constructor {
        input.block = self.reconstruct_block(input.block).0;
        input
    }

    fn reconstruct_module(&mut self, mut input: Module) -> Module {
        input.functions = input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect();
        input
    }
}
