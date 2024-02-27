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

use crate::{ConditionalTreeNode, TreeNode};
use indexmap::{IndexMap, IndexSet};
use leo_ast::{Identifier, Type};
use leo_errors::TypeCheckerError;
use leo_span::{Span, Symbol};

// TODO: Could optimize by removing duplicate paths (if set of futures is the same).
pub struct AwaitChecker<'a> {
    /// All possible subsets of futures that must be awaited.
    pub(crate) to_await: &'a mut Vec<ConditionalTreeNode>,
    /// Statically updated set of futures to await.
    pub(crate) static_to_await: IndexSet<Identifier>,
    /// Whether or not to do full tree search for await checking.
    pub(crate) enabled: bool,
    /// Maximum nesting depth to search for await checking.
    pub(crate) max_depth: usize,
}

impl AwaitChecker {
    /// Initializes a new `AwaitChecker`.
    pub fn new(max_depth: usize, enabled: bool) -> Self {
        Self {
            to_await: &mut Vec::new(),
            static_to_await: IndexSet::new(),
            enabled,
            max_depth,
        }
    }
    
    /// Initialize futures.
    pub fn set_futures(&mut self, futures: IndexSet<Identifier>) {
        (self.to_await, self.static_to_await) = (&mut vec![TreeNode::new(
            futures.clone(),
        )], futures);
    }

    /// Enter scope for `then` branch of conditional.
    pub fn create_then_scope(
        &mut self,
        is_finalize: bool,
        input: Span,
    ) -> Result<&mut Vec<ConditionalTreeNode>, TypeCheckerError> {
        if is_finalize && self.enabled {
            let mut current_nodes = Vec::new();
            // Extend all paths by one node to represent the upcoming `then` branch.
            for node in self.to_await.iter() {
                // Error if exceed maximum depth.
                if node.depth > self.max_depth {
                    return Err(TypeCheckerError::max_conditional_block_depth_exceeded(self.max_depth, input));
                }
                // Extend current path.
                current_nodes.push(node.clone().create_child());
            }
            // Update the set of nodes to be current set.
            self.to_await = &mut current_nodes.clone();
            Ok(&mut current_nodes)
        } else {
            Ok(&mut Vec::new())
        }
    }

    /// Exit scope for `then` branch of conditional.
    pub fn exit_then_scope(&mut self, is_finalize: bool, parent_nodes: &mut Vec<ConditionalTreeNode>) -> &mut Vec<ConditionalTreeNode> {
        // Check if a nested conditional statement signaled their existence. 
        if is_finalize && self.enabled {
            let saved = self.to_await;
            self.to_await = parent_nodes;
            saved
        } else {
            &mut Vec::new()
        }
    }

    /// Exit scope for conditional statement at current depth.
    pub fn exit_statement_scope(&mut self, is_finalize: bool, then_nodes: &mut Vec<ConditionalTreeNode>) {
        if is_finalize && self.enabled {
            // Merge together the current set of nodes (from `otherwise` branch) with `then` nodes.
            self.to_await = [then_nodes, self.to_await].concat()
        }
    }
}
