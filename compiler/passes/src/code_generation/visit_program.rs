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

use leo_ast::{Composite, Function, Location, Mapping, Member, Mode, Program, ProgramScope, Type, Variant};
use leo_span::{sym, Symbol};

use indexmap::IndexMap;
use itertools::Itertools;
use std::fmt::Write as _;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn visit_program(&mut self, input: &'a Program) -> String {
        // Accumulate instructions into a program string.
        let mut program_string = String::new();

        // Print out the dependencies of the program. Already arranged in post order by Retriever module.
        input.stubs.iter().for_each(|(program_name, _)| {
            program_string.push_str(&format!("import {}.aleo;\n", program_name));
        });

        // Retrieve the program scope.
        // Note that type checking guarantees that there is exactly one program scope.
        let program_scope: &ProgramScope = input.program_scopes.values().next().unwrap();

        self.program_id = Some(program_scope.program_id);

        // Print the program id.
        writeln!(program_string, "program {};", program_scope.program_id)
            .expect("Failed to write program id to string.");

        // Newline separator.
        program_string.push('\n');

        // Get the post-order ordering of the composite data types.
        // Note that the unwrap is safe since type checking guarantees that the struct dependency graph is acyclic.
        let order = self.struct_graph.post_order().unwrap();

        // Create a mapping of symbols to references of structs so can perform constant-time lookups.
        let structs_map: IndexMap<Symbol, &Composite> = self
            .symbol_table
            .structs
            .iter()
            .filter_map(|(name, struct_)| {
                // Only include structs and local records.
                if !(struct_.is_record
                    && struct_.external.map(|program| program != self.program_id.unwrap().name.name).unwrap_or(false))
                {
                    Some((name.name, struct_))
                } else {
                    None
                }
            })
            .collect();

        // Visit each `Struct` or `Record` in the post-ordering and produce an Aleo struct or record.
        program_string.push_str(
            &order
                .into_iter()
                .map(|name| {
                    match structs_map.get(&name) {
                        // If the struct is found, it is a struct or external record.
                        Some(struct_) => self.visit_struct_or_record(struct_),
                        // If the struct is not found, it is an imported record.
                        None => String::new(),
                    }
                })
                .join("\n"),
        );

        // Newline separator.
        program_string.push('\n');

        // Visit each mapping in the Leo AST and produce an Aleo mapping declaration.
        program_string
            .push_str(&program_scope.mappings.iter().map(|(_, mapping)| self.visit_mapping(mapping)).join("\n"));

        // Visit each function in the program scope and produce an Aleo function.
        // Note that in the function inlining pass, we reorder the functions such that they are in post-order.
        // In other words, a callee function precedes its caller function in the program scope.
        program_string.push_str(
            &program_scope
                .functions
                .iter()
                .map(|(_, function)| {
                    if function.variant != Variant::AsyncFunction {
                        let mut function_string = self.visit_function(function);

                        // Attach the associated finalize to async transitions.
                        if function.variant == Variant::AsyncTransition {
                            // Set state variables.
                            self.finalize_caller = Some(function.identifier.name);
                            // Generate code for the associated finalize function.
                            let finalize = &self
                                .symbol_table
                                .lookup_fn_symbol(Location::new(
                                    Some(self.program_id.unwrap().name.name),
                                    function.identifier.name,
                                ))
                                .unwrap()
                                .clone()
                                .finalize
                                .unwrap()
                                .name;
                            // Write the finalize string.
                            function_string.push_str(&format!(
                                "{}\n",
                                &self.visit_function(
                                    &program_scope.functions.iter().find(|(name, _f)| name == finalize).unwrap().1
                                )
                            ));
                        }

                        function_string
                    } else {
                        String::new()
                    }
                })
                .join("\n"),
        );

        program_string
    }

    fn visit_struct_or_record(&mut self, struct_: &'a Composite) -> String {
        if struct_.is_record { self.visit_record(struct_) } else { self.visit_struct(struct_) }
    }

    fn visit_struct(&mut self, struct_: &'a Composite) -> String {
        // Add private symbol to composite types.
        self.composite_mapping.insert(&struct_.identifier.name, (false, String::from("private"))); // todo: private by default here.

        let mut output_string = format!("struct {}:\n", struct_.identifier); // todo: check if this is safe from name conflicts.

        // Construct and append the record variables.
        for var in struct_.members.iter() {
            writeln!(output_string, "    {} as {};", var.identifier, Self::visit_type(&var.type_),)
                .expect("failed to write to string");
        }

        output_string
    }

    fn visit_record(&mut self, record: &'a Composite) -> String {
        // Add record symbol to composite types.
        let mut output_string = String::from("record");
        self.composite_mapping.insert(&record.identifier.name, (true, output_string.clone()));
        writeln!(output_string, " {}:", record.identifier).expect("failed to write to string"); // todo: check if this is safe from name conflicts.

        let mut members = Vec::with_capacity(record.members.len());
        let mut member_map: IndexMap<Symbol, Member> =
            record.members.clone().into_iter().map(|member| (member.identifier.name, member)).collect();

        // Add the owner field to the beginning of the members list.
        // Note that type checking ensures that the owner field exists.
        members.push(member_map.shift_remove(&sym::owner).unwrap());

        // Add the remaining fields to the members list.
        members.extend(member_map.into_iter().map(|(_, member)| member));

        // Construct and append the record variables.
        for var in members.iter() {
            let mode = match var.mode {
                Mode::Constant => "constant",
                Mode::Public => "public",
                Mode::None | Mode::Private => "private",
            };
            writeln!(
                output_string,
                "    {} as {}.{mode};", // todo: CAUTION private record variables only.
                var.identifier,
                Self::visit_type(&var.type_)
            )
            .expect("failed to write to string");
        }

        output_string
    }

    fn visit_function(&mut self, function: &'a Function) -> String {
        // Initialize the state of `self` with the appropriate values before visiting `function`.
        self.next_register = 0;
        self.variable_mapping = IndexMap::new();
        self.variant = Some(function.variant);
        // TODO: Figure out a better way to initialize.
        self.variable_mapping.insert(&sym::SelfLower, "self".to_string());
        self.variable_mapping.insert(&sym::block, "block".to_string());
        self.variable_mapping.insert(&sym::network, "network".to_string());
        self.current_function = Some(function);

        // Construct the header of the function.
        // If a function is a program function, generate an Aleo `function`,
        // if it is a standard function generate an Aleo `closure`,
        // otherwise, it is an inline function, in which case a function should not be generated.
        let mut function_string = match function.variant {
            Variant::Transition | Variant::AsyncTransition => format!("\nfunction {}:\n", function.identifier),
            Variant::Function => format!("\nclosure {}:\n", function.identifier),
            Variant::AsyncFunction => format!("\nfinalize {}:\n", self.finalize_caller.unwrap()),
            Variant::Inline => return String::from("\n"),
        };

        // Construct and append the input declarations of the function.
        let mut futures = self
            .symbol_table
            .lookup_fn_symbol(Location::new(Some(self.program_id.unwrap().name.name), function.identifier.name))
            .unwrap()
            .future_inputs
            .clone();
        for input in function.input.iter() {
            let register_string = format!("r{}", self.next_register);
            self.next_register += 1;

            let type_string = {
                self.variable_mapping.insert(&input.identifier.name, register_string.clone());
                // Note that this unwrap is safe because we set the variant at the beginning of the function.
                let visibility = match (self.variant.unwrap(), input.mode) {
                    (Variant::AsyncTransition, Mode::None) | (Variant::Transition, Mode::None) => Mode::Private,
                    (Variant::AsyncFunction, Mode::None) => Mode::Public,
                    _ => input.mode,
                };
                // Futures are displayed differently in the input section. `input r0 as foo.aleo/bar.future;`
                if matches!(input.type_, Type::Future(_)) {
                    let location = futures.remove(0);
                    format!("{}.aleo/{}.future", location.program.unwrap(), location.name)
                } else {
                    self.visit_type_with_visibility(&input.type_, visibility)
                }
            };

            writeln!(function_string, "    input {register_string} as {type_string};",)
                .expect("failed to write to string");
        }

        //  Construct and append the function body.
        let block_string = self.visit_block(&function.block);
        function_string.push_str(&block_string);

        function_string
    }

    fn visit_mapping(&mut self, mapping: &'a Mapping) -> String {
        // Create the prefix of the mapping string, e.g. `mapping foo:`.
        let mut mapping_string = format!("\nmapping {}:\n", mapping.identifier);

        // Helper to construct the string associated with the type.
        let create_type = |type_: &Type| {
            match type_ {
                Type::Mapping(_) | Type::Tuple(_) => unreachable!("Mappings cannot contain mappings or tuples."),
                Type::Identifier(identifier) => {
                    // Lookup the type in the composite mapping.
                    // Note that this unwrap is safe since all struct and records have been added to the composite mapping.
                    let (is_record, _) = self.composite_mapping.get(&identifier.name).unwrap();
                    match is_record {
                        // If the type is a struct, then add the public modifier.
                        false => self.visit_type_with_visibility(type_, Mode::Public),
                        true => unreachable!("Type checking guarantees that mappings cannot contain records."),
                    }
                }
                type_ => self.visit_type_with_visibility(type_, Mode::Public),
            }
        };

        // Create the key string, e.g. `    key as address.public`.
        mapping_string.push_str(&format!("\tkey as {};\n", create_type(&mapping.key_type)));

        // Create the value string, e.g. `    value as address.public`.
        mapping_string.push_str(&format!("\tvalue as {};\n", create_type(&mapping.value_type)));

        // Add the mapping to the variable mapping.
        self.global_mapping.insert(&mapping.identifier.name, mapping.identifier.to_string());

        mapping_string
    }
}
