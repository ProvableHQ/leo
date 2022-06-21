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

use leo_ast::*;

use crate::{Declaration, Flattener};

impl<'a> ExpressionReconstructor for Flattener<'a> {
    fn reconstruct_identifier(&mut self, input: Identifier) -> Expression {
        // let x: u32 = 10u32;
        // let y: u32 = x;

        // return y == 10u32;

        /*
        function main(b: bool) {
            let x = 0;
            if b {
                x = 1;
            } else {
                x = 2;
            }
            x == 1
        }

        function main() {
            let x = 0;
            if true {
                x = 1;
            } else {
                x = 2;
            }
            x == 1
        }
        */
        if let Some(var_value) = self.var_references.get(&input.name) {
            Expression::Literal(var_value.clone())
        } else {
            match &self
                .symbol_table
                .borrow()
                .lookup_variable(input.name)
                .unwrap()
                .declaration
            {
                Declaration::Const(Some(c)) | Declaration::Mut(Some(c)) => Expression::Literal(c.clone().into()),
                _ => Expression::Identifier(input),
            }
        }
    }

    fn reconstruct_call(&mut self, input: CallExpression) -> Expression {
        Expression::Call(CallExpression {
            function: input.function,
            arguments: input
                .arguments
                .into_iter()
                .map(|arg| self.reconstruct_expression(arg))
                .collect(),
            span: input.span,
        })
    }
}
