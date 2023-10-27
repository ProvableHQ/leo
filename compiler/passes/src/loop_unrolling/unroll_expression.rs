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

use leo_ast::*;
use leo_errors::LoopUnrollerError;

use crate::Unroller;

impl ExpressionReconstructor for Unroller<'_> {
    type AdditionalOutput = bool;

    fn reconstruct_array_access(&mut self, input: ArrayAccess) -> (Expression, Self::AdditionalOutput) {
        // Reconstruct the index.
        let index = self.reconstruct_expression(*input.index).0;
        // If the index is not a literal, then emit an error.
        if !matches!(index, Expression::Literal(_)) {
            self.emit_err(LoopUnrollerError::variable_array_access(input.span));
        }

        (
            Expression::Access(AccessExpression::Array(ArrayAccess {
                array: Box::new(self.reconstruct_expression(*input.array).0),
                index: Box::new(index),
                span: input.span,
                id: input.id,
            })),
            Default::default(),
        )
    }

    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Self::AdditionalOutput) {
        // Substitute the identifier with the constant value if it is a constant.
        if let Some(expr) = self.constant_propagation_table.borrow().lookup_constant(input.name) {
            return (expr.clone(), Default::default());
        }
        (Expression::Identifier(input), Default::default())
    }
}
