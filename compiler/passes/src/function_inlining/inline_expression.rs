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

use crate::{FunctionInliner, Replacer};

use leo_ast::{
    CallExpression,
    Expression,
    ExpressionReconstructor,
    Identifier,
    ReturnStatement,
    Statement,
    StatementReconstructor,
    Type,
    UnitExpression,
    Variant,
};

use indexmap::IndexMap;
use itertools::Itertools;

impl ExpressionReconstructor for FunctionInliner<'_> {
    type AdditionalOutput = Vec<Statement>;

    fn reconstruct_call(&mut self, input: CallExpression) -> (Expression, Self::AdditionalOutput) {
        // Type checking guarantees that only functions local to the program scope can be inlined.
        if input.program.unwrap() != self.program.unwrap() {
            return (Expression::Call(input), Default::default());
        }

        // Get the name of the callee function.
        let function_name = match *input.function {
            Expression::Identifier(identifier) => identifier.name,
            _ => unreachable!("Parser guarantees that `input.function` is always an identifier."),
        };

        // Lookup the reconstructed callee function.
        // Since this pass processes functions in post-order, the callee function is guaranteed to exist in `self.reconstructed_functions`
        let (_, callee) = self.reconstructed_functions.iter().find(|(symbol, _)| *symbol == function_name).unwrap();

        // Inline the callee function, if required, otherwise, return the call expression.
        match callee.variant {
            Variant::Inline => {
                // Construct a mapping from input variables of the callee function to arguments passed to the callee.
                let parameter_to_argument = callee
                    .input
                    .iter()
                    .map(|input| input.identifier().name)
                    .zip_eq(input.arguments)
                    .collect::<IndexMap<_, _>>();

                // Replace each input variable with the appropriate parameter.
                let replace = |identifier: &Identifier| {
                    parameter_to_argument.get(&identifier.name).cloned().unwrap_or(Expression::Identifier(*identifier))
                };

                let mut inlined_statements =
                    Replacer::new(replace).reconstruct_block(callee.block.clone()).0.statements;

                // If the inlined block returns a value, then use the value in place of the call expression; otherwise, use the unit expression.
                let result = match inlined_statements.last() {
                    Some(Statement::Return(_)) => {
                        // Note that this unwrap is safe since we know that the last statement is a return statement.
                        match inlined_statements.pop().unwrap() {
                            Statement::Return(ReturnStatement { expression, .. }) => expression,
                            _ => panic!("This branch checks that the last statement is a return statement."),
                        }
                    }
                    _ => {
                        let id = self.node_builder.next_id();
                        self.type_table.insert(id, Type::Unit);
                        Expression::Unit(UnitExpression { span: Default::default(), id })
                    }
                };

                (result, inlined_statements)
            }
            Variant::Function | Variant::AsyncFunction | Variant::Transition | Variant::AsyncTransition => {
                (Expression::Call(input), Default::default())
            }
        }
    }
}
