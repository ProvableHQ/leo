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

use crate::FunctionInliner;
use indexmap::IndexMap;

use leo_ast::{Finalize, Function, ProgramReconstructor, ProgramScope, StatementReconstructor};

impl ProgramReconstructor for FunctionInliner<'_> {
    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        let mut reconstructed_functions = IndexMap::new();

        // TODO: Reconstruct each of the functions in post-order and add them to the function map.
        // TODO: Once implemented, we do not need to reorder functions during code generation.

        ProgramScope {
            program_id: input.program_id,
            structs: input.structs,
            mappings: input.mappings,
            functions: reconstructed_functions,
            span: input.span,
        }
    }

    fn reconstruct_function(&mut self, input: Function) -> Function {
        // TODO: Reconstruct the function in the correct order
        Function {
            annotations: input.annotations,
            variant: input.variant,
            identifier: input.identifier,
            input: input.input,
            output: input.output,
            output_type: input.output_type,
            block: self.reconstruct_block(input.block).0,
            finalize: input.finalize.map(|finalize| Finalize {
                identifier: finalize.identifier,
                input: finalize.input,
                output: finalize.output,
                output_type: finalize.output_type,
                block: self.reconstruct_block(finalize.block).0,
                span: finalize.span,
            }),
            span: input.span,
        }
    }
}
