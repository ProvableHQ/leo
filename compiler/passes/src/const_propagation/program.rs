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

use super::ConstPropagationVisitor;

use leo_ast::{Function, Node, ProgramReconstructor, ProgramScope, Statement, StatementReconstructor as _};

impl ProgramReconstructor for ConstPropagationVisitor<'_> {
    fn reconstruct_program_scope(&mut self, mut input: ProgramScope) -> ProgramScope {
        self.program = input.program_id.name.name;

        for (_sym, c) in input.consts.iter_mut() {
            let Statement::Const(declaration) = self.reconstruct_const(std::mem::take(c)).0 else {
                panic!("`reconstruct_const` always returns `Statement::Const`");
            };
            *c = declaration;
        }

        for (_sym, f) in input.functions.iter_mut() {
            *f = self.reconstruct_function(std::mem::take(f));
        }

        input
    }

    fn reconstruct_function(&mut self, mut function: Function) -> Function {
        self.in_scope(function.id(), |slf| {
            function.block = slf.reconstruct_block(function.block).0;
            function
        })
    }
}
