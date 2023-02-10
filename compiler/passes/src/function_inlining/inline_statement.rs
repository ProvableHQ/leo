// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::FunctionInliner;

use leo_ast::{Block, ConsoleStatement, DefinitionStatement, IterationStatement, Statement, StatementReconstructor};

impl StatementReconstructor for FunctionInliner<'_> {
    /// Reconstructs the statements inside a basic block, accumulating any statements produced by function inlining.
    fn reconstruct_block(&mut self, block: Block) -> (Block, Self::AdditionalOutput) {
        let mut statements = Vec::with_capacity(block.statements.len());

        for statement in block.statements {
            let (reconstructed_statement, additional_statements) = self.reconstruct_statement(statement);
            statements.extend(additional_statements);
            statements.push(reconstructed_statement);
        }

        (
            Block {
                span: block.span,
                statements,
            },
            Default::default(),
        )
    }

    /// Parsing guarantees that console statements are not present in the program.
    fn reconstruct_console(&mut self, _: ConsoleStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`ConsoleStatement`s should not be in the AST at this phase of compilation.")
    }

    /// Static single assignment replaces definition statements with assignment statements.
    fn reconstruct_definition(&mut self, _definition: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`DefinitionStatement`s should not exist in the AST at this phase of compilation.")
    }

    /// Loop unrolling unrolls and removes iteration statements from the program.
    fn reconstruct_iteration(&mut self, _input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }
}
