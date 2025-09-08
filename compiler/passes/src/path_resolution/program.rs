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

use super::PathResolutionVisitor;
use leo_ast::{AstReconstructor, Module, ProgramReconstructor, Statement};

impl ProgramReconstructor for PathResolutionVisitor<'_> {
    fn reconstruct_module(&mut self, input: Module) -> Module {
        self.in_module_scope(&input.path.clone(), |slf| Module {
            program_name: input.program_name,
            path: input.path,
            structs: input.structs.into_iter().map(|(i, c)| (i, slf.reconstruct_struct(c))).collect(),
            functions: input.functions.into_iter().map(|(i, f)| (i, slf.reconstruct_function(f))).collect(),
            consts: input
                .consts
                .into_iter()
                .map(|(i, c)| match slf.reconstruct_const(c) {
                    (Statement::Const(declaration), _) => (i, declaration),
                    _ => panic!("`reconstruct_const` can only return `Statement::Const`"),
                })
                .collect(),
        })
    }
}
