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
use super::*;

use leo_asg::*;
use leo_errors::Span;

use petgraph::graph::NodeIndex;
use std::collections::HashMap;

/// An ASG pass for constructung
pub struct Dotifier<'a, 'b> {
    pub graph: DotGraph,
    pub context: &'b AsgContext<'a>,
    pub id_map: HashMap<u32, NodeIndex>,
    pub edges: Vec<(u32, u32, String, &'static str)>, // For edges that are meant to be added after entire ASG is traversed
}

impl<'a, 'b> Dotifier<'a, 'b> {
    /// Returns a new `Dotifier`.
    pub fn new(graph: DotGraph, context: &'b AsgContext<'a>) -> Self {
        Dotifier {
            graph,
            context,
            id_map: HashMap::new(),
            edges: Vec::new(),
        }
    }

    /// Adds a node to the DotGraph if it has not been added yet. Returns the corresponding NodeIndex.
    pub fn add_or_get_node(&mut self, id: u32, name: String, labels: Vec<(&'static str, String)>) -> NodeIndex {
        let &mut Dotifier {
            ref mut id_map,
            ref mut graph,
            context: _,
            edges: _,
        } = self;
        *id_map.entry(id).or_insert_with(|| {
            let node = DotNode {
                id: format!("N{:}", id),
                name,
                labels,
            };
            graph.add_node(node)
        })
    }

    /// Adds a DotEdge to the underlying DotGraph.
    pub fn add_edge(&mut self, start_idx: NodeIndex, end_idx: NodeIndex, label: String, color: &'static str) {
        let edge = DotEdge {
            start_idx,
            end_idx,
            label,
            color,
        };
        self.graph.add_edge(edge);
    }

    /// Creates a DotEdge for each element in `self.edges` and adds it to the underlying DotGraph.
    pub fn add_remaining_edges(&mut self) {
        for (start_id, end_id, label, color) in self.edges.drain(..) {
            //let start_idx = self.id_map.get(&start_id).unwrap(); // All nodes should have been added to ID map
            //let end_idx = self.id_map.get(&end_id).unwrap(); // All nodes should have been added to ID map
            //todo: ASG passes can leave references to nodes that are no longer part of the ASG
            //note: skipping for now
            if let (Some(start_idx), Some(end_idx)) = (self.id_map.get(&start_id), self.id_map.get(&end_id)) {
                let edge = DotEdge {
                    start_idx: *start_idx,
                    end_idx: *end_idx,
                    label,
                    color,
                };
                self.graph.add_edge(edge);
            }
        }
    }

    /// Optinally adds labels for information contained in `Span`
    pub fn add_span_info(labels: &mut Vec<(&'a str, String)>, span: &Option<Span>) {
        if let Some(span) = span {
            labels.push(("File", span.path.to_string()));
            labels.push(("Location", format!("{:}", span)));
            labels.push(("Content", span.content.to_string()));
        }
    }

    /// Returns a string representation for `Option<Type<'a>>`.
    pub fn generate_type_info(typ: Option<Type<'a>>) -> String {
        if let Some(typ) = typ {
            format!("{:}", typ)
        } else {
            "None".to_string()
        }
    }
}
