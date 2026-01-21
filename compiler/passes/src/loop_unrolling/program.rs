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

use leo_ast::*;

use super::UnrollingVisitor;

impl ProgramReconstructor for UnrollingVisitor<'_> {
    fn reconstruct_stub(&mut self, input: Stub) -> Stub {
        // Set the current program.
        self.program = input.stub_id.name.name;
        Stub {
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function_stub(f))).collect(),
            ..input
        }
    }

    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        // Set the current program.
        self.program = input.program_id.name.name;
        ProgramScope {
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect(),
            constructor: input.constructor.map(|c| self.reconstruct_constructor(c)),
            ..input
        }
    }

    // Reconstruct the function body, entering the associated scopes as needed.
    fn reconstruct_function(&mut self, function: Function) -> Function {
        self.in_scope(function.id(), |slf| Function { block: slf.reconstruct_block(function.block).0, ..function })
    }

    // Reconstruct the constructor body, entering the associated scopes as needed.
    fn reconstruct_constructor(&mut self, constructor: Constructor) -> Constructor {
        self.in_scope(constructor.id(), |slf| Constructor {
            block: slf.reconstruct_block(constructor.block).0,
            ..constructor
        })
    }
}
