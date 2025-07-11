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

use super::TypeCheckingVisitor;
use leo_ast::{ArrayType, CompositeType, ExpressionVisitor, TypeVisitor};
use leo_errors::TypeCheckerError;

impl TypeVisitor for TypeCheckingVisitor<'_> {
    fn visit_array_type(&mut self, input: &ArrayType) {
        self.visit_type(&input.element_type);
        self.visit_expression_infer_default_u32(&input.length);
    }

    fn visit_composite_type(&mut self, input: &CompositeType) {
        let struct_ = self.lookup_struct(self.scope_state.program_name, input.id.name).clone();
        if let Some(struct_) = struct_ {
            // Check the number of const arguments against the number of the struct's const parameters
            if struct_.const_parameters.len() != input.const_arguments.len() {
                self.emit_err(TypeCheckerError::incorrect_num_const_args(
                    "Struct type",
                    struct_.const_parameters.len(),
                    input.const_arguments.len(),
                    input.id.span,
                ));
            }

            // Check the types of const arguments against the types of the struct's const parameters
            for (expected, argument) in struct_.const_parameters.iter().zip(input.const_arguments.iter()) {
                self.visit_expression(argument, &Some(expected.type_().clone()));
            }
        } else if !input.const_arguments.is_empty() {
            self.emit_err(TypeCheckerError::unexpected_const_args(input, input.id.span));
        }
    }
}
