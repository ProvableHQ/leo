// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use super::*;
use crate::{ConditionalTreeNode, TypeChecker, VariableSymbol, VariableType};

use leo_ast::{
    Type::{Future, Tuple},
    *,
};
use leo_errors::StaticAnalyzerError;

use itertools::Itertools;

impl<'a, N: Network> StatementVisitor<'a> for StaticAnalyzer<'a, N> {
    fn visit_block(&mut self, input: &'a Block) {
        // Enter the block scope.
        let scope_index = self.current_scope_index();
        let previous_scope_index = self.enter_scope(scope_index);

        input.statements.iter().for_each(|stmt| self.visit_statement(stmt));

        // Exit the block scope.
        self.exit_scope(previous_scope_index);
    }

    fn visit_conditional(&mut self, input: &'a ConditionalStatement) {
        self.visit_expression(&input.condition, &Default::default());

        // Create scope for checking awaits in `then` branch of conditional.
        let current_bst_nodes: Vec<ConditionalTreeNode> =
            match self.await_checker.create_then_scope(self.variant == Some(Variant::AsyncFunction), input.span) {
                Ok(nodes) => nodes,
                Err(warn) => return self.emit_warning(warn),
            };

        // Visit block.
        self.visit_block(&input.then);

        // Exit scope for checking awaits in `then` branch of conditional.
        let saved_paths =
            self.await_checker.exit_then_scope(self.variant == Some(Variant::AsyncFunction), current_bst_nodes);

        if let Some(otherwise) = &input.otherwise {
            match &**otherwise {
                Statement::Block(stmt) => {
                    // Visit the otherwise-block.
                    self.visit_block(stmt);
                }
                Statement::Conditional(stmt) => self.visit_conditional(stmt),
                _ => unreachable!("Else-case can only be a block or conditional statement."),
            }
        }

        // Update the set of all possible BST paths.
        self.await_checker.exit_statement_scope(self.variant == Some(Variant::AsyncFunction), saved_paths);
    }
}
