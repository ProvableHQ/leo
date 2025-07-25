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

use crate::Location;
use leo_span::Symbol;

use indexmap::{IndexMap, IndexSet};
use std::{fmt::Debug, hash::Hash, rc::Rc};

/// A struct dependency graph.
/// The `Vec<Symbol>` is to the absolute path to each struct
pub type StructGraph = DiGraph<Vec<Symbol>>;

/// A call graph.
pub type CallGraph = DiGraph<Location>;

/// An import dependency graph.
pub type ImportGraph = DiGraph<Symbol>;

/// A node in a graph.
pub trait GraphNode: Clone + 'static + Eq + PartialEq + Debug + Hash {}

impl<T> GraphNode for T where T: 'static + Clone + Eq + PartialEq + Debug + Hash {}

/// Errors in directed graph operations.
#[derive(Debug)]
pub enum DiGraphError<N: GraphNode> {
    /// An error that is emitted when a cycle is detected in the directed graph. Contains the path of the cycle.
    CycleDetected(Vec<N>),
}

/// A directed graph using reference-counted nodes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiGraph<N: GraphNode> {
    /// The set of nodes in the graph.
    nodes: IndexSet<Rc<N>>,

    /// The directed edges in the graph.
    /// Each entry in the map is a node in the graph, and the set of nodes that it points to.
    edges: IndexMap<Rc<N>, IndexSet<Rc<N>>>,
}

impl<N: GraphNode> Default for DiGraph<N> {
    fn default() -> Self {
        Self { nodes: IndexSet::new(), edges: IndexMap::new() }
    }
}

impl<N: GraphNode> DiGraph<N> {
    /// Initializes a new `DiGraph` from a set of source nodes.
    pub fn new(nodes: IndexSet<N>) -> Self {
        let nodes: IndexSet<_> = nodes.into_iter().map(Rc::new).collect();
        Self { nodes, edges: IndexMap::new() }
    }

    /// Adds a node to the graph.
    pub fn add_node(&mut self, node: N) {
        self.nodes.insert(Rc::new(node));
    }

    /// Returns an iterator over the nodes in the graph.
    pub fn nodes(&self) -> impl Iterator<Item = &N> {
        self.nodes.iter().map(|rc| rc.as_ref())
    }

    /// Adds an edge to the graph.
    pub fn add_edge(&mut self, from: N, to: N) {
        // Add `from` and `to` to the set of nodes if they are not already in the set.
        let from_rc = self.get_or_insert(from);
        let to_rc = self.get_or_insert(to);

        // Add the edge to the adjacency list.
        self.edges.entry(from_rc).or_default().insert(to_rc);
    }

    /// Removes a node and all associated edges from the graph.
    pub fn remove_node(&mut self, node: &N) -> bool {
        if let Some(rc_node) = self.nodes.shift_take(&Rc::new(node.clone())) {
            // Remove all outgoing edges from the node
            self.edges.shift_remove(&rc_node);

            // Remove all incoming edges to the node
            for targets in self.edges.values_mut() {
                targets.shift_remove(&rc_node);
            }
            true
        } else {
            false
        }
    }

    /// Returns an iterator to the immediate neighbors of a given node.
    pub fn neighbors(&self, node: &N) -> impl Iterator<Item = &N> {
        self.edges
            .get(node) // ← no Rc::from() needed!
            .into_iter()
            .flat_map(|neighbors| neighbors.iter().map(|rc| rc.as_ref()))
    }

    /// Returns `true` if the graph contains the given node.
    pub fn contains_node(&self, node: N) -> bool {
        self.nodes.contains(&Rc::new(node))
    }

    /// Returns the post-order ordering of the graph.
    /// Detects if there is a cycle in the graph.
    pub fn post_order(&self) -> Result<IndexSet<N>, DiGraphError<N>> {
        self.post_order_with_filter(|_| true)
    }

    /// Returns the post-order ordering of the graph but only considering a subset of the nodes that
    /// satisfy the given filter.
    ///
    /// Detects if there is a cycle in the graph.
    pub fn post_order_with_filter<F>(&self, filter: F) -> Result<IndexSet<N>, DiGraphError<N>>
    where
        F: Fn(&N) -> bool,
    {
        // The set of nodes that do not need to be visited again.
        let mut finished = IndexSet::with_capacity(self.nodes.len());

        // Perform a depth-first search of the graph, starting from `node`, for each node in the graph that satisfies
        // `is_entry_point`.
        for node_rc in self.nodes.iter().filter(|n| filter(n.as_ref())) {
            // If the node has not been explored, explore it.
            if !finished.contains(node_rc) {
                // The set of nodes that are on the path to the current node in the searc
                let mut discovered = IndexSet::new();
                // Check if there is a cycle in the graph starting from `node`.
                if let Some(cycle_node) = self.contains_cycle_from(node_rc, &mut discovered, &mut finished) {
                    let mut path = vec![cycle_node.as_ref().clone()];
                    // Backtrack through the discovered nodes to find the cycle.
                    while let Some(next) = discovered.pop() {
                        // Add the node to the path.
                        path.push(next.as_ref().clone());
                        // If the node is the same as the first node in the path, we have found the cycle.
                        if Rc::ptr_eq(&next, &cycle_node) {
                            break;
                        }
                    }
                    // Reverse the path to get the cycle in the correct order.
                    path.reverse();
                    // A cycle was detected. Return the path of the cycle.
                    return Err(DiGraphError::CycleDetected(path));
                }
            }
        }

        // No cycle was found. Return the set of nodes in topological order.
        Ok(finished.iter().map(|rc| (**rc).clone()).collect())
    }

