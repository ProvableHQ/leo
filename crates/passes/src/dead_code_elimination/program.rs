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

use super::DeadCodeEliminatingVisitor;

use leo_ast::{AstReconstructor, Constructor, Function, Location, ProgramReconstructor, UpgradeVariant};
use leo_errors::StaticAnalyzerError;

impl ProgramReconstructor for DeadCodeEliminatingVisitor<'_> {
    fn reconstruct_program_scope(&mut self, mut input: leo_ast::ProgramScope) -> leo_ast::ProgramScope {
        self.program_name = input.program_id.name.name;
        input.functions = input
            .functions
            .into_iter()
            .filter(|(name, fun)| {
                use leo_ast::Variant::*;
                let location = Location::new(self.program_name, vec![*name]);
                let call_count = *self.state.call_count.get(&location).unwrap();

                match fun.variant {
                    Fn => call_count > 0,
                    FinalFn => false,
                    EntryPoint | Finalize => true,
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|(i, f)| (i, self.reconstruct_function(f)))
            .collect();
        input.constructor = input.constructor.map(|c| self.reconstruct_constructor(c));
        input
    }

    fn reconstruct_function(&mut self, mut input: Function) -> Function {
        // Reset the state of the dead code eliminator.
        self.used_variables.clear();
        // Traverse the function body.
        input.block = self.reconstruct_block(input.block).0;
        input
    }

    fn reconstruct_constructor(&mut self, mut input: Constructor) -> Constructor {
        // Reset the state of the dead code eliminator.
        self.used_variables.clear();
        // Traverse the constructor body.
        input.block = self.reconstruct_block(input.block).0;
        // If the reconstructed input has no statements, and the constructor is a custom one, return an error.
        if input
            .get_upgrade_variant_with_network(self.state.network)
            .expect("Type checking guarantees that the upgrade variant is valid")
            == UpgradeVariant::Custom
            && input.block.statements.is_empty()
        {
            self.state.handler.emit_err(StaticAnalyzerError::custom_error(
                "The `@custom` constructor has no statements after dead code elimination.",
                Some("Add a non-trivial implementation"),
                input.span,
            ))
        }
        input
    }
}
