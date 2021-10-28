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
use std::borrow::Cow;

pub struct DotNode {
    pub id: String,
    pub name: String,
    pub labels: Vec<(&'static str, String)>,
}

pub struct DotEdge {
    pub start_idx: NodeIndex,
    pub end_idx: NodeIndex,
    pub label: String,
    pub color: &'static str,
}

pub struct DotGraph<'a> {
    id: String,
    graph: Graph<DotNode, DotEdge>,
    filter_keys: &'a [String],
}

impl<'a> DotGraph<'a> {
    pub fn new(id: String, filter_keys: &'a [String]) -> Self {
        DotGraph {
            id,
            graph: Graph::new(),
            filter_keys,
        }
    }

    pub fn add_node(&mut self, node: DotNode) -> NodeIndex {
        self.graph.add_node(node)
    }

    pub fn add_edge(&mut self, edge: DotEdge) -> EdgeIndex {
        // Prevents duplicate edges as traversals may go through paths multiple times
        self.graph.update_edge(edge.start_idx, edge.end_idx, edge)
    }
}

impl<'a> dot::Labeller<'a, (NodeIndex, &'a DotNode), (EdgeIndex, &'a DotEdge)> for DotGraph<'a> {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new(self.id.as_str()).unwrap()
    }

    fn node_id(&'a self, n: &(NodeIndex, &'a DotNode)) -> dot::Id<'a> {
        let &(i, _) = n;
        dot::Id::new(self.graph[i].id.as_str()).unwrap()
    }

    fn node_label(&'a self, n: &(NodeIndex, &'a DotNode)) -> dot::LabelText<'a> {
        let mut label = String::new();
        for (key, value) in &n.1.labels {
            if !self.filter_keys.contains(&String::from(*key)) {
                label.push_str(format!("{:}: {:}\n", key, value).as_str())
            }
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

impl<'a> dot::GraphWalk<'a, (NodeIndex, &'a DotNode), (EdgeIndex, &'a DotEdge)> for DotGraph<'a> {
    fn nodes(&'a self) -> dot::Nodes<'a, (NodeIndex, &'a DotNode)> {
        let mut dot_nodes = Vec::new();
        for (idx, node) in self.graph.node_indices().zip(self.graph.node_weights()) {
            dot_nodes.push((idx, node))
        }
        Cow::Owned(dot_nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a, (EdgeIndex, &'a DotEdge)> {
        let mut dot_edges = Vec::new();
        for (idx, edge) in self.graph.edge_indices().zip(self.graph.edge_weights()) {
            if !self.filter_keys.contains(&edge.label) {
                dot_edges.push((idx, edge))
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
