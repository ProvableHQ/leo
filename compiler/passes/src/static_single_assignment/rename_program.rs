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

use leo_ast::{
    Expression, Function, FunctionInput, ProgramReconstructor, ReturnStatement, Statement, StatementReconstructor,
    TernaryExpression,
};

impl ProgramReconstructor for StaticSingleAssigner<'_> {
    /// Reconstructs the `Function`s in the `Program`, while allocating the appropriate `RenameTable`s.
    fn reconstruct_function(&mut self, function: Function) -> Function {
        // Allocate a `RenameTable` for the function.
        self.push();

        // There is no need to reconstruct `function.inputs`.
        // However, for each input, we must add each symbol to the rename table.
        for input in function.input.iter() {
            match input {
                FunctionInput::Variable(function_input_variable) => {
                    self.rename_table.update(
                        function_input_variable.identifier.name,
                        function_input_variable.identifier.name,
                    );
                }
            }
        }

        let mut block = self.reconstruct_block(function.block);

        // Add the `ReturnStatement` to the end of the block.
        let mut returns = self.clear_early_returns();

        // Type checking guarantees that there exists at least one return statement in the function body.
        let (_, last_return_expression) = returns.pop().unwrap();

        // Fold all return expressions into a single ternary expression.
        let expression =
            returns
                .into_iter()
                .rev()
                .fold(last_return_expression, |acc, (guard, expression)| match guard {
                    None => unreachable!("All return statements except for the last one must have a guard."),
                    Some(guard) => Expression::Ternary(TernaryExpression {
                        condition: Box::new(guard),
                        if_true: Box::new(expression),
                        if_false: Box::new(acc),
                        span: Default::default(),
                    }),
                });

        // Add the `ReturnStatement` to the end of the block.
        block.statements.push(Statement::Return(ReturnStatement {
            expression,
            span: Default::default(),
        }));

        // Remove the `RenameTable` for the function.
        self.pop();

        Function {
            identifier: function.identifier,
            input: function.input,
            output: function.output,
            core_mapping: function.core_mapping,
            block,
            span: function.span,
        }
    }
}
