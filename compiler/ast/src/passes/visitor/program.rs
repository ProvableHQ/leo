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

/// A Visitor trait for the program represented by the AST.
pub trait ProgramVisitor<'a>: StatementVisitor<'a> {
    fn visit_program(&mut self, input: &'a Program) {
        input.imports.values().for_each(|import| self.visit_import(&import.0));

        input
            .program_scopes
            .values()
            .for_each(|scope| self.visit_program_scope(scope));
    }

    fn visit_program_scope(&mut self, input: &'a ProgramScope) {
        input.structs.values().for_each(|function| self.visit_struct(function));

        input.mappings.values().for_each(|mapping| self.visit_mapping(mapping));

        input
            .functions
            .values()
            .for_each(|function| self.visit_function(function));
    }

    fn visit_import(&mut self, input: &'a Program) {
        self.visit_program(input)
    }

    fn visit_struct(&mut self, _input: &'a Struct) {}

    fn visit_mapping(&mut self, _input: &'a Mapping) {}

    fn visit_function(&mut self, input: &'a Function) {
        self.visit_block(&input.block);
        if let Some(finalize) = &input.finalize {
            self.visit_block(&finalize.block);
        }
    }
}
