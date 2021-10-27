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

use dot::{ArrowShape, Style};
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

pub struct Dotifier<'a, 'b> {
    pub graph: DotGraph,
    pub context: &'b AsgContext<'a>,
    pub id_map: HashMap<u32, NodeIndex>,
    pub edges: Vec<(u32, u32, (String, LabelType), Option<(String, LabelType)>)>, // For edges that are meant to be added after entire ASG is traversed
}

impl<'a, 'b> Dotifier<'a, 'b> {
    pub fn new(graph: DotGraph, context: &'b AsgContext<'a>) -> Self {
        Dotifier {
            graph,
            context,
            id_map: HashMap::new(),
            edges: Vec::new(),
        }
    }

    // Helper functions to make it easier to construct graphs
    pub fn add_or_get_node(&mut self, id: u32, label: String, typ: LabelType) -> NodeIndex {
        let &mut Dotifier {
            ref mut id_map,
            ref mut graph,
            context: _,
            edges: _,
        } = self;
        *id_map.entry(id).or_insert_with(|| {
            let node = DotNode {
                id: format!("N{:}", id),
                shape: None,
                label: (label, typ),
                style: Style::None,
                color: None,
            };
            graph.add_node(node)
        })
    }

    pub fn add_edge(
        &mut self,
        start_idx: NodeIndex,
        end_idx: NodeIndex,
        label: (String, LabelType),
        color: Option<(String, LabelType)>,
    ) {
        let edge = DotEdge {
            start_idx,
            end_idx,
            label,
            end_arrow: ArrowShape::NoArrow,
            start_arrow: ArrowShape::NoArrow,
            style: Style::None,
            color,
        };
        self.graph.add_edge(edge);
    }

    pub fn add_remaining_edges(&mut self) {
        for (start_id, end_id, label, color) in self.edges.drain(..) {
            let start_idx = self.id_map.get(&start_id).unwrap(); // All nodes should have been added to ID map
            let end_idx = self.id_map.get(&end_id).unwrap(); // All nodes should have been added to ID map
            let edge = DotEdge {
                start_idx: *start_idx,
                end_idx: *end_idx,
                label,
                end_arrow: ArrowShape::NoArrow,
                start_arrow: ArrowShape::NoArrow,
                style: Style::None,
                color,
            };
            self.graph.add_edge(edge);
        }
    }

    pub fn generate_span_info(span: &Option<Span>) -> String {
        if let Some(span) = span {
            format!(
                "File: {:}\nLocation: {:}\n\nContent: {:}",
                span.path.to_string(),
                span,
                span.content.to_string(),
            )
        } else {
            "".to_string()
        }
    }

    pub fn generate_type_info(typ: Option<Type<'a>>) -> String {
        if let Some(typ) = typ {
            format!("{:}", typ)
        } else {
            "None".to_string()
        }
    }
}
