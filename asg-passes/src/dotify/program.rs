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

type M = Fixed<NodeIndex>;

impl<'a, 'b> MonoidalReducerProgram<'a, M> for Dotifier<'a, 'b> {
    fn reduce_function(&mut self, input: &'a Function<'a>, body: M) -> M {
        let label = format!(
            "Function: {:}\nNode ID: {:}",
            input.name.borrow().name.to_string(),
            input.id
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);
        let Fixed(end_idx) = body;

        // TODO: Output type
        //TODO: arguments
        //TODO: qualifier
        //TODO: annotations
        //TODO: circuit

        self.add_edge(
            start_idx,
            end_idx,
            ("body".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        Fixed(start_idx)
    }

    fn reduce_circuit_member(&mut self, input: &CircuitMember<'a>, function: Option<M>) -> M {
        //TODO: Need to figure out how to reduce types
        //TODO: Might need to fix monoidal director
        let start_idx = self.add_or_get_node(self.context.get_id(), "circuit_member".to_string(), LabelType::Label);
        Fixed(start_idx)
    }

    fn reduce_circuit(&mut self, input: &'a Circuit<'a>, members: Vec<M>) -> M {
        let label = format!(
            "Circuit: {:}\nNode ID: {:}\n\n{:}",
            input.name.borrow().name.to_string(),
            input.id,
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        //TODO: Core mapping?

        for (i, Fixed(end_idx)) in members.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("member_{:}", i), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            );
        }

        Fixed(start_idx)
    }

    fn reduce_program(&mut self, input: &Program, imported_modules: Vec<M>, functions: Vec<M>, circuits: Vec<M>) -> M {
        let label = format!("Program: {:}\nNode ID: {:}", input.name, input.id);
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        //TODO: imported modules?
        //TODO: Aliases
        //TODO: global consts

        for (i, Fixed(end_idx)) in functions.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("function_{:}", i), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            );
        }

        for (i, Fixed(end_idx)) in circuits.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("circuit_{:}", i), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            );
        }

        self.add_remaining_edges();

        Fixed(start_idx)
    }
}
