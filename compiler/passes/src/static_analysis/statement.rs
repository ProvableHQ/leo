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

use super::StaticAnalyzingVisitor;
use crate::ConditionalTreeNode;

use leo_ast::*;

impl StatementVisitor for StaticAnalyzingVisitor<'_> {
    fn visit_conditional(&mut self, input: &ConditionalStatement) {
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
