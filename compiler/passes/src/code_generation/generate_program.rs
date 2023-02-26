// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use leo_ast::{
    functions, Function, FunctionConsumer, ImportConsumer, Mapping, MappingConsumer, Mode, Program, ProgramConsumer,
    ProgramScope, StatementConsumer, Struct, StructConsumer, Type, Variant,
};

use indexmap::IndexMap;
use itertools::Itertools;
use leo_span::sym;
use std::fmt::Write as _;

impl FunctionConsumer for CodeGenerator<'_> {
    type Output = String;

    fn consume_function(&mut self, function: Function) -> Self::Output {
        // Initialize the state of `self` with the appropriate values before consumeing `function`.
        self.next_register = 0;
        self.variable_mapping = IndexMap::new();
        // TODO: Figure out a better way to initialize.
        self.variable_mapping.insert(sym::SelfLower, "self".to_string());
        self.current_function = Some(function.identifier.name);

        // Construct the header of the function.
        // If a function is a program function, generate an Aleo `function`,
        // if it is a standard function generate an Aleo `closure`,
        // otherwise, it is an inline function, in which case a function should not be generated.
        let mut function_string = match function.variant {
            Variant::Transition => format!("function {}:\n", function.identifier),
            Variant::Standard => format!("closure {}:\n", function.identifier),
            Variant::Inline => return String::from("\n"),
        };

        // Construct and append the input declarations of the function.
        for input in function.input.into_iter() {
            let register_string = format!("r{}", self.next_register);
            self.next_register += 1;

            let type_string = match input {
                functions::Input::Internal(input) => {
                    self.variable_mapping
                        .insert(input.identifier.name, register_string.clone());
                    let visibility = match (self.is_transition_function, input.mode) {
                        (true, Mode::None) => Mode::Private,
                        _ => input.mode,
                    };
                    self.consume_type_with_visibility(&input.type_, visibility)
                }
                functions::Input::External(input) => {
                    self.variable_mapping
                        .insert(input.identifier.name, register_string.clone());
                    format!("{}.aleo/{}.record", input.program_name, input.record)
                }
            };

            writeln!(function_string, "    input {register_string} as {type_string};",)
                .expect("failed to write to string");
        }

        //  Construct and append the function body.
        let block_string = self.consume_block(function.block);
        function_string.push_str(&block_string);

        // If the finalize block exists, generate the appropriate bytecode.
        if let Some(finalize) = function.finalize {
            // Clear the register count.
            self.next_register = 0;
            self.in_finalize = true;

            // Clear the variable mapping.
            // TODO: Figure out a better way to initialize.
            self.variable_mapping = IndexMap::new();
            self.variable_mapping.insert(sym::SelfLower, "self".to_string());

            function_string.push_str(&format!("\nfinalize {}:\n", finalize.identifier));

            // Construct and append the input declarations of the finalize block.
            for input in finalize.input.into_iter() {
                let register_string = format!("r{}", self.next_register);
                self.next_register += 1;

                // TODO: Dedup code.
                let type_string = match input {
                    functions::Input::Internal(input) => {
                        self.variable_mapping
                            .insert(input.identifier.name, register_string.clone());

                        let visibility = match (self.is_transition_function, input.mode) {
                            (true, Mode::None) => Mode::Public,
                            _ => input.mode,
                        };
                        self.consume_type_with_visibility(&input.type_, visibility)
                    }
                    functions::Input::External(input) => {
                        self.variable_mapping
                            .insert(input.program_name.name, register_string.clone());
                        format!("{}.aleo/{}.record", input.program_name, input.record)
                    }
                };

                writeln!(function_string, "    input {register_string} as {type_string};",)
                    .expect("failed to write to string");
            }

            // Construct and append the finalize block body.
            function_string.push_str(&self.consume_block(finalize.block));

            self.in_finalize = false;
        }

        function_string
    }
}

impl StructConsumer for CodeGenerator<'_> {
    type Output = String;

    fn consume_struct(&mut self, struct_: Struct) -> String {
        match struct_.is_record {
            true => {
                // Add record symbol to composite types.
                let mut output_string = String::from("record");
                self.composite_mapping
                    .insert(struct_.identifier.name, (true, output_string.clone()));
                writeln!(output_string, " {}:", struct_.identifier).expect("failed to write to string"); // todo: check if this is safe from name conflicts.

                // Construct and append the record variables.
                for var in struct_.members.iter() {
                    let mode = match var.mode {
                        Mode::Constant => "constant",
                        Mode::Public => "public",
                        Mode::None | Mode::Private => "private",
                    };
                    writeln!(
                        output_string,
                        "    {} as {}.{mode};", // todo: CAUTION private record variables only.
                        var.identifier, var.type_
                    )
                    .expect("failed to write to string");
                }

                output_string
            }
            false => {
                // Add private symbol to composite types.
                self.composite_mapping
                    .insert(struct_.identifier.name, (false, String::from("private"))); // todo: private by default here.

                let mut output_string = format!("struct {}:\n", struct_.identifier); // todo: check if this is safe from name conflicts.

                // Construct and append the record variables.
                for var in struct_.members.iter() {
                    writeln!(output_string, "    {} as {};", var.identifier, var.type_,)
                        .expect("failed to write to string");
                }

                output_string
            }
        }
    }
}

