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
use leo_ast::{ArrayType, Expression, ExpressionVisitor, IntegerType, Node, Type, TypeVisitor};

impl TypeVisitor for TypeCheckingVisitor<'_> {
    fn visit_array_type(&mut self, input: &ArrayType) {
        self.visit_type(&input.element_type);
        let mut length_type = self.visit_expression(&input.length, &None);
        if length_type == Type::Numeric {
            // Infer `U32` as the default type for array lengths.
            length_type = Type::Integer(IntegerType::U32);

            let Expression::Literal(literal) = &*input.length else {
                panic!("only literals can have Type::Numeric");
            };

            // Do not forget to ensure validity of the literal as `U32`
            if !self.check_numeric_literal(literal, &length_type) {
                length_type = Type::Err;
            }
        }

        // Keep track of the type of the length expression in type_table
        self.state.type_table.insert(input.length.id(), length_type);
    }
}
