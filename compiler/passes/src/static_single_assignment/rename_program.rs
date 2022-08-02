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
    Expression, ExpressionKind, Function, ProgramReconstructor, ReturnStatement, Statement, StatementReconstructor,
    TernaryExpression, TupleExpression,
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

        let mk_ternary = |cond, then, els| {
            let span = Default::default();
            let kind = ExpressionKind::Ternary(TernaryExpression {
                condition: Box::new(cond),
                if_true: Box::new(then),
                if_false: Box::new(els),
            });
            Expression { kind, span }
        };

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
                    (
                        Expression {
                            kind: ExpressionKind::Tuple(expr_tuple),
                            ..
                        },
                        Expression {
                            kind: ExpressionKind::Tuple(acc_tuple),
                            ..
                        },
                    ) => {
                        let span = Default::default();
                        let elements = expr_tuple
                            .elements
                            .into_iter()
                            .zip_eq(acc_tuple.elements.into_iter())
                            .map(|(if_true, if_false)| mk_ternary(guard.clone(), if_true, if_false))
                            .collect();
                        let kind = ExpressionKind::Tuple(TupleExpression { elements });
                        Expression { kind, span }
                    }
                    // Otherwise, fold the return expressions into a single ternary expression.
                    // Note that `expr` and `acc` are correspond to the `if` and `else` cases of the ternary expression respectively.
                    (expr, acc) => mk_ternary(guard, expr, acc),
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
