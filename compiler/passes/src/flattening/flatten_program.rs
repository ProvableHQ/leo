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

use crate::{Flattener};

use leo_ast::{Finalize, FinalizeStatement, Function, ProgramReconstructor, ReturnStatement, Statement, StatementReconstructor};

// TODO: Document.

impl ProgramReconstructor for Flattener<'_> {
    fn reconstruct_function(&mut self, function: Function) -> Function {
        let mut block = self.reconstruct_block(function.block).0;

        // Get all of the guards and return expression.
        let returns = self.clear_early_returns();

        // If the function contains return statements, then we fold them into a single return statement.
        if !returns.is_empty() {
            let (stmts, expression) = self.fold_guards("ret$", returns);

            // Add all of the accumulated statements to the end of the block.
            block.statements.extend(stmts);

            // Add the `ReturnStatement` to the end of the block.
            block.statements.push(Statement::Return(ReturnStatement {
                expression,
                span: Default::default(),
            }));
        }

        // Get all of the guards and finalize expression.
        let finalizes = self.clear_early_finalizes();

        // If the function contains finalize statements, then we fold them into a single finalize statement.
        if !finalizes.is_empty() {
            let (stmts, expression) = self.fold_guards("fin$", finalizes);

            // Add all of the accumulated statements to the end of the block.
            block.statements.extend(stmts);

            // Add the `FinalizeStatement` to the end of the block.
            block.statements.push(Statement::Finalize(FinalizeStatement {
                expression,
                span: Default::default(),
            }));
        }

        Function {
            annotations: function.annotations,
            identifier: function.identifier,
            input: function.input,
            output: function.output,
            output_type: function.output_type,
            block,
            finalize: function.finalize.map(|finalize| {
                let mut block = self.reconstruct_block(finalize.block).0;

                // Get all of the guards and return expression.
                let returns = self.clear_early_returns();

                // If the function contains return statements, then we fold them into a single return statement.
                if !returns.is_empty() {
                    let (stmts, expression) = self.fold_guards("ret$", returns);

                    // Add all of the accumulated statements to the end of the block.
                    block.statements.extend(stmts);

                    // Add the `ReturnStatement` to the end of the block.
                    block.statements.push(Statement::Return(ReturnStatement {
                        expression,
                        span: Default::default(),
                    }));
                }

                Finalize {
                    input: finalize.input,
                    output: finalize.output,
                    output_type: finalize.output_type,
                    block,
                    span: finalize.span
                }
            }),
            span: function.span,
        }
    }
}
