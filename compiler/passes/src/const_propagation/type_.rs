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

use super::ConstPropagationVisitor;

use leo_ast::{ExpressionReconstructor, Node, TypeReconstructor};

impl TypeReconstructor for ConstPropagationVisitor<'_> {
    fn reconstruct_array_type(&mut self, input: leo_ast::ArrayType) -> (leo_ast::Type, Self::AdditionalOutput) {
        let (length, opt_value) = self.reconstruct_expression(*input.length);

        // If we can't evaluate this array length, keep track of it for error reporting later
        if opt_value.is_none() {
            self.array_length_not_evaluated = Some(length.span());
        }

        (
            leo_ast::Type::Array(leo_ast::ArrayType {
                element_type: Box::new(self.reconstruct_type(*input.element_type).0),
                length: Box::new(length),
            }),
            Default::default(),
        )
    }
}
