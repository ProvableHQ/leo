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

use petgraph::graph::NodeIndex;
use std::ops::Deref;

type M = Fixed<NodeIndex>;

impl<'a, 'b> MonoidalReducerProgram<'a, M> for Dotifier<'a, 'b> {
    fn reduce_function(&mut self, input: &'a Function<'a>, arguments: Vec<M>, body: M) -> M {
        let mut labels = vec![
            ("NodeID", input.id.to_string()),
            ("Name", input.name.borrow().name.to_string()),
            (
                "FunctionQualifier",
                match input.qualifier {
                    FunctionQualifier::SelfRef => "SelfRef",
                    FunctionQualifier::ConstSelfRef => "ConstSelfRef",
                    FunctionQualifier::MutSelfRef => "MutSelfRef",
                    FunctionQualifier::Static => "Static",
                }
                .to_string(),
            ),
            ("OutputType", input.output.to_string()),
            //TODO: Display for Vec<Annotation>
            ("Annotations", format!("{:?}", input.annotations)),
        ];

        Dotifier::add_span_info(&mut labels, &input.span);

        let start_idx = self.add_or_get_node(input.id, "Function".to_string(), labels);

        for (i, Fixed(end_idx)) in arguments.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("variable_{:}", i), "olive");
        }

        let Fixed(end_idx) = body;
        self.add_edge(start_idx, end_idx, "body".to_string(), "black");

        if let Some(circuit) = input.circuit.get() {
            self.edges.push((input.id, circuit.id, "circuit".to_string(), "green"))
        }

        Fixed(start_idx)
    }

    fn reduce_circuit_member(&mut self, input: &CircuitMember<'a>, function: Option<M>) -> M {
        if let CircuitMember::Variable(typ) = input {
            let id = self.context.get_id();
            let labels = vec![("NodeID", id.to_string()), ("Type", typ.to_string())];

            let start_idx = self.add_or_get_node(id, "Variable".to_string(), labels);
            Fixed(start_idx)
        } else {
            function.unwrap() // If circuit member is a function, monoidal director always visits it
        }
    }

    fn reduce_circuit(&mut self, input: &'a Circuit<'a>, members: Vec<M>) -> M {
        let mut labels = vec![
            ("NodeID", input.id.to_string()),
            ("Name", input.name.borrow().name.to_string()),
            (
                "CoreMapping",
                match input.core_mapping.borrow().deref() {
                    None => "None",
                    Some(s) => s.as_str(),
                }
                .to_string(),
            ),
        ];

        Dotifier::add_span_info(&mut labels, &input.span);

        let start_idx = self.add_or_get_node(input.id, "Circuit".to_string(), labels);

        for (i, Fixed(end_idx)) in members.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("member_{:}", i), "black");
        }

        Fixed(start_idx)
    }

    fn reduce_alias(&mut self, input: &'a Alias<'a>) -> M {
        let labels = vec![("NodeID", input.id.to_string())];
        let start_idx = self.add_or_get_node(input.id, "Alias".to_string(), labels);

        Fixed(start_idx)
    }

    fn reduce_program(
        &mut self,
        input: &Program,
        imported_modules: Vec<M>,
        aliases: Vec<M>,
        functions: Vec<M>,
        global_consts: Vec<M>,
        circuits: Vec<M>,
    ) -> M {
        let labels = vec![("NodeID", input.id.to_string()), ("Name", input.name.clone())];

        let start_idx = self.add_or_get_node(input.id, "Program".to_string(), labels);

        for (i, Fixed(end_idx)) in imported_modules.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("import_{:}", i), "orange");
        }

        for (i, Fixed(end_idx)) in aliases.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("alias_{:}", i), "pink");
        }

        for (i, Fixed(end_idx)) in functions.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("function_{:}", i), "black");
        }

        for (i, Fixed(end_idx)) in global_consts.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("global_const_{:}", i), "purple");
        }

        for (i, Fixed(end_idx)) in circuits.iter().enumerate() {
            self.add_edge(start_idx, *end_idx, format!("circuit_{:}", i), "black");
        }

        self.add_remaining_edges();

        Fixed(start_idx)
    }
}
