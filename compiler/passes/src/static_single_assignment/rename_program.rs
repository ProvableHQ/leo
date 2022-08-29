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
    Block, FinalizeStatement, Function, FunctionConsumer, Program, ProgramConsumer, ReturnStatement, Statement,
    StatementConsumer,
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

        // Get all of the guards and return expression.
        let returns = self.clear_early_returns();

        // If the function contains return statements, then we fold them into a single return statement.
        if !returns.is_empty() {
            let (stmts, expression) = self.fold_guards("ret$", returns);

            // Add all of the accumulated statements to the end of the block.
            statements.extend(stmts);

            // Add the `ReturnStatement` to the end of the block.
            statements.push(Statement::Return(ReturnStatement {
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
            statements.extend(stmts);

            // Add the `FinalizeStatement` to the end of the block.
            statements.push(Statement::Finalize(FinalizeStatement {
                expression,
                span: Default::default(),
            }));
        }

        // Remove the `RenameTable` for the function.
        self.pop();

        Function {
            annotations: function.annotations,
            identifier: function.identifier,
            input: function.input,
            output: function.output,
            block: Block {
                span: Default::default(),
                statements,
            },
            finalize: function.finalize,
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
            mappings: input.mappings,
        }
    }
}
