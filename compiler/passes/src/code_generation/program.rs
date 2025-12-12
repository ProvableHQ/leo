// Copyright (C) 2019-2026 Provable Inc.
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
};
use leo_span::{Symbol, sym};

use indexmap::IndexMap;
use itertools::Itertools;
use snarkvm::prelude::{CanaryV0, MainnetV0, TestnetV0};

impl<'a> CodeGeneratingVisitor<'a> {
    pub fn visit_program(&mut self, input: &'a Program) -> AleoProgram {
        // Dependencies of the program. Already arranged in post order by Retriever module.

        let imports = input.stubs.iter().map(|(program_name, _)| program_name.to_string()).collect();

        // Retrieve the program scope.
        // Note that type checking guarantees that there is exactly one program scope.
        let program_scope: &ProgramScope = input.program_scopes.values().next().unwrap();

        let program_id = program_scope.program_id;
        self.program_id = Some(program_id);

        // Get the post-order ordering of the composite data types.
        // Note that the unwrap is safe since type checking guarantees that the composite dependency graph is acyclic.
        let order = self.state.composite_graph.post_order().unwrap();

        let this_program = self.program_id.unwrap().name.name;

        let lookup = |loc: &Location| {
            if loc.program == this_program {
                self.state
                    .symbol_table
                    .lookup_struct(this_program, loc)
                    .or_else(|| self.state.symbol_table.lookup_record(this_program, loc))
            } else {
                None
            }
        };

        // Add each `struct` or `record` in the post-ordering and produce an Aleo struct or record.
        let data_types = order
            .into_iter()
            .filter_map(|loc| lookup(&loc).map(|composite| self.visit_struct_or_record(composite, &loc)))
            .collect();

        // Visit each mapping in the Leo AST and produce an Aleo mapping declaration.
        let mappings = program_scope.mappings.iter().map(|(_symbol, mapping)| self.visit_mapping(mapping)).collect();

        // Visit each function in the program scope and produce an Aleo function.
        // Note that in the function inlining pass, we reorder the functions such that they are in post-order.
        // In other words, a callee function precedes its caller function in the program scope.
        let functions = program_scope
            .functions
            .iter()
            .filter_map(|(_symbol, function)| {
                if function.variant != Variant::AsyncFunction {
                    let mut aleo_function = self.visit_function(function);

                    // Attach the associated finalize to async transitions.
                    if function.variant == Variant::AsyncTransition {
                        // Set state variables.
                        self.finalize_caller = Some(function.identifier.name);
                        // Generate code for the associated finalize function.
                        let finalize = &self
                            .state
                            .symbol_table
                            .lookup_function(
                                this_program,
                                &Location::new(
                                    this_program,
                                    vec![function.identifier.name], // Guaranteed to live in program scope, not in any submodule
                                ),
                            )
                            .unwrap()
                            .clone()
                            .finalizer
                            .unwrap();
                        // Write the finalize string.
                        if let Some(caller) = &mut aleo_function {
                            caller.as_function_ref_mut().finalize = Some(
                                self.visit_function_with(
                                    &program_scope
                                        .functions
                                        .iter()
                                        .find(|(name, _f)| vec![*name] == finalize.location.path)
                                        .unwrap()
                                        .1,
                                    &finalize.future_inputs,
                                )
                                .unwrap()
                                .as_finalize(),
                            );
                        }
                    }
                    aleo_function
                } else {
                    None
                }
            })
            .collect();

        // If the constructor exists, visit it and produce an Aleo constructor.
        let constructor = program_scope.constructor.as_ref().map(|c| self.visit_constructor(c));

        AleoProgram { imports, program_id, data_types, mappings, functions, constructor }
    }

    fn visit_struct_or_record(&mut self, composite: &'a Composite, loc: &Location) -> AleoDatatype {
        if composite.is_record {
            AleoDatatype::Record(self.visit_record(composite, loc))
        } else {
            AleoDatatype::Struct(self.visit_struct(composite, loc))
        }
    }

    fn visit_struct(&mut self, struct_: &'a Composite, loc: &Location) -> AleoStruct {
        // Add private symbol to composite types.
        self.composite_mapping.insert(loc.clone(), false); // todo: private by default here.

        // todo: check if this is safe from name conflicts.
        let name = Self::legalize_path(&loc.path)
            .unwrap_or_else(|| panic!("path format cannot be legalized at this point: {}", loc.path.iter().join("::")));

        // Construct and append the record variables.
        let fields = struct_
            .members
            .iter()
            .filter_map(|var| {
                if var.type_.is_empty() {
                    None
                } else {
                    Some((var.identifier.to_string(), self.visit_type(&var.type_)))
                }
            })
            .collect();

        AleoStruct { name, fields }
    }

