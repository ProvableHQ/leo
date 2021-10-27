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
        let mut label = format!(
            "Function: {:}\nNode ID: {:}\n\nFunction Qualifier: {:}\nOutput: {:}\nAnnotations: ",
            input.name.borrow().name.to_string(),
            input.id,
            match input.qualifier {
                FunctionQualifier::SelfRef => "SelfRef",
                FunctionQualifier::ConstSelfRef => "ConstSelfRef",
                FunctionQualifier::MutSelfRef => "MutSelfRef",
                FunctionQualifier::Static => "Static",
            },
            input.output,
        );

        for anno in &input.annotations {
            label.push_str(format!("{:},", anno).as_str())
        }

        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);
        for (i, Fixed(end_idx)) in arguments.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("variable_{:}", i), LabelType::Label),
                Some(("olive".to_string(), LabelType::Label)),
            );
        }

        let Fixed(end_idx) = body;
        self.add_edge(
            start_idx,
            end_idx,
            ("body".to_string(), LabelType::Label),
            Some(("black".to_string(), LabelType::Label)),
        );

        if let Some(circuit) = input.circuit.get() {
            self.edges.push((
                input.id,
                circuit.id,
                ("circuit".to_string(), LabelType::Label),
                Some(("green".to_string(), LabelType::Label)),
            ))
        }

        Fixed(start_idx)
    }

    fn reduce_circuit_member(&mut self, input: &CircuitMember<'a>, function: Option<M>) -> M {
        if let CircuitMember::Variable(typ) = input {
            let id = self.context.get_id();
            let label = format!("Variable\nNode ID: {:}\n\nType: {:}", id, typ,);
            let start_idx = self.add_or_get_node(id, label, LabelType::Esc);
            Fixed(start_idx)
        } else {
            function.unwrap() // If circuit member is a function, monoidal director always visits it
        }
    }

    fn reduce_circuit(&mut self, input: &'a Circuit<'a>, members: Vec<M>) -> M {
        let label = format!(
            "Circuit: {:}\nNode ID: {:}\n\nCore Mapping: {:}\n\n{:}",
            input.name.borrow().name.to_string(),
            input.id,
            match input.core_mapping.borrow().deref() {
                None => "None",
                Some(s) => s.as_str(),
            },
            Dotifier::generate_span_info(&input.span)
        );
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

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

    fn reduce_alias(&mut self, input: &'a Alias<'a>) -> M {
        let label = format!("Alias\nNode ID: {:}\n\n", input.id);
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

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
        let label = format!("Program: {:}\nNode ID: {:}", input.name, input.id);
        let start_idx = self.add_or_get_node(input.id, label, LabelType::Esc);

        for (i, Fixed(end_idx)) in imported_modules.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("import_{:}", i), LabelType::Label),
                Some(("orange".to_string(), LabelType::Label)),
            );
        }

        for (i, Fixed(end_idx)) in aliases.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("alias_{:}", i), LabelType::Label),
                Some(("pink".to_string(), LabelType::Label)),
            );
        }

        for (i, Fixed(end_idx)) in functions.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("function_{:}", i), LabelType::Label),
                Some(("black".to_string(), LabelType::Label)),
            );
        }

        for (i, Fixed(end_idx)) in global_consts.iter().enumerate() {
            self.add_edge(
                start_idx,
                *end_idx,
                (format!("global_const_{:}", i), LabelType::Label),
                Some(("purple".to_string(), LabelType::Label)),
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
