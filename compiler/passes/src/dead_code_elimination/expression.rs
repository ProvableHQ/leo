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

use super::DeadCodeEliminatingVisitor;

use leo_ast::{Expression, ExpressionReconstructor, Identifier};

impl ExpressionReconstructor for DeadCodeEliminatingVisitor<'_> {
    type AdditionalOutput = ();

    // Use and reconstruct an identifier.
    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Self::AdditionalOutput) {
        self.used_variables.insert(input.name);
        (Expression::Identifier(input), Default::default())
    }

    // We need to make sure we hit identifiers, so do our own traversal
    // rather than relying on the default.
    fn reconstruct_struct_init(
        &mut self,
        mut input: leo_ast::StructExpression,
    ) -> (Expression, Self::AdditionalOutput) {
        for member in input.members.iter_mut() {
            if let Some(expr) = std::mem::take(&mut member.expression) {
                member.expression = Some(self.reconstruct_expression(expr).0);
            } else {
                // We're not actually going to modify it.
                self.reconstruct_identifier(member.identifier);
            }
        }

        (Expression::Struct(input), Default::default())
    }
}