    /// Retains a subset of the nodes, and removes all edges in which the source or destination is not in the subset.
    pub fn retain_nodes(&mut self, keep: &IndexSet<N>) {
        let keep: IndexSet<_> = keep.iter().map(|n| Rc::new(n.clone())).collect();
        // Remove the nodes from the set of nodes.
        self.nodes.retain(|n| keep.contains(n));
        self.edges.retain(|n, _| keep.contains(n));
        // Remove the edges that reference the nodes.
        for targets in self.edges.values_mut() {
            targets.retain(|t| keep.contains(t));
        }
    }

    // Detects if there is a cycle in the graph starting from the given node, via a recursive depth-first search.
    // If there is no cycle, returns `None`.
    // If there is a cycle, returns the node that was most recently discovered.
    // Nodes are added to `finished` in post-order order.
    fn contains_cycle_from(
        &self,
        node: &Rc<N>,
        discovered: &mut IndexSet<Rc<N>>,
        finished: &mut IndexSet<Rc<N>>,
    ) -> Option<Rc<N>> {
        // Add the node to the set of discovered nodes.
        discovered.insert(node.clone());

        // Check each outgoing edge of the node.
        if let Some(children) = self.edges.get(node) {
            for child in children {
                // If the node already been discovered, there is a cycle.
                if discovered.contains(child) {
                    // Insert the child node into the set of discovered nodes; this is used to reconstruct the cycle.
                    // Note that this case is always hit when there is a cycle.
                    return Some(child.clone());
                }
                // If the node has not been explored, explore it.
                if !finished.contains(child) {
                    if let Some(cycle_node) = self.contains_cycle_from(child, discovered, finished) {
                        return Some(cycle_node);
                    }
                }
            }
        }

        // Remove the node from the set of discovered nodes.
        discovered.pop();
        // Add the node to the set of finished nodes.
        finished.insert(node.clone());
        None
    }

    /// Helper: get or insert Rc<N> into the graph.
    fn get_or_insert(&mut self, node: N) -> Rc<N> {
        if let Some(existing) = self.nodes.get(&node) {
            return existing.clone();
        }
        let rc = Rc::new(node);
        self.nodes.insert(rc.clone());
        rc
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_post_order<N: GraphNode>(graph: &DiGraph<N>, expected: &[N]) {
        let result = graph.post_order();
        assert!(result.is_ok());

        let order: Vec<N> = result.unwrap().into_iter().collect();
        assert_eq!(order, expected);
    }

    #[test]
    fn test_post_order() {
        let mut graph = DiGraph::<u32>::new(IndexSet::new());

        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        check_post_order(&graph, &[5, 4, 2, 3, 1]);

        let mut graph = DiGraph::<u32>::new(IndexSet::new());

        // F -> B
        graph.add_edge(6, 2);
        // B -> A
        graph.add_edge(2, 1);
        // B -> D
        graph.add_edge(2, 4);
        // D -> C
        graph.add_edge(4, 3);
        // D -> E
        graph.add_edge(4, 5);
        // F -> G
        graph.add_edge(6, 7);
        // G -> I
        graph.add_edge(7, 9);
        // I -> H
        graph.add_edge(9, 8);

        // A, C, E, D, B, H, I, G, F.
        check_post_order(&graph, &[1, 3, 5, 4, 2, 8, 9, 7, 6]);
    }

    #[test]
    fn test_cycle() {
        let mut graph = DiGraph::<u32>::new(IndexSet::new());

        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(2, 4);
        graph.add_edge(4, 1);

        let result = graph.post_order();
        assert!(result.is_err());

        let DiGraphError::CycleDetected(cycle) = result.unwrap_err();
        let expected = Vec::from([1u32, 2, 4, 1]);
        assert_eq!(cycle, expected);
    }

    #[test]
    fn test_unconnected_graph() {
        let graph = DiGraph::<u32>::new(IndexSet::from([1, 2, 3, 4, 5]));

        check_post_order(&graph, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_retain_nodes() {
        let mut graph = DiGraph::<u32>::new(IndexSet::new());

        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 5);
        graph.add_edge(2, 3);
        graph.add_edge(2, 4);
        graph.add_edge(2, 5);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        let mut nodes = IndexSet::new();
        nodes.insert(1);
        nodes.insert(2);
        nodes.insert(3);

        graph.retain_nodes(&nodes);

        let mut expected = DiGraph::<u32>::new(IndexSet::new());
        expected.add_edge(1, 2);
        expected.add_edge(1, 3);
        expected.add_edge(2, 3);
        expected.edges.insert(3.into(), IndexSet::new());

        assert_eq!(graph, expected);
    }
}
