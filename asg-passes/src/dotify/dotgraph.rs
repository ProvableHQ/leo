// Copyright (C) 2019-2021 Aleo Systems Inc.
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
use petgraph::graph::{EdgeIndex, Graph, NodeIndex};
use petgraph::visit::{DfsPostOrder, EdgeRef};
use petgraph::Direction;
use std::borrow::Cow;
use std::iter;

//todo: Comments

/// A node in a DOT graph.
pub struct DotNode {
    pub id: String,
    pub name: String,
    pub labels: Vec<(&'static str, String)>,
}

impl DotNode {
    /// Initialize a node without labels.
    pub fn new(id: String, name: String) -> Self {
        DotNode {
            id,
            name,
            labels: Vec::new(),
        }
    }

    /// Remove labels from the DotNode.
    pub fn filter_labels(&mut self, excluded_labels: &[String]) {
        self.labels
            .retain(|(key, _)| !excluded_labels.iter().any(|label| label == key));
    }
}

/// An edge in a DOT graph.
pub struct DotEdge {
    pub start_idx: NodeIndex,
    pub end_idx: NodeIndex,
    pub label: String,
    pub color: &'static str,
}

/// A directed graph that can be rendered into the DOT language.
pub struct DotGraph {
    id: String,
    graph: Graph<DotNode, DotEdge>,
    source: NodeIndex,
}

impl DotGraph {
    /// Returns a new DotGraph without any nodes or edges.
    pub fn new(id: String) -> Self {
        DotGraph {
            id,
            graph: Graph::new(),
            source: NodeIndex::default(),
        }
    }

    /// Returns the source node in the DotGraph.
    pub fn get_source(&self) -> NodeIndex {
        self.source
    }

    /// Set a source node in the DotGraph.
    pub fn set_source(&mut self, idx: NodeIndex) {
        self.source = idx;
    }

    /// Add a node to the DotGraph.
    pub fn add_node(&mut self, node: DotNode) -> NodeIndex {
        self.graph.add_node(node)
    }

    /// Add a node to the DotGraph.
    pub fn add_edge(&mut self, edge: DotEdge) -> EdgeIndex {
        // Prevents duplicate edges as traversals may go through paths multiple times
        self.graph.update_edge(edge.start_idx, edge.end_idx, edge)
    }

    /// Remove labels from all nodes in the DotGraph.
    pub fn filter_node_labels(&mut self, excluded_labels: &[String]) {
        for node in self.graph.node_weights_mut() {
            node.filter_labels(excluded_labels)
        }
    }

    /// Remove edges with certain labels from the DotGraph.
    pub fn filter_node_edges(&mut self, excluded_edges: &[String]) {
        self.graph.retain_edges(|graph, edge_idx| {
            let edge = &graph[edge_idx];
            !excluded_edges.contains(&edge.label)
        });
    }

    /// Returns a vector of nodes indices reachable from the source node.
    pub fn get_reachable_set(&self) -> Vec<NodeIndex> {
        let mut dfs = DfsPostOrder::new(&self.graph, self.source);
        iter::from_fn(|| dfs.next(&self.graph)).collect()
    }
}


impl<'a> dot::Labeller<'a, (NodeIndex, &'a DotNode), (EdgeIndex, &'a DotEdge)> for DotGraph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new(self.id.as_str()).unwrap()
    }

    fn node_id(&'a self, n: &(NodeIndex, &'a DotNode)) -> dot::Id<'a> {
        let &(i, _) = n;
        dot::Id::new(self.graph[i].id.as_str()).unwrap()
    }

    fn node_label(&'a self, n: &(NodeIndex, &'a DotNode)) -> dot::LabelText<'a> {
        let mut label = n.1.name.clone();
        for (key, value) in &n.1.labels {
            label.push_str(format!("\n{:}: {:}", key, value).as_str())
        }
        dot::LabelText::escaped(label)
    }

    fn edge_label(&'a self, e: &(EdgeIndex, &'a DotEdge)) -> dot::LabelText<'a> {
        dot::LabelText::label(e.1.label.as_str())
    }

    fn edge_end_arrow(&'a self, _e: &(EdgeIndex, &'a DotEdge)) -> dot::Arrow {
        dot::Arrow::from_arrow(dot::ArrowShape::Normal(dot::Fill::Filled, dot::Side::Both))
    }

    fn edge_color(&'a self, e: &(EdgeIndex, &'a DotEdge)) -> Option<dot::LabelText<'a>> {
        Some(dot::LabelText::label(e.1.color))
    }
}

