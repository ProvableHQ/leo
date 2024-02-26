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

pub struct AwaitChecker {
    /// All possible subsets of futures that must be awaited.
    pub(crate) to_await: Vec<ConditionalTreeNode>,
    /// Updated set of futures to await.
    pub(crate) updated_to_await: Vec<ConditionalTreeNode>,
    /// Statically updated set of futures to await.
    pub(crate) static_to_await: IndexSet<Identifier>,
    /// Whether or not to do full tree search for await checking.
    pub(crate) enabled: bool,
    /// Maximum nesting depth to search for await checking.
    pub(crate) max_depth: usize,
    /// Whether the current BST has a root node.
    pub(crate) has_root: bool,
    /// A signal to a potential parent nested conditional that it is not a leaf node in the BST. 
    pub(crate) outer_is_parent: bool,
}

impl AwaitChecker {
    /// Initializes a new `AwaitChecker`.
    pub fn new(max_depth: usize, enabled: bool) -> Self {
        Self {
            to_await: Vec::new(),
            updated_to_await: Vec::new(),
            static_to_await: IndexSet::new(),
            enabled,
            max_depth,
            has_root: false,
            outer_is_parent: false,
        }
    }
    
    /// Initialize futures.
    pub fn set_futures(&mut self, futures: IndexSet<Identifier>) {
        (self.to_await, self.static_to_await) = (vec![TreeNode::new(
            futures.clone(),
        )], futures);
    }

    /// Enter scope for `then` branch of conditional.
    pub fn create_then_scope(
        &mut self,
        is_finalize: bool,
        input: Span,
    ) -> Result<Vec<ConditionalTreeNode>, TypeCheckerError> {
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
            // Reset the `is_parent` signal.
            self.outer_is_parent = false;
            // Update the set of nodes to be current set.
            self.to_await = current_nodes.clone();
            Ok(current_nodes)
        } else {
            Ok(Vec::new())
        }
    }

    /// Exit scope for `then` branch of conditional.
    pub fn exit_then_scope(&mut self, is_finalize: bool) {
        // Check if a nested conditional statement signaled their existence. 
        if !self.outer_is_parent && is_finalize && self.enabled {
            // In the case that the current conditional statement is not a leaf node in the tree, need to commit updated set of futures to await.
            self.updated_to_await.extend(self.to_await.clone());
        }
    }

    /// Enter scope for `otherwise` branch of conditional.
    pub fn create_otherwise_scope(&mut self, is_finalize: bool, current_bst_nodes: Vec<ConditionalTreeNode>) {
        if is_finalize && self.enabled {
            self.to_await = current_bst_nodes;
            // Reset the `is_parent` signal.
            self.outer_is_parent = false;
        }
    }

    /// Exit scope for conditional statement at current depth.
    pub fn exit_scope(&mut self, is_root: bool, is_finalize: bool) {
        if is_finalize && self.enabled {
            // Check if the current conditional statement is a leaf node.
            if !self.outer_is_parent {
                // Add to the updated set of futures to await.
                self.updated_to_await.extend(self.to_await.clone());
            }

            if is_root {
                // Update the set of all possible paths of futures awaited.
                self.to_await = core::mem::replace(&mut self.updated_to_await, Vec::new());
                // Restore the `has_root` flag.
                self.has_root = false;
            }

            // Set `is_parent` flag to signal to possible parent conditional that they are not a leaf node.
            self.outer_is_parent = true;
        }
    }
}
