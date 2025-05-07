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

use crate::ConditionalTreeNode;
use indexmap::IndexSet;
use leo_ast::Identifier;
use leo_errors::StaticAnalyzerWarning;
use leo_span::{Span, Symbol};

// TODO: Could optimize by removing duplicate paths (if set of futures is the same).
pub struct AwaitChecker {
    /// All possible subsets of futures that must be awaited.
    pub(crate) to_await: Vec<ConditionalTreeNode>,
    /// Statically updated set of futures to await.
    pub(crate) static_to_await: IndexSet<Symbol>,
}

impl AwaitChecker {
    /// Initializes a new `AwaitChecker`.
    pub fn new() -> Self {
        Self { to_await: Vec::new(), static_to_await: IndexSet::new() }
    }

    /// Remove from list.
    /// Returns `true` if there was a path where the future was not awaited in the order of the input list.
    pub fn remove(&mut self, id: &Identifier) -> bool {
        // Can assume in finalize block.
        // Remove from dynamic list.
        let is_not_first = self.to_await.iter_mut().any(|node| node.remove_element(&id.name));

        // Remove from static list.
        self.static_to_await.shift_remove(&id.name);

        is_not_first
    }

    /// Initialize futures.
    pub fn set_futures(&mut self, futures: IndexSet<Symbol>) {
        if futures.is_empty() {
            self.to_await = Vec::new();
        } else {
            self.to_await = vec![ConditionalTreeNode::new(futures.clone())];
        }
        self.static_to_await = futures;
    }

    /// Enter scope for `then` branch of conditional.
    pub fn create_then_scope(
        &mut self,
        is_finalize: bool,
        _input: Span,
    ) -> Result<Vec<ConditionalTreeNode>, StaticAnalyzerWarning> {
        if is_finalize {
            let mut current_nodes = Vec::new();
            // Extend all paths by one node to represent the upcoming `then` branch.
            for node in self.to_await.iter() {
                // Extend current path.
                current_nodes.push(node.clone().create_child());
            }
            // Update the set of nodes to be current set.
            self.to_await = current_nodes.clone();
            Ok(current_nodes)
        } else {
            Ok(Vec::new())
        }
    }

    /// Exit scope for `then` branch of conditional.
    pub fn exit_then_scope(
        &mut self,
        is_finalize: bool,
        parent_nodes: Vec<ConditionalTreeNode>,
    ) -> Vec<ConditionalTreeNode> {
        // Check if a nested conditional statement signaled their existence.
        if is_finalize { core::mem::replace(&mut self.to_await, parent_nodes) } else { Vec::new() }
    }

    /// Exit scope for conditional statement at current depth.
    pub fn exit_statement_scope(&mut self, is_finalize: bool, then_nodes: Vec<ConditionalTreeNode>) {
        if is_finalize {
            // Merge together the current set of nodes (from `otherwise` branch) with `then` nodes.
            self.to_await.extend(then_nodes);
        }
    }
}