impl<'a> dot::GraphWalk<'a, (NodeIndex, &'a DotNode), (EdgeIndex, &'a DotEdge)> for DotGraph {
    fn nodes(&'a self) -> dot::Nodes<'a, (NodeIndex, &'a DotNode)> {
        let mut dot_nodes = Vec::new();
        for idx in self.get_reachable_set() {
            dot_nodes.push((idx, &self.graph[idx]))
        }
        Cow::Owned(dot_nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a, (EdgeIndex, &'a DotEdge)> {
        let mut dot_edges = Vec::new();
        for idx in self.get_reachable_set() {
            for edge in self.graph.edges_directed(idx, Direction::Outgoing) {
                dot_edges.push((edge.id(), edge.weight()));
            }
        }
        Cow::Owned(dot_edges)
    }

    fn source(&'a self, e: &(EdgeIndex, &'a DotEdge)) -> (NodeIndex, &'a DotNode) {
        let &(_, edge) = e;
        (edge.start_idx, &self.graph[edge.start_idx])
    }

    fn target(&'a self, e: &(EdgeIndex, &'a DotEdge)) -> (NodeIndex, &'a DotNode) {
        let &(_, edge) = e;
        (edge.end_idx, &self.graph[edge.end_idx])
    }
}

#[cfg(test)]
mod tests {
    use crate::dotify::dotgraph::{DotEdge, DotGraph, DotNode};
    use std::error::Error;

    #[test]
    fn test_render() -> Result<(), Box<dyn Error>> {
        let mut graph = DotGraph::new("example1".to_string());

        let mut add_node = |id: &str| graph.add_node(DotNode::new(id.to_string(), "".to_string()));

        let node0 = add_node("N0");
        let node1 = add_node("N1");
        let node2 = add_node("N2");
        let node3 = add_node("N3");
        let node4 = add_node("N4");

        let mut add_edge = |start_idx, end_idx| {
            let edge = DotEdge {
                start_idx,
                end_idx,
                label: "".to_string(),
                color: "black",
            };
            graph.add_edge(edge)
        };

        add_edge(node0, node1);
        add_edge(node0, node2);
        add_edge(node1, node3);
        add_edge(node2, node3);
        add_edge(node3, node4);
        add_edge(node4, node4);

        let mut raw_output = Vec::new();
        dot::render(&graph, &mut raw_output)?;

        let output = String::from_utf8(raw_output)?;

        let expected_output = String::from(
            "\
        digraph example1 {\n\
        \x20\x20\x20\x20N4[label=\"\"];\n\
        \x20\x20\x20\x20N3[label=\"\"];\n\
        \x20\x20\x20\x20N1[label=\"\"];\n\
        \x20\x20\x20\x20N2[label=\"\"];\n\
        \x20\x20\x20\x20N0[label=\"\"];\n\
        \x20\x20\x20\x20N4 -> N4[label=\"\"][color=\"black\"][arrowhead=\"normal\"];\n\
        \x20\x20\x20\x20N3 -> N4[label=\"\"][color=\"black\"][arrowhead=\"normal\"];\n\
        \x20\x20\x20\x20N1 -> N3[label=\"\"][color=\"black\"][arrowhead=\"normal\"];\n\
        \x20\x20\x20\x20N2 -> N3[label=\"\"][color=\"black\"][arrowhead=\"normal\"];\n\
        \x20\x20\x20\x20N0 -> N2[label=\"\"][color=\"black\"][arrowhead=\"normal\"];\n\
        \x20\x20\x20\x20N0 -> N1[label=\"\"][color=\"black\"][arrowhead=\"normal\"];\n\
        }\n",
        );

        assert_eq!(output, expected_output);

        Ok(())
    }

    #[test]
    fn test_render_reachable_set() -> Result<(), Box<dyn Error>> {
        let mut graph = DotGraph::new("example1".to_string());

        let mut add_node = |id: &str| graph.add_node(DotNode::new(id.to_string(), "".to_string()));

        let node0 = add_node("N0");
        let node1 = add_node("N1");
        let node2 = add_node("N2");
        let node3 = add_node("N3");
        let node4 = add_node("N4");

        let mut add_edge = |start_idx, end_idx| {
            let edge = DotEdge {
                start_idx,
                end_idx,
                label: "".to_string(),
                color: "black",
            };
            graph.add_edge(edge)
        };

        add_edge(node0, node1);
        add_edge(node1, node3);
        add_edge(node2, node3);
        add_edge(node3, node4);
        add_edge(node4, node4);

        graph.set_source(node0);

        let mut raw_output = Vec::new();
        dot::render(&graph, &mut raw_output)?;

        let output = String::from_utf8(raw_output)?;

        let expected_output = String::from(
            "\
        digraph example1 {\n\
        \x20\x20\x20\x20N4[label=\"\"];\n\
        \x20\x20\x20\x20N3[label=\"\"];\n\
        \x20\x20\x20\x20N1[label=\"\"];\n\
        \x20\x20\x20\x20N0[label=\"\"];\n\
        \x20\x20\x20\x20N4 -> N4[label=\"\"][color=\"black\"][arrowhead=\"normal\"];\n\
        \x20\x20\x20\x20N3 -> N4[label=\"\"][color=\"black\"][arrowhead=\"normal\"];\n\
        \x20\x20\x20\x20N1 -> N3[label=\"\"][color=\"black\"][arrowhead=\"normal\"];\n\
        \x20\x20\x20\x20N0 -> N1[label=\"\"][color=\"black\"][arrowhead=\"normal\"];\n\
        }\n",
        );

        assert_eq!(output, expected_output);

        Ok(())
    }
}
