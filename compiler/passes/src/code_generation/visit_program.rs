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

use crate::CodeGenerator;

use leo_ast::{Circuit, CircuitMember, Function, Program};

use itertools::Itertools;
use std::{collections::HashMap, fmt::Write as _};

impl<'a> CodeGenerator<'a> {
    pub(crate) fn visit_program(&mut self, input: &'a Program) -> String {
        let mut program_string = format!("program {}.{};\n", input.name, input.network);

        // Visit each `Circuit` or `Record` in the Leo AST and produce a bytecode circuit.
        program_string.push_str(
            &input
                .circuits
                .values()
                .map(|circuit| self.visit_circuit_or_record(circuit))
                .join("\n"),
        );

        program_string.push('\n');

        // Visit each `Function` in the Leo AST and produce a bytecode function.
        program_string.push_str(
            &input
                .functions
                .values()
                .map(|function| self.visit_function(function))
                .join("\n"),
        );

        program_string
    }

    fn visit_circuit_or_record(&mut self, circuit: &'a Circuit) -> String {
        if circuit.is_record {
            self.visit_record(circuit)
        } else {
            self.visit_circuit(circuit)
        }
    }

    fn visit_circuit(&mut self, circuit: &'a Circuit) -> String {
        // Add private symbol to composite types.
        self.composite_mapping
            .insert(&circuit.identifier.name, String::from("private")); // todo: private by default here.

        let mut output_string = format!("interface {}:\n", circuit.identifier.to_string().to_lowercase()); // todo: check if this is safe from name conflicts.

        // Construct and append the record variables.
        for var in circuit.members.iter() {
            let (name, type_) = match var {
                CircuitMember::CircuitVariable(name, type_) => (name, type_),
            };

            writeln!(output_string, "    {} as {};", name, type_,).expect("failed to write to string");
        }

        output_string
    }

    fn visit_record(&mut self, record: &'a Circuit) -> String {
        // Add record symbol to composite types.
        let mut output_string = String::from("record");
        self.composite_mapping
            .insert(&record.identifier.name, output_string.clone());
        writeln!(output_string, " {}:", record.identifier.to_string().to_lowercase())
            .expect("failed to write to string"); // todo: check if this is safe from name conflicts.

        // Construct and append the record variables.
        for var in record.members.iter() {
            let (name, type_) = match var {
                CircuitMember::CircuitVariable(name, type_) => (name, type_),
            };

            writeln!(
                output_string,
                "    {} as {}.private;", // todo: CAUTION private record variables only.
                name, type_,
            )
            .expect("failed to write to string");
        }

        output_string
    }

    fn visit_function(&mut self, function: &'a Function) -> String {
        // Initialize the state of `self` with the appropriate values before visiting `function`.
        self.next_register = 0;
        self.variable_mapping = HashMap::new();
        self.current_function = Some(function);

        // Construct the header of the function.
        let mut function_string = format!("function {}:\n", function.identifier);

        // Construct and append the input declarations of the function.
        for input in function.input.iter() {
            let register_string = format!("r{}", self.next_register);
            self.next_register += 1;

            self.variable_mapping
                .insert(&input.get_variable().identifier.name, register_string.clone());

            let type_string =
                self.visit_type_with_visibility(&input.get_variable().type_, Some(input.get_variable().mode()));
            writeln!(function_string, "    input {} as {};", register_string, type_string,)
                .expect("failed to write to string");
        }

        //  Construct and append the function body.
        let block_string = self.visit_block(&function.block);
        function_string.push_str(&block_string);

        function_string
    }
}
