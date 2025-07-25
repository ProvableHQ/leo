// Copyright (C) 2019-2025 Provable Inc.
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

use leo_ast::{
    Composite,
    Constructor,
    Function,
    Location,
    Mapping,
    Member,
    Mode,
    NetworkName,
    Program,
    ProgramScope,
    Type,
    UpgradeVariant,
    Variant,
    snarkvm_admin_constructor,
    snarkvm_checksum_constructor,
    snarkvm_noupgrade_constructor,
};
use leo_span::{Symbol, sym};

use indexmap::IndexMap;
use itertools::Itertools;
use snarkvm::prelude::{CanaryV0, MainnetV0, TestnetV0};
use std::fmt::Write as _;

const EXPECT_STR: &str = "Failed to write code";

impl<'a> CodeGeneratingVisitor<'a> {
    pub fn visit_program(&mut self, input: &'a Program) -> String {
        // Accumulate instructions into a program string.
        let mut program_string = String::new();

        // Print out the dependencies of the program. Already arranged in post order by Retriever module.
        input.stubs.iter().for_each(|(program_name, _)| {
            writeln!(program_string, "import {program_name}.aleo;").expect(EXPECT_STR);
        });

        // Retrieve the program scope.
        // Note that type checking guarantees that there is exactly one program scope.
        let program_scope: &ProgramScope = input.program_scopes.values().next().unwrap();

        self.program_id = Some(program_scope.program_id);

        // Print the program id.
        writeln!(program_string, "program {};", program_scope.program_id).expect(EXPECT_STR);

        // Get the post-order ordering of the composite data types.
        // Note that the unwrap is safe since type checking guarantees that the struct dependency graph is acyclic.
        let order = self.state.struct_graph.post_order().unwrap();

        let this_program = self.program_id.unwrap().name.name;

        let lookup = |name: &[Symbol]| {
            self.state
                .symbol_table
                .lookup_struct(name)
                .or_else(|| self.state.symbol_table.lookup_record(&Location::new(this_program, name.to_vec())))
        };

        // Visit each `Struct` or `Record` in the post-ordering and produce an Aleo struct or record.
        for name in order.into_iter() {
            if let Some(struct_) = lookup(&name) {
                program_string.push_str(&self.visit_struct_or_record(struct_, &name));
            }
        }

        // Visit each mapping in the Leo AST and produce an Aleo mapping declaration.
        for (_symbol, mapping) in program_scope.mappings.iter() {
            program_string.push_str(&self.visit_mapping(mapping));
        }

        // Visit each function in the program scope and produce an Aleo function.
        // Note that in the function inlining pass, we reorder the functions such that they are in post-order.
        // In other words, a callee function precedes its caller function in the program scope.
        for (_symbol, function) in program_scope.functions.iter() {
            if function.variant != Variant::AsyncFunction {
                let mut function_string = self.visit_function(function);

                // Attach the associated finalize to async transitions.
                if function.variant == Variant::AsyncTransition {
                    // Set state variables.
                    self.finalize_caller = Some(function.identifier.name);
                    // Generate code for the associated finalize function.
                    let finalize = &self
                        .state
                        .symbol_table
                        .lookup_function(&Location::new(
                            self.program_id.unwrap().name.name,
                            vec![function.identifier.name], // Guaranteed to live in program scope, not in any submodule
                        ))
                        .unwrap()
                        .clone()
                        .finalizer
                        .unwrap();
                    // Write the finalize string.
                    function_string.push_str(
                        &self.visit_function_with(
                            &program_scope
                                .functions
                                .iter()
                                .find(|(name, _f)| vec![*name] == finalize.location.path)
                                .unwrap()
                                .1,
                            &finalize.future_inputs,
                        ),
                    );
                }

                program_string.push_str(&function_string);
            }
        }

        // If the constructor exists, visit it and produce an Aleo constructor.
        if let Some(constructor) = program_scope.constructor.as_ref() {
            // Generate code for the constructor.
            program_string.push_str(&self.visit_constructor(constructor));
        }

        program_string
    }

    fn visit_struct_or_record(&mut self, struct_: &'a Composite, absolute_path: &[Symbol]) -> String {
        if struct_.is_record {
            self.visit_record(struct_, absolute_path)
        } else {
            self.visit_struct(struct_, absolute_path)
        }
    }

