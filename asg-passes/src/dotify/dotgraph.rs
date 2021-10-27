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

use petgraph::graph::{EdgeIndex, NodeIndex};
use std::borrow::{Borrow, Cow};

pub enum LabelType {
    Label,
    Esc,
    Html,
}

pub struct DotNode {
    pub id: String,
    pub shape: Option<(String, LabelType)>,
    pub label: (String, LabelType),
    pub style: dot::Style,
    pub color: Option<(String, LabelType)>,
}

pub struct DotEdge {
    pub start_idx: NodeIndex,
    pub end_idx: NodeIndex,
    pub label: (String, LabelType),
    pub end_arrow: dot::ArrowShape,
    pub start_arrow: dot::ArrowShape,
    pub style: dot::Style,
    pub color: Option<(String, LabelType)>,
}

pub struct DotGraph {
    id: String,
    graph: petgraph::Graph<DotNode, DotEdge>,
}

impl DotGraph {
    pub fn new(id: String) -> Self {
        DotGraph {
            id,
            graph: petgraph::Graph::new(),
        }
    }

    pub fn add_node(&mut self, node: DotNode) -> NodeIndex {
        self.graph.add_node(node)
    }

    pub fn add_edge(&mut self, edge: DotEdge) -> EdgeIndex {
        // Prevents duplicate edges
        self.graph.update_edge(edge.start_idx, edge.end_idx, edge)
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

    fn node_shape(&'a self, n: &(NodeIndex, &'a DotNode)) -> Option<dot::LabelText<'a>> {
        let &(i, _) = n;
        if let Some((str, typ)) = self.graph[i].shape.borrow() {
            Some(create_label_text(str, typ))
        } else {
            None
        }
    }

    fn node_label(&'a self, n: &(NodeIndex, &'a DotNode)) -> dot::LabelText<'a> {
        let &(i, _) = n;
        let (str, typ) = self.graph[i].label.borrow();
        create_label_text(str, typ)
    }

    fn edge_label(&'a self, e: &(EdgeIndex, &'a DotEdge)) -> dot::LabelText<'a> {
        let &(i, _) = e;
        let (str, typ) = self.graph[i].label.borrow();
        create_label_text(str, typ)
    }

    fn node_style(&'a self, n: &(NodeIndex, &'a DotNode)) -> dot::Style {
        n.1.style
    }

    fn node_color(&'a self, n: &(NodeIndex, &'a DotNode)) -> Option<dot::LabelText<'a>> {
        let &(i, _) = n;
        if let Some((str, typ)) = self.graph[i].color.borrow() {
            Some(create_label_text(str, typ))
        } else {
            None
        }
    }

    fn edge_end_arrow(&'a self, e: &(EdgeIndex, &'a DotEdge)) -> dot::Arrow {
        dot::Arrow::from_arrow(e.1.end_arrow)
    }

    fn edge_start_arrow(&'a self, e: &(EdgeIndex, &'a DotEdge)) -> dot::Arrow {
        dot::Arrow::from_arrow(e.1.start_arrow)
    }

    fn edge_style(&'a self, e: &(EdgeIndex, &'a DotEdge)) -> dot::Style {
        e.1.style
    }

    fn edge_color(&'a self, e: &(EdgeIndex, &'a DotEdge)) -> Option<dot::LabelText<'a>> {
        let &(i, _) = e;
        if let Some((str, typ)) = self.graph[i].color.borrow() {
            Some(create_label_text(str, typ))
        } else {
            None
        }
    }

    fn kind(&self) -> dot::Kind {
        dot::Kind::Digraph
    }
}

fn create_label_text<'a>(str: &'a str, typ: &LabelType) -> dot::LabelText<'a> {
    match typ {
        LabelType::Label => dot::LabelText::label(str),
        LabelType::Esc => dot::LabelText::escaped(str),
        LabelType::Html => dot::LabelText::html(str),
    }
}

impl<'a> dot::GraphWalk<'a, (NodeIndex, &'a DotNode), (EdgeIndex, &'a DotEdge)> for DotGraph {
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
            dot_edges.push((idx, edge))
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

trait LabelGen {
    fn generate_label() -> DotNode;
}

impl<'a> LabelGen for dyn leo_asg::ExpressionNode<'a> {
    fn generate_label() -> DotNode {
        todo!()
    }
}
