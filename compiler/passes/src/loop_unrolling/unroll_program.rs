// Copyright (C) 2019-2023 Aleo Systems Inc.
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
    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        // Don't need to reconstructed consts, just need to add them to constant propagation table
        input.consts.into_iter().for_each(|(_, c)| {
            self.reconstruct_const(c);
        });
        ProgramScope {
            program_id: input.program_id,
            structs: input.structs,
            mappings: input.mappings,
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect(),
            consts: Vec::new(),
            span: input.span,
        }
    }

    fn reconstruct_function(&mut self, function: Function) -> Function {
        // Lookup function metadata in the symbol table.
        // Note that this unwrap is safe since function metadata is stored in a prior pass.
        let function_index = self.symbol_table.borrow().lookup_fn_symbol(function.identifier.name).unwrap().id;

        // Enter the function's scope.
        let previous_function_index = self.enter_scope(function_index);

        let previous_scope_index = self.enter_scope(self.scope_index);

        let block = self.reconstruct_block(function.block).0;

        self.exit_scope(previous_scope_index);

        let finalize = function.finalize.map(|finalize| {
            let previous_scope_index = self.enter_scope(self.scope_index);

            let block = self.reconstruct_block(finalize.block).0;

            self.exit_scope(previous_scope_index);

            Finalize {
                identifier: finalize.identifier,
                input: finalize.input,
                output: finalize.output,
                output_type: finalize.output_type,
                block,
                span: finalize.span,
                id: finalize.id,
            }
        });

        // Reconstruct the function block.
        let reconstructed_function = Function {
            annotations: function.annotations,
            variant: function.variant,
            identifier: function.identifier,
            input: function.input,
            output: function.output,
            output_type: function.output_type,
            block,
            finalize,
            span: function.span,
            id: function.id,
        };

        // Exit the function's scope.
        self.exit_scope(previous_function_index);

        reconstructed_function
    }
}
