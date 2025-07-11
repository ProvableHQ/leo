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

use super::ProcessingScriptVisitor;

use leo_ast::{AstReconstructor as _, Function, ProgramReconstructor, ProgramScope, Variant};
use leo_errors::TypeCheckerError;

impl ProgramReconstructor for ProcessingScriptVisitor<'_> {
    fn reconstruct_program_scope(&mut self, mut input: ProgramScope) -> ProgramScope {
        self.program_name = input.program_id.name.name;

        input.functions = input.functions.into_iter().map(|(i, f)| (i, self.reconstruct_function(f))).collect();
        if !self.state.is_test {
            for (_i, f) in input.functions.iter() {
                if matches!(f.variant, Variant::Script) {
                    self.state.handler.emit_err(TypeCheckerError::script_in_non_test(f.identifier, f.span))
                }
            }
        }
        input.functions.retain(|(_i, f)| !matches!(f.variant, Variant::Script));
        input
    }

    fn reconstruct_function(&mut self, mut input: Function) -> Function {
        self.current_variant = input.variant;
        input.block = self.reconstruct_block(input.block).0;
        input
    }
}