impl ImportConsumer for CodeGenerator<'_> {
    type Output = String;

    // TODO: Fix once imports are redesigned .
    fn consume_import(&mut self, input: Program) -> Self::Output {
        // Get the name of the imported program.
        // Note that the unwrap is safe since parsing guarantees that there is exactly one program scope.
        let name = input.program_scopes.keys().next().unwrap().name;

        // Load symbols into composite mapping.
        let _ = self.consume_program(input);
        // todo: We do not need the import program string because we generate instructions for imports separately during leo build.

        // Generate string for import statement.
        format!("import {}.aleo;", name)
    }
}

impl MappingConsumer for CodeGenerator<'_> {
    type Output = String;

    fn consume_mapping(&mut self, mapping: Mapping) -> Self::Output {
        // Create the prefix of the mapping string, e.g. `mapping foo:`.
        let mut mapping_string = format!("mapping {}:\n", mapping.identifier);

        // Helper to construct the string associated with the type.
        let create_type = |type_: &Type| {
            match type_ {
                Type::Mapping(_) | Type::Tuple(_) => unreachable!("Mappings cannot contain mappings or tuples."),
                Type::Identifier(identifier) => {
                    // Lookup the type in the composite mapping.
                    // Note that this unwrap is safe since all struct and records have been added to the composite mapping.
                    let (is_record, _) = self.composite_mapping.get(&identifier.name).unwrap();
                    match is_record {
                        // If the type is a record, then declare the type as is.
                        true => format!("{identifier}.record"),
                        // If the type is a struct, then add the public modifier.
                        false => format!("{identifier}.public"),
                    }
                }
                type_ => format!("{type_}.public"),
            }
        };

        // Create the key string, e.g. `    key as address.public`.
        mapping_string.push_str(&format!("\tkey left as {};\n", create_type(&mapping.key_type)));

        // Create the value string, e.g. `    value as address.public`.
        mapping_string.push_str(&format!("\tvalue right as {};\n", create_type(&mapping.value_type)));

        mapping_string
    }
}

impl ProgramConsumer for CodeGenerator<'_> {
    type Output = String;

    fn consume_program(&mut self, input: Program) -> Self::Output {
        // Accumulate instructions into a program string.
        let mut program_string = String::new();

        if !input.imports.is_empty() {
            // Visit each import statement and produce a Aleo import instruction.
            program_string.push_str(
                &input
                    .imports
                    .into_values()
                    .map(|(imported_program, _)| self.consume_import(imported_program))
                    .join("\n"),
            );

            // Newline separator.
            program_string.push('\n');
        }

        // Retrieve the program scope.
        // Note that type checking guarantees that there is exactly one program scope.
        let mut program_scope: ProgramScope = input.program_scopes.into_values().next().unwrap();

        // Print the program id.
        writeln!(program_string, "program {};", program_scope.program_id)
            .expect("Failed to write program id to string.");

        // Newline separator.
        program_string.push('\n');

        // Get the post-order ordering of the composite data types.
        // Note that the unwrap is safe since type checking guarantees that the struct dependency graph is acyclic.
        let order = self.struct_graph.post_order().unwrap();

        // Visit each `Struct` or `Record` in the post-ordering and produce an Aleo struct or record.
        program_string.push_str(
            &order
                .into_iter()
                .map(|name| {
                    match program_scope.structs.remove(&name) {
                        // If the struct is found, it is a local struct.
                        Some(struct_) => self.consume_struct(struct_),
                        // If the struct is not found, it is an imported struct.
                        None => String::new(),
                    }
                })
                .join("\n"),
        );

        // Newline separator.
        program_string.push('\n');

        // Visit each mapping in the Leo AST and produce an Aleo mapping declaration.
        program_string.push_str(
            &program_scope
                .mappings
                .into_values()
                .map(|mapping| self.consume_mapping(mapping))
                .join("\n"),
        );

        // Visit each function in the program scope and produce an Aleo function.
        // Note that in the function inlining pass, we reorder the functions such that they are in post-order.
        // In other words, a callee function precedes its caller function in the program scope.
        program_string.push_str(
            &program_scope
                .functions
                .into_values()
                .map(|function| {
                    // Set the `is_transition_function` flag.
                    self.is_transition_function = matches!(function.variant, Variant::Transition);

                    let function_string = self.consume_function(function);

                    // Unset the `is_transition_function` flag.
                    self.is_transition_function = false;

                    function_string
                })
                .join("\n"),
        );

        program_string
    }
}
