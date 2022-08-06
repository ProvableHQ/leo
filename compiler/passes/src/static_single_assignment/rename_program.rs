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

use crate::StaticSingleAssigner;
use itertools::Itertools;

use leo_ast::{
    Expression, Function, ProgramReconstructor, ReturnStatement, Statement, StatementReconstructor, TernaryExpression,
    TupleExpression,
};

impl ProgramReconstructor for StaticSingleAssigner<'_> {
    /// Reconstructs the `Function`s in the `Program`, while allocating the appropriate `RenameTable`s.
    fn reconstruct_function(&mut self, function: Function) -> Function {
        // Allocate a `RenameTable` for the function.
        self.push();

        // There is no need to reconstruct `function.inputs`.
        // However, for each input, we must add each symbol to the rename table.
        for input_variable in function.input.iter() {
            self.rename_table
                .update(input_variable.identifier.name, input_variable.identifier.name);
        }

        let mut block = self.reconstruct_block(function.block);

        // Add the `ReturnStatement` to the end of the block.
        let mut returns = self.clear_early_returns();

        // Type checking guarantees that there exists at least one return statement in the function body.
        let (_, last_return_expression) = returns.pop().unwrap();

        // Fold all return expressions into a single ternary expression.
        let expression = returns
            .into_iter()
            .rev()
            .fold(last_return_expression, |acc, (guard, expr)| match guard {
                None => unreachable!("All return statements except for the last one must have a guard."),
                // Note that type checking guarantees that all expressions in return statements in the function body have the same type.
                Some(guard) => match (expr, acc) {
                    // If the function returns tuples, fold the return expressions into a tuple of ternary expressions.
                    // Note that `expr` and `acc` are correspond to the `if` and `else` cases of the ternary expression respectively.
                    (Expression::Tuple(expr_tuple), Expression::Tuple(acc_tuple)) => {
                        Expression::Tuple(TupleExpression {
                            elements: expr_tuple
                                .elements
                                .into_iter()
                                .zip_eq(acc_tuple.elements.into_iter())
                                .map(|(if_true, if_false)| {
                                    Expression::Ternary(TernaryExpression {
                                        condition: Box::new(guard.clone()),
                                        if_true: Box::new(if_true),
                                        if_false: Box::new(if_false),
                                        span: Default::default(),
                                    })
                                })
                                .collect(),
                            span: Default::default(),
                        })
                    }
                    // Otherwise, fold the return expressions into a single ternary expression.
                    // Note that `expr` and `acc` are correspond to the `if` and `else` cases of the ternary expression respectively.
                    (expr, acc) => Expression::Ternary(TernaryExpression {
                        condition: Box::new(guard),
                        if_true: Box::new(expr),
                        if_false: Box::new(acc),
                        span: Default::default(),
                    }),
                },
            });

        // Add the `ReturnStatement` to the end of the block.
        block.statements.push(Statement::Return(ReturnStatement {
            expression,
            span: Default::default(),
        }));

        // Remove the `RenameTable` for the function.
        self.pop();

        Function {
            annotations: function.annotations,
            identifier: function.identifier,
            input: function.input,
            output: function.output,
            core_mapping: function.core_mapping,
            block,
            span: function.span,
        }
    }
}
