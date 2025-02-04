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

use crate::{DeadCodeEliminator, VariableTracker};

use leo_ast::{Function, Node as _, ProgramReconstructor, ProgramVisitor, StatementReconstructor, StatementVisitor};

impl ProgramReconstructor for DeadCodeEliminator<'_> {
    fn reconstruct_function(&mut self, mut input: Function) -> Function {
        self.in_scope(input.id(), |slf| {
            input.block = slf.reconstruct_block(input.block).0;
            input
        })
    }
}

impl ProgramVisitor for VariableTracker<'_> {
    fn visit_program(&mut self, input: &leo_ast::Program) {
        self.symbol_table.clear_used_symbols();
        input.program_scopes.values().for_each(|scope| self.visit_program_scope(scope));
    }

    fn visit_program_scope(&mut self, input: &leo_ast::ProgramScope) {
        input.functions.iter().for_each(|(_, c)| self.visit_function(c));
    }

    fn visit_function(&mut self, input: &Function) {
        self.in_scope(input.id(), |slf| slf.visit_block(&input.block));
    }
}