    fn visit_struct(&mut self, struct_: &'a Composite, absolute_path: &[Symbol]) -> String {
        // Add private symbol to composite types.
        self.composite_mapping.insert(absolute_path.to_vec(), (false, String::from("private"))); // todo: private by default here.

        let mut output_string = format!(
            "\nstruct {}:\n",
            Self::legalize_path(absolute_path).unwrap_or_else(|| panic!(
                "path format cannot be legalized at this point: {}",
                absolute_path.iter().join("::")
            ))
        ); // todo: check if this is safe from name conflicts.

        // Construct and append the record variables.
        for var in struct_.members.iter() {
            writeln!(output_string, "    {} as {};", var.identifier, Self::visit_type(&var.type_),).expect(EXPECT_STR);
        }

        output_string
    }

    fn visit_record(&mut self, record: &'a Composite, absolute_path: &[Symbol]) -> String {
        // Add record symbol to composite types.
        self.composite_mapping.insert(absolute_path.to_vec(), (true, "record".into()));

        let mut output_string = format!("\nrecord {}:\n", record.identifier); // todo: check if this is safe from name conflicts.

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
            .expect(EXPECT_STR);
        }

        output_string
    }

    fn visit_function_with(&mut self, function: &'a Function, futures: &[Location]) -> String {
        // Initialize the state of `self` with the appropriate values before visiting `function`.
        self.next_register = 0;
        self.variable_mapping = IndexMap::new();
        self.variant = Some(function.variant);
        // TODO: Figure out a better way to initialize.
        self.variable_mapping.insert(sym::SelfLower, "self".to_string());
        self.variable_mapping.insert(sym::block, "block".to_string());
        self.variable_mapping.insert(sym::network, "network".to_string());
        self.current_function = Some(function);

        // Construct the header of the function.
        // If a function is a program function, generate an Aleo `function`,
        // if it is a standard function generate an Aleo `closure`,
        // otherwise, it is an inline function, in which case a function should not be generated.
        let mut function_string = match function.variant {
            Variant::Transition | Variant::AsyncTransition => format!("\nfunction {}:\n", function.identifier),
            Variant::Function => format!("\nclosure {}:\n", function.identifier),
            Variant::AsyncFunction => format!("\nfinalize {}:\n", self.finalize_caller.unwrap()),
            Variant::Inline => return String::new(),
            Variant::Script => panic!("script should not appear in native code"),
        };

        let mut futures = futures.iter();

        self.internal_record_inputs.clear();

        // Construct and append the input declarations of the function.
        for input in function.input.iter() {
            let register_string = self.next_register();

            // Track all internal record inputs.
            if let Type::Composite(comp) = &input.type_ {
                let program = comp.program.unwrap_or(self.program_id.unwrap().name.name);
                if let Some(record) =
                    self.state.symbol_table.lookup_record(&Location::new(program, comp.path.absolute_path().to_vec()))
                {
                    if record.external.is_none() || record.external == self.program_id.map(|id| id.name.name) {
                        self.internal_record_inputs.insert(register_string.clone());
                    }
                }
            }

            let type_string = {
                self.variable_mapping.insert(input.identifier.name, register_string.clone());
                // Note that this unwrap is safe because we set the variant at the beginning of the function.
                let visibility = match (self.variant.unwrap(), input.mode) {
                    (Variant::AsyncTransition, Mode::None) | (Variant::Transition, Mode::None) => Mode::Private,
                    (Variant::AsyncFunction, Mode::None) => Mode::Public,
                    _ => input.mode,
                };
                // Futures are displayed differently in the input section. `input r0 as foo.aleo/bar.future;`
                if matches!(input.type_, Type::Future(_)) {
                    let location = futures
                        .next()
                        .expect("Type checking guarantees we have future locations for each future input");
                    let [future_name] = location.path.as_slice() else {
                        panic!("All futures must have a single segment paths since they don't belong to submodules.")
                    };
                    format!("{}.aleo/{}.future", location.program, future_name)
                } else {
                    self.visit_type_with_visibility(&input.type_, visibility)
                }
            };

            writeln!(function_string, "    input {register_string} as {type_string};",).expect(EXPECT_STR);
        }

        //  Construct and append the function body.
        let block_string = self.visit_block(&function.block);
        if matches!(self.variant.unwrap(), Variant::Function | Variant::AsyncFunction)
            && block_string.lines().all(|line| line.starts_with("    output "))
        {
            // There are no real instructions, which is invalid in Aleo, so
            // add a dummy instruction.
            function_string.push_str("    assert.eq true true;\n");
        }

        function_string.push_str(&block_string);

        function_string
    }

    fn visit_function(&mut self, function: &'a Function) -> String {
        self.visit_function_with(function, &[])
    }

    fn visit_constructor(&mut self, constructor: &'a Constructor) -> String {
        // Initialize the state of `self` with the appropriate values before visiting `constructor`.
        self.next_register = 0;
        self.variable_mapping = IndexMap::new();
        self.variant = Some(Variant::AsyncFunction);
        // TODO: Figure out a better way to initialize.
        self.variable_mapping.insert(sym::SelfLower, "self".to_string());
        self.variable_mapping.insert(sym::block, "block".to_string());
        self.variable_mapping.insert(sym::network, "network".to_string());

        // Get the upgrade variant.
        let upgrade_variant = constructor
            .get_upgrade_variant_with_network(self.state.network)
            .expect("Type checking should have validated the upgrade variant");

        // Construct the constructor.
        // If the constructor is one of the standard constructors, use the hardcoded defaults.
        let constructor = match &upgrade_variant {
            UpgradeVariant::Admin { address } => snarkvm_admin_constructor(address),
            UpgradeVariant::Checksum { mapping, key, .. } => {
                if mapping.program
                    == self.program_id.expect("Program ID should be set before traversing the program").name.name
                {
                    let [mapping_name] = &mapping.path[..] else {
                        panic!("Mappings are only allowed in the top level program at this stage");
                    };
                    snarkvm_checksum_constructor(mapping_name, key)
                } else {
                    snarkvm_checksum_constructor(mapping, key)
                }
            }
            UpgradeVariant::Custom => format!("\nconstructor:\n{}\n", self.visit_block(&constructor.block)),
            UpgradeVariant::NoUpgrade => snarkvm_noupgrade_constructor(),
        };

        // Check that the constructor is well-formed.
        if let Err(e) = match self.state.network {
            NetworkName::MainnetV0 => check_snarkvm_constructor::<MainnetV0>(&constructor),
            NetworkName::TestnetV0 => check_snarkvm_constructor::<TestnetV0>(&constructor),
            NetworkName::CanaryV0 => check_snarkvm_constructor::<CanaryV0>(&constructor),
        } {
            panic!("Compilation produced an invalid constructor: {e}");
        };

        // Return the constructor string.
        constructor
    }

    fn visit_mapping(&mut self, mapping: &'a Mapping) -> String {
        // Create the prefix of the mapping string, e.g. `mapping foo:`.
        let mut mapping_string = format!("\nmapping {}:\n", mapping.identifier);

        // Helper to construct the string associated with the type.
        let create_type = |type_: &Type| {
            match type_ {
                Type::Mapping(_) | Type::Tuple(_) => panic!("Mappings cannot contain mappings or tuples."),
                Type::Identifier(identifier) => {
                    // Lookup the type in the composite mapping.
                    // Note that this unwrap is safe since all struct and records have been added to the composite mapping.
                    let (is_record, _) = self.composite_mapping.get(&vec![identifier.name]).unwrap();
                    assert!(!is_record, "Type checking guarantees that mappings cannot contain records.");
                    self.visit_type_with_visibility(type_, Mode::Public)
                }
                type_ => self.visit_type_with_visibility(type_, Mode::Public),
            }
        };

        // Create the key string, e.g. `    key as address.public`.
        writeln!(mapping_string, "    key as {};", create_type(&mapping.key_type)).expect(EXPECT_STR);

        // Create the value string, e.g. `    value as address.public`.
        writeln!(mapping_string, "    value as {};", create_type(&mapping.value_type)).expect(EXPECT_STR);

        // Add the mapping to the variable mapping.
        self.global_mapping.insert(mapping.identifier.name, mapping.identifier.to_string());

        mapping_string
    }
}