    fn visit_record(&mut self, record: &'a Composite, loc: &Location) -> AleoRecord {
        // Add record symbol to composite types.
        self.composite_mapping.insert(loc.clone(), true);

        let name = record.identifier.to_string(); // todo: check if this is safe from name conflicts.

        let mut members = Vec::with_capacity(record.members.len());
        let mut member_map: IndexMap<Symbol, Member> =
            record.members.clone().into_iter().map(|member| (member.identifier.name, member)).collect();

        // Add the owner field to the beginning of the members list.
        // Note that type checking ensures that the owner field exists.
        members.push(member_map.shift_remove(&sym::owner).unwrap());

        // Add the remaining fields to the members list.
        members.extend(member_map.into_iter().map(|(_, member)| member));

        // Construct and append the record variables.
        let fields = members
            .iter()
            .filter_map(|var| {
                if var.type_.is_empty() {
                    None
                } else {
                    Some((var.identifier.to_string(), self.visit_type(&var.type_), match var.mode {
                        Mode::Constant => AleoVisibility::Constant,
                        Mode::Public => AleoVisibility::Public,
                        Mode::None | Mode::Private => AleoVisibility::Private,
                    }))
                }
            })
            .collect();

        AleoRecord { name, fields }
    }

    fn visit_function_with(&mut self, function: &'a Function, futures: &[Location]) -> Option<AleoFunctional> {
        // Initialize the state of `self` with the appropriate values before visiting `function`.
        self.next_register = 0;
        self.variable_mapping = IndexMap::new();
        self.variant = Some(function.variant);
        // TODO: Figure out a better way to initialize.
        self.variable_mapping.insert(sym::SelfLower, AleoExpr::Reg(AleoReg::Self_));
        self.variable_mapping.insert(sym::block, AleoExpr::Reg(AleoReg::Block));
        self.variable_mapping.insert(sym::network, AleoExpr::Reg(AleoReg::Network));
        self.current_function = Some(function);

        // Construct the header of the function.
        // If a function is a program function, generate an Aleo `function`,
        // if it is a standard function generate an Aleo `closure`,
        // otherwise, it is an inline function, in which case a function should not be generated.
        let function_name = match function.variant {
            Variant::Inline => return None,
            Variant::Script => panic!("script should not appear in native code"),
            Variant::Transition | Variant::AsyncTransition => function.identifier.to_string(),
            Variant::Function => function.identifier.to_string(),
            Variant::AsyncFunction => self.finalize_caller.unwrap().to_string(),
        };

        let mut futures = futures.iter();

        self.internal_record_inputs.clear();

        // Construct and append the input declarations of the function.
        let inputs: Vec<AleoInput> = function
            .input
            .iter()
            .filter_map(|input| {
                if input.type_.is_empty() {
                    return None;
                }
                let register_num = self.next_register();
                let current_program = self.program_id.unwrap().name.name;

                // Track all internal record inputs.
                if let Type::Composite(comp) = &input.type_ {
                    let composite_location = comp.path.expect_global_location();
                    if self.state.symbol_table.lookup_record(current_program, composite_location).is_some()
                        && (composite_location.program == current_program)
                    {
                        self.internal_record_inputs.insert(AleoExpr::Reg(register_num.clone()));
                    }
                }

                let (input_type, input_visibility) = {
                    self.variable_mapping.insert(input.identifier.name, AleoExpr::Reg(register_num.clone()));
                    // Note that this unwrap is safe because we set the variant at the beginning of the function.
                    let visibility = match (self.variant.unwrap(), input.mode) {
                        (Variant::AsyncTransition, Mode::None) | (Variant::Transition, Mode::None) => {
                            Some(AleoVisibility::Private)
                        }
                        (Variant::AsyncFunction, Mode::None) => Some(AleoVisibility::Public),
                        (_, mode) => AleoVisibility::maybe_from(mode),
                    };
                    // Futures are displayed differently in the input section. `input r0 as foo.aleo/bar.future;`
                    if matches!(input.type_, Type::Future(_)) {
                        let location = futures
                            .next()
                            .expect("Type checking guarantees we have future locations for each future input");
                        let [future_name] = location.path.as_slice() else {
                            panic!(
                                "All futures must have a single segment paths since they don't belong to submodules."
                            )
                        };
                        (
                            AleoType::Future { name: future_name.to_string(), program: location.program.to_string() },
                            None,
                        )
                    } else {
                        self.visit_type_with_visibility(&input.type_, visibility)
                    }
                };

                Some(AleoInput { register: register_num, type_: input_type, visibility: input_visibility })
            })
            .collect();

        //  Construct and append the function body.
        let mut statements = self.visit_block(&function.block);
        if matches!(self.variant.unwrap(), Variant::Function | Variant::AsyncFunction)
            && statements.iter().all(|stm| matches!(stm, AleoStmt::Output(..)))
        {
            // There are no real instructions, which is invalid in Aleo, so
            // add a dummy instruction.
            statements.insert(0, AleoStmt::AssertEq(AleoExpr::Bool(true), AleoExpr::Bool(true)));
        }

        match function.variant {
            Variant::Inline | Variant::Script => None,
            Variant::Transition | Variant::AsyncTransition => {
                Some(AleoFunctional::Function(AleoFunction { name: function_name, inputs, statements, finalize: None })) // finalize added by caller
            }
            Variant::Function => Some(AleoFunctional::Closure(AleoClosure { name: function_name, inputs, statements })),
            Variant::AsyncFunction => {
                Some(AleoFunctional::Finalize(AleoFinalize { caller_name: function_name, inputs, statements }))
            }
        }
    }

