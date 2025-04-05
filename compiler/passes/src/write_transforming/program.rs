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

use super::WriteTransformingVisitor;

use leo_ast::{Function, ProgramReconstructor, StatementReconstructor as _};

impl ProgramReconstructor for WriteTransformingVisitor<'_> {
    fn reconstruct_function(&mut self, input: Function) -> Function {
        // Since the input parameters may be structs or arrays that are written to,
        // we may need to define variable members.
        let mut statements = Vec::new();
        for parameter in input.input.iter() {
            self.define_variable_members(parameter.identifier, &mut statements);
        }
        let mut block = self.reconstruct_block(input.block).0;
        statements.extend(block.statements);
        block.statements = statements;
        Function { block, ..input }
    }

    fn reconstruct_program_scope(&mut self, input: leo_ast::ProgramScope) -> leo_ast::ProgramScope {
        self.program = input.program_id.name.name;
        leo_ast::ProgramScope {
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect(),
            ..input
        }
    }
}
