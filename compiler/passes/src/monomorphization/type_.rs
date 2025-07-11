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

use super::MonomorphizationVisitor;

use leo_ast::{CompositeType, Expression, Identifier, Type, TypeReconstructor};

impl TypeReconstructor for MonomorphizationVisitor<'_> {
    fn reconstruct_composite_type(&mut self, input: leo_ast::CompositeType) -> (leo_ast::Type, Self::AdditionalOutput) {
        // Proceed only if there are some const arguments.
        if input.const_arguments.is_empty() {
            return (Type::Composite(input), Default::default());
        }

        // Ensure all const arguments are literals; if not, we skip this struct type instantiation for now and mark it
        // as unresolved.
        //
        // The types of the const arguments are checked in the type checking pass.
        if input.const_arguments.iter().any(|arg| !matches!(arg, Expression::Literal(_))) {
            self.unresolved_struct_types.push(input.clone());
            return (Type::Composite(input), Default::default());
        }

        // At this stage, we know that we're going to modify the program
        self.changed = true;
        (
            Type::Composite(CompositeType {
                id: Identifier {
                    name: self.monomorphize_struct(&input.id.name, &input.const_arguments), // use the new name
                    span: input.id.span,
                    id: self.state.node_builder.next_id(),
                },
                const_arguments: vec![], // remove const arguments
                program: input.program,
            }),
            Default::default(),
        )
    }
}