    fn visit_function(&mut self, function: &'a Function) -> Option<AleoFunctional> {
        self.visit_function_with(function, &[])
    }

    fn visit_constructor(&mut self, constructor: &'a Constructor) -> AleoConstructor {
        // Initialize the state of `self` with the appropriate values before visiting `constructor`.
        self.next_register = 0;
        self.variable_mapping = IndexMap::new();
        self.variant = Some(Variant::AsyncFunction);
        // TODO: Figure out a better way to initialize.
        self.variable_mapping.insert(sym::SelfLower, AleoExpr::Reg(AleoReg::Self_));
        self.variable_mapping.insert(sym::block, AleoExpr::Reg(AleoReg::Block));
        self.variable_mapping.insert(sym::network, AleoExpr::Reg(AleoReg::Network));

        // Get the upgrade variant.
        let upgrade_variant = constructor
            .get_upgrade_variant_with_network(self.state.network)
            .expect("Type checking should have validated the upgrade variant");

        // Construct the constructor.
        // If the constructor is one of the standard constructors, use the hardcoded defaults.
        let constructor = match &upgrade_variant {
            // This is the expected snarkVM constructor bytecode for a program that is only upgradable by a fixed admin.
            UpgradeVariant::Admin { address } => AleoConstructor {
                statements: vec![AleoStmt::AssertEq(
                    AleoExpr::RawName("program_owner".to_string()),
                    AleoExpr::RawName(address.to_string()),
                )],
            },

            UpgradeVariant::Checksum { mapping, key, .. } => {
                let map_name = if mapping.program
                    == self.program_id.expect("Program ID should be set before traversing the program").name.name
                {
                    let [mapping_name] = &mapping.path[..] else {
                        panic!("Mappings are only allowed in the top level program at this stage");
                    };
                    mapping_name.to_string()
                } else {
                    mapping.to_string()
                };
                // This is the required snarkVM constructor bytecode for a program that is only upgradable
                // if the new program's checksum matches the one declared in a pre-determined mapping.
                AleoConstructor {
                    statements: vec![
                        AleoStmt::BranchEq(
                            AleoExpr::RawName("edition".to_string()),
                            AleoExpr::U16(0),
                            "end".to_string(),
                        ),
                        AleoStmt::Get(AleoExpr::RawName(map_name), AleoExpr::RawName(key.to_string()), AleoReg::R(0)),
                        AleoStmt::AssertEq(AleoExpr::RawName("checksum".to_string()), AleoExpr::Reg(AleoReg::R(0))),
                        AleoStmt::Position("end".to_string()),
                    ],
                }
            }
            UpgradeVariant::Custom => AleoConstructor { statements: self.visit_block(&constructor.block) },
            UpgradeVariant::NoUpgrade => {
                // This is the expected snarkVM constructor bytecode for a program that is not upgradable.
                AleoConstructor {
                    statements: vec![AleoStmt::AssertEq(AleoExpr::RawName("edition".to_string()), AleoExpr::U16(0))],
                }
            }
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

    fn visit_mapping(&mut self, mapping: &'a Mapping) -> AleoMapping {
        let legalized_mapping_name = Self::legalize_path(&[mapping.identifier.name]);
        // Create the prefix of the mapping string, e.g. `mapping foo:`.
        let name = legalized_mapping_name
            .clone()
            .unwrap_or_else(|| panic!("path format cannot be legalized at this point: {}", mapping.identifier));

        // Helper to construct the string associated with the type.
        let create_type = |type_: &Type| {
            match type_ {
                Type::Mapping(_) | Type::Tuple(_) => panic!("Mappings cannot contain mappings or tuples."),
                Type::Identifier(identifier) => {
                    // Lookup the type in the composite mapping.
                    // Note that this unwrap is safe since all struct and records have been added to the composite mapping.
                    let is_record = self
                        .composite_mapping
                        .get(&Location::new(self.program_id.unwrap().name.name, vec![identifier.name]))
                        .unwrap();
                    assert!(!is_record, "Type checking guarantees that mappings cannot contain records.");
                    self.visit_type_with_visibility(type_, Some(AleoVisibility::Public))
                }
                type_ => self.visit_type_with_visibility(type_, Some(AleoVisibility::Public)),
            }
        };

        // Create the key string, e.g. `    key as address.public`.
        let (key_type, key_visibility) = create_type(&mapping.key_type);

        // Create the value string, e.g. `    value as address.public`.
        let (value_type, value_visibility) = create_type(&mapping.value_type);

        // Add the mapping to the variable mapping.
        self.global_mapping.insert(
            mapping.identifier.name,
            AleoExpr::RawName(
                legalized_mapping_name
                    .unwrap_or_else(|| panic!("path format cannot be legalized at this point: {}", mapping.identifier)),
            ),
        );

        AleoMapping { name, key_type, value_type, key_visibility, value_visibility }
    }
}
