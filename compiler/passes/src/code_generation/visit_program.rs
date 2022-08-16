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

use leo_ast::{Circuit, CircuitMember, Function, Identifier, Program};

use indexmap::IndexMap;
use itertools::Itertools;
use leo_span::sym;
use std::fmt::Write as _;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn visit_program(&mut self, input: &'a Program) -> String {
        // Accumulate instructions into a program string.
        let mut program_string = String::new();

        if !input.imports.is_empty() {
            // Visit each import statement and produce a Aleo import instruction.
            program_string.push_str(
                &input
                    .imports
                    .iter()
                    .map(|(identifier, imported_program)| self.visit_import(identifier, imported_program))
                    .join("\n"),
            );

            // Newline separator.
            program_string.push('\n');
        }

        // Print the program id.
        writeln!(program_string, "program {}.{};", input.name, input.network)
            .expect("Failed to write program id to string.");

        // Newline separator.
        program_string.push('\n');

        // Visit each `Circuit` or `Record` in the Leo AST and produce a Aleo interface instruction.
        program_string.push_str(
            &input
                .circuits
                .values()
                .map(|circuit| self.visit_circuit_or_record(circuit))
                .join("\n"),
        );

        // Newline separator.
        program_string.push('\n');

        // Store closures and functions in separate strings.
        let mut closures = String::new();
        let mut functions = String::new();

        // Visit each `Function` in the Leo AST and produce Aleo instructions.
        input.functions.values().for_each(|function| {
            // If the function is annotated with `@program`, then it is a program function.
            for annotation in function.annotations.iter() {
                if annotation.identifier.name == sym::program {
                    self.is_program_function = true;
                }
            }

            let function_string = self.visit_function(function);

            if self.is_program_function {
                functions.push_str(&function_string);
                functions.push('\n');
            } else {
                closures.push_str(&function_string);
                closures.push('\n');
            }

            // Unset the `is_program_function` flag.
            self.is_program_function = false;
        });

        // Closures must precede functions in the Aleo program.
        program_string.push_str(&closures);
        program_string.push('\n');
        program_string.push_str(&functions);

        program_string
    }

    fn visit_import(&mut self, import_name: &'a Identifier, import_program: &'a Program) -> String {
        // Load symbols into composite mapping.
        let _import_program_string = self.visit_program(import_program);
        // todo: We do not need the import program string because we generate instructions for imports separately during leo build.

        // Generate string for import statement.
        format!("import {}.aleo;", import_name)
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
            .insert(&circuit.identifier.name, (false, String::from("private"))); // todo: private by default here.

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
            .insert(&record.identifier.name, (true, output_string.clone()));
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
                name,
                type_.to_string().to_lowercase(),
            )
            .expect("failed to write to string");
        }

        output_string
    }

    fn visit_function(&mut self, function: &'a Function) -> String {
        // Initialize the state of `self` with the appropriate values before visiting `function`.
        self.next_register = 0;
        self.variable_mapping = IndexMap::new();
        self.current_function = Some(function);

        // Construct the header of the function.
        // If a function is a program function, generate an Aleo `function`, otherwise generate an Aleo `closure`.
        let mut function_string = match self.is_program_function {
            true => format!("function {}:\n", function.identifier),
            false => format!("closure {}:\n", function.identifier),
        };

        // Construct and append the input declarations of the function.
        for input in function.input.iter() {
            let register_string = format!("r{}", self.next_register);
            self.next_register += 1;

            self.variable_mapping
                .insert(&input.identifier.name, register_string.clone());

            let type_string = self.visit_type_with_visibility(&input.type_, input.mode());
            writeln!(function_string, "    input {} as {};", register_string, type_string,)
                .expect("failed to write to string");
        }

        //  Construct and append the function body.
        let block_string = self.visit_block(&function.block);
        function_string.push_str(&block_string);

        function_string
    }
}
