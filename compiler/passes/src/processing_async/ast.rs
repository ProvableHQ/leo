// Copyright (C) 2019-2026 Provable Inc.
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

use super::ProcessingAsyncVisitor;
use crate::BlockToFunctionRewriter;
use leo_ast::{AstReconstructor, AsyncExpression, Block, Expression, IterationStatement, Node, Statement, Variant};

impl AstReconstructor for ProcessingAsyncVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    /// Transforms an `AsyncExpression` into a standalone async `Function` and returns
    /// a call to this function. This process:
    fn reconstruct_async(&mut self, input: AsyncExpression, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        // Generate a unique name for the async function.
        let finalize_fn_name = self.state.assigner.unique_symbol(self.current_function, "$");

        // Convert the block into a function and a function call.
        let mut block_to_function_rewriter = BlockToFunctionRewriter::new(self.state, self.current_program);
        let (function, call_to_finalize) =
            block_to_function_rewriter.rewrite_block(&input.block, finalize_fn_name, Variant::AsyncFunction);

        // Ensure we're not trying to capture too many variables.
        if function.input.len() > self.max_inputs {
            self.state.handler.emit_err(leo_errors::StaticAnalyzerError::async_block_capturing_too_many_vars(
                function.input.len(),
                self.max_inputs,
                input.span,
            ));
        }

        // Register the generated function
        self.new_async_functions.push((finalize_fn_name, function));

        self.modified = true;

        (call_to_finalize, ())
    }

    fn reconstruct_block(&mut self, input: Block) -> (Block, Self::AdditionalOutput) {
        self.in_scope(input.id(), |slf| {
            (
                Block {
                    statements: input.statements.into_iter().map(|s| slf.reconstruct_statement(s).0).collect(),
                    span: input.span,
                    id: input.id,
                },
                Default::default(),
            )
        })
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        self.in_scope(input.id(), |slf| {
            (
                IterationStatement {
                    type_: input.type_.map(|ty| slf.reconstruct_type(ty).0),
                    start: slf.reconstruct_expression(input.start, &()).0,
                    stop: slf.reconstruct_expression(input.stop, &()).0,
                    block: slf.reconstruct_block(input.block).0,
                    ..input
                }
                .into(),
                Default::default(),
            )
        })
    }
}
