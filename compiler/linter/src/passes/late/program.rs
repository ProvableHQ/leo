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

use leo_ast::{AstVisitor as _, Constructor, Function, ProgramVisitor};

use super::visitor::LateLintingVisitor;

impl ProgramVisitor for LateLintingVisitor<'_> {
    fn visit_function(&mut self, input: &Function) {
        for lint in &mut self.lints {
            lint.check_function(input);
        }

        input.const_parameters.iter().for_each(|input| self.visit_type(&input.type_));
        input.input.iter().for_each(|input| self.visit_type(&input.type_));
        input.output.iter().for_each(|output| self.visit_type(&output.type_));
        self.visit_type(&input.output_type);
        self.visit_block(&input.block);
    }

    fn visit_constructor(&mut self, input: &Constructor) {
        for lint in &mut self.lints {
            lint.check_constructor(input);
        }

        self.visit_block(&input.block);
    }
}
