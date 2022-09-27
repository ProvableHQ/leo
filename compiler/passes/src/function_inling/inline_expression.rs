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

use crate::{CallType, Inliner};

use leo_ast::{CallExpression, Expression, ExpressionReconstructor, Statement};

impl ExpressionReconstructor for Inliner<'_> {
    type AdditionalOutput = Vec<Statement>;

    fn reconstruct_call(&mut self, input: CallExpression) -> (Expression, Self::AdditionalOutput) {
        // Check if we are inlining the function.
        let function = match *input.function {
            Expression::Identifier(function) => function,
            _ => unreachable!("A call expression's function must be an identifier."),
        };
        // Note that this unwrap is safe since type checking guarantees that the function exists.
        let inline = self.symbol_table.lookup_fn_symbol(function.name).unwrap().call_type == CallType::Inlined;

        // Inline the function if `self.inlining` is true, otherwise, return the original call expression.
        let expression = match inline {
            false => input,
            true => {
                // TODO
                input
            }
        };

        (Expression::Call(expression), Default::default())
    }
}
