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
    Block, CircuitExpression, CircuitVariableInitializer, Expression, Function, FunctionConsumer, Identifier, Program,
    ProgramConsumer, ReturnStatement, Statement, StatementConsumer, TernaryExpression, TupleExpression,
};

impl FunctionConsumer for StaticSingleAssigner<'_> {
    type Output = Function;

    /// Reconstructs the `Function`s in the `Program`, while allocating the appropriate `RenameTable`s.
    fn consume_function(&mut self, function: Function) -> Self::Output {
        // Allocate a `RenameTable` for the function.
        self.push();

        // There is no need to reconstruct `function.inputs`.
        // However, for each input, we must add each symbol to the rename table.
        for input_variable in function.input.iter() {
            self.rename_table
                .update(input_variable.identifier.name, input_variable.identifier.name);
        }

        let mut statements = self.consume_block(function.block);

        // Add the `ReturnStatement` to the end of the block.
        let mut returns = self.clear_early_returns();

        // Type checking guarantees that there exists at least one return statement in the function body.
        let (_, last_return_expression) = returns.pop().unwrap();

        // Produce a chain of ternary expressions and assignments for the set of early returns.
        let mut stmts = Vec::with_capacity(returns.len());

        // Helper to construct and store ternary assignments. e.g `$ret$0 = $var$0 ? $var$1 : $var$2`
        let mut construct_ternary_assignment = |guard: Expression, if_true: Expression, if_false: Expression| {
            let place = Expression::Identifier(Identifier {
                name: self.unique_symbol("$ret"),
                span: Default::default(),
            });
            stmts.push(Self::simple_assign_statement(
                place.clone(),
                Expression::Ternary(TernaryExpression {
                    condition: Box::new(guard),
                    if_true: Box::new(if_true),
                    if_false: Box::new(if_false),
                    span: Default::default(),
                }),
            ));
            place
        };

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
                                    construct_ternary_assignment(guard.clone(), if_true, if_false)
                                })
                                .collect(),
                            span: Default::default(),
                        })
                    }
                    // If the function returns circuits, fold the return expressions into a circuit of ternary expressions.
                    // Note that `expr` and `acc` are correspond to the `if` and `else` cases of the ternary expression respectively.
                    (Expression::Circuit(expr_circuit), Expression::Circuit(acc_circuit)) => {
                        Expression::Circuit(CircuitExpression {
                            name: acc_circuit.name,
                            span: acc_circuit.span,
                            members: expr_circuit
                                .members
                                .into_iter()
                                .zip_eq(acc_circuit.members.into_iter())
                                .map(|(if_true, if_false)| {
                                    let expression = construct_ternary_assignment(
                                        guard.clone(),
                                        match if_true.expression {
                                            None => Expression::Identifier(if_true.identifier),
                                            Some(expr) => expr,
                                        },
                                        match if_false.expression {
                                            None => Expression::Identifier(if_false.identifier),
                                            Some(expr) => expr,
                                        },
                                    );
                                    CircuitVariableInitializer {
                                        identifier: if_true.identifier,
                                        expression: Some(expression),
                                    }
                                })
                                .collect(),
                        })
                    }
                    // Otherwise, fold the return expressions into a single ternary expression.
                    // Note that `expr` and `acc` are correspond to the `if` and `else` cases of the ternary expression respectively.
                    (expr, acc) => construct_ternary_assignment(guard, expr, acc),
                },
            });

        // Add all of the accumulated statements to the end of the block.
        statements.extend(stmts);

        // Add the `ReturnStatement` to the end of the block.
        statements.push(Statement::Return(ReturnStatement {
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
            block: Block {
                span: Default::default(),
                statements,
            },
            span: function.span,
        }
    }
}

impl ProgramConsumer for StaticSingleAssigner<'_> {
    type Output = Program;

    fn consume_program(&mut self, input: Program) -> Self::Output {
        Program {
            name: input.name,
            network: input.network,
            expected_input: input.expected_input,
            // TODO: Do inputs need to be processed? They are not processed in the existing compiler.
            imports: input.imports,
            functions: input
                .functions
                .into_iter()
                .map(|(i, f)| (i, self.consume_function(f)))
                .collect(),
            circuits: input.circuits,
        }
    }
}
