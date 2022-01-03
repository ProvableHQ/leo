// Copyright (C) 2019-2022 Aleo Systems Inc.
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
            ("Annotations", format!("{:?}", input.annotations)), //Note: Debug seems to work
        ];

        Dotifier::add_span_info(&mut labels, input.span.as_ref());

        let start_idx = self.add_or_get_node(input.id, "Function".to_string(), labels);

        self.enumerate_and_add_edges(start_idx, DotColor::Olive, "variable_", arguments);

        let Fixed(end_idx) = body;
        self.graph
            .add_edge(start_idx, end_idx, "body".to_string(), DotColor::Black);

        if let Some(circuit) = input.circuit.get() {
            self.edges
                .push((input.id, circuit.id, "circuit".to_string(), DotColor::Green))
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
        ];

        Dotifier::add_span_info(&mut labels, input.span.as_ref());

        let start_idx = self.add_or_get_node(input.id, "Circuit".to_string(), labels);
        self.enumerate_and_add_edges(start_idx, DotColor::Black, "member_", members);

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

        self.enumerate_and_add_edges(start_idx, DotColor::Orange, "import_", imported_modules);
        self.enumerate_and_add_edges(start_idx, DotColor::Pink, "alias_", aliases);
        self.enumerate_and_add_edges(start_idx, DotColor::Black, "function_", functions);
        self.enumerate_and_add_edges(start_idx, DotColor::Purple, "global_const_", global_consts);
        self.enumerate_and_add_edges(start_idx, DotColor::Black, "circuit_", circuits);

        self.add_remaining_edges();

        Fixed(start_idx)
    }
}
