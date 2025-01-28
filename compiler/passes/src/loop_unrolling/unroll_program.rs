// Copyright (C) 2019-2025 Aleo Systems Inc.
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

use crate::Unroller;

impl ProgramReconstructor for Unroller<'_> {
    fn reconstruct_stub(&mut self, input: Stub) -> Stub {
        // Set the current program.
        self.current_program = Some(input.stub_id.name.name);
        Stub {
            imports: input.imports,
            stub_id: input.stub_id,
            consts: input.consts,
            structs: input.structs,
            mappings: input.mappings,
            span: input.span,
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function_stub(f))).collect(),
        }
    }

    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        // Set the current program.
        self.current_program = Some(input.program_id.name.name);
        // Don't need to reconstructed consts, just need to add them to constant propagation table
        input.consts.into_iter().for_each(|(_, c)| {
            self.reconstruct_const(c);
        });
        // Reconstruct the program scope
        ProgramScope {
            program_id: input.program_id,
            structs: input.structs,
            mappings: input.mappings,
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect(),
            consts: Vec::new(),
            span: input.span,
        }
    }

    // Reconstruct the function body, entering the associated scopes as needed.
    fn reconstruct_function(&mut self, function: Function) -> Function {
        self.in_scope(function.id(), |slf| Function {
            annotations: function.annotations,
            variant: function.variant,
            identifier: function.identifier,
            input: function.input,
            output: function.output,
            output_type: function.output_type,
            block: slf.reconstruct_block(function.block).0,
            span: function.span,
            id: function.id,
        })
    }
}
