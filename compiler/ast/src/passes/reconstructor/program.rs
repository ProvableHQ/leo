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

use crate::*;

/// A Reconstructor trait for the program represented by the AST.
pub trait ProgramReconstructor: StatementReconstructor {
    fn reconstruct_program(&mut self, input: Program) -> Program {
        Program {
            imports: input
                .imports
                .into_iter()
                .map(|(id, import)| (id, (self.reconstruct_import(import.0), import.1)))
                .collect(),
            program_scopes: input
                .program_scopes
                .into_iter()
                .map(|(id, scope)| (id, self.reconstruct_program_scope(scope)))
                .collect(),
        }
    }

    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        ProgramScope {
            program_id: input.program_id,
            structs: input
                .structs
                .into_iter()
                .map(|(i, c)| (i, self.reconstruct_struct(c)))
                .collect(),
            mappings: input
                .mappings
                .into_iter()
                .map(|(id, mapping)| (id, self.reconstruct_mapping(mapping)))
                .collect(),
            functions: input
                .functions
                .into_iter()
                .map(|(i, f)| (i, self.reconstruct_function(f)))
                .collect(),
            span: input.span,
        }
    }

    fn reconstruct_function(&mut self, input: Function) -> Function {
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

    fn reconstruct_struct(&mut self, input: Struct) -> Struct {
        input
    }

    fn reconstruct_import(&mut self, input: Program) -> Program {
        self.reconstruct_program(input)
    }

    fn reconstruct_mapping(&mut self, input: Mapping) -> Mapping {
        input
    }
}
