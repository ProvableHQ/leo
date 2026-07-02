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

use super::SsaConstPropagationVisitor;

use leo_ast::{AstReconstructor, Constructor, Function, Library, Location, ProgramScope, Statement, UnitReconstructor};

impl UnitReconstructor for SsaConstPropagationVisitor<'_> {
    fn reconstruct_library(&mut self, input: Library) -> Library {
        // Library functions have already been inlined into the consuming program by the
        // function-inlining pass. Pass the library stub through unchanged.
        input
    }

    fn reconstruct_program_scope(&mut self, input: ProgramScope) -> ProgramScope {
        self.program = input.program_id.as_symbol();
        self.composites = input
            .composites
            .iter()
            .map(|(_, composite)| (Location::new(self.program, vec![composite.identifier.name]), composite.clone()))
            .collect();

        ProgramScope {
            program_id: input.program_id,
            parents: input.parents.into_iter().map(|(s, t)| (s, self.reconstruct_type(t).0)).collect(),
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match self.reconstruct_const(c) {
                    (Statement::Const(declaration), _) => (i, declaration),
                    _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
            composites: input.composites.into_iter().map(|(i, c)| (i, self.reconstruct_composite(c))).collect(),
            mappings: input.mappings.into_iter().map(|(id, mapping)| (id, self.reconstruct_mapping(mapping))).collect(),
            storage_variables: input
                .storage_variables
                .into_iter()
                .map(|(id, storage_variable)| (id, self.reconstruct_storage_variable(storage_variable)))
                .collect(),
            functions: input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect(),
            interfaces: input.interfaces.into_iter().map(|(i, int)| (i, self.reconstruct_interface(int))).collect(),
            constructor: input.constructor.map(|c| self.reconstruct_constructor(c)),
            span: input.span,
        }
    }

    fn reconstruct_function(&mut self, mut input: Function) -> Function {
        if self.tracks_optional_unwraps() && !input.variant.is_finalize_context() {
            return input;
        }

        // In SSA form, each function has its own scope, so analysis facts are local.
        self.clear_tracked_values();
        // Traverse the function body.
        input.block = self.reconstruct_block(input.block).0;
        self.clear_tracked_values();
        input
    }

    fn reconstruct_constructor(&mut self, mut input: Constructor) -> Constructor {
        if self.tracks_optional_unwraps() {
            return input;
        }

        // Constructors also have their own SSA scope.
        self.clear_tracked_values();
        // Traverse the constructor body.
        input.block = self.reconstruct_block(input.block).0;
        self.clear_tracked_values();
        input
    }
}
