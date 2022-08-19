// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::Inliner;

use leo_ast::{CallExpression, Expression, ExpressionReconstructor};

impl ExpressionReconstructor for Inliner<'_> {
    type AdditionalOutput = ();

    fn reconstruct_call(&mut self, input: CallExpression) -> (Expression, Self::AdditionalOutput) {
        // Reconstruct the function expression.
        let function = self.reconstruct_expression(*input.function).0;

        (
            Expression::Call(CallExpression {
                function: Box::new(function),
                arguments: input
                    .arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg).0)
                    .collect(),
                span: input.span,
            }),
            Default::default(),
        )
    }
}
