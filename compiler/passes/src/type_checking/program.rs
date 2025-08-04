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

use super::TypeCheckingVisitor;
use crate::{VariableSymbol, VariableType};

use leo_ast::{DiGraphError, Type, *};
use leo_errors::TypeCheckerError;
use leo_span::{Symbol, sym};

use itertools::Itertools;
use snarkvm::prelude::{CanaryV0, MainnetV0, TestnetV0};
use std::collections::{BTreeMap, HashSet};

impl ProgramVisitor for TypeCheckingVisitor<'_> {
    fn visit_program(&mut self, input: &Program) {
        // Typecheck the program's stubs.
        input.stubs.iter().for_each(|(symbol, stub)| {
            // Check that naming and ordering is consistent.
            if symbol != &stub.stub_id.name.name {
                self.emit_err(TypeCheckerError::stub_name_mismatch(
                    symbol,
                    stub.stub_id.name,
                    stub.stub_id.network.span,
                ));
            }
            self.visit_stub(stub)
        });
        self.scope_state.is_stub = false;

        // Typecheck the program scopes.
        input.program_scopes.values().for_each(|scope| self.visit_program_scope(scope));
    }

    fn visit_program_scope(&mut self, input: &ProgramScope) {
        let program_name = input.program_id.name;

        // Ensure that the program name is legal, i.e., it does not contain the keyword `aleo`
        check_name(&self.state.handler, program_name, "program");

        // Set the current program name.
        self.scope_state.program_name = Some(program_name.name);

        // Collect a map from record names to their spans
        let record_info: BTreeMap<String, leo_span::Span> = input
            .structs
            .iter()
            .filter(|(_, c)| c.is_record)
            .map(|(_, r)| (r.name().to_string(), r.identifier.span))
            .collect();

        // Check if any record name is a prefix for another record name. We don't really collect all possible prefixes
        // here but only adjacent ones. That is, if we have records `Foo`, `FooBar`, and `FooBarBaz`, we only emit
        // errors for `Foo/FooBar` and for `FooBar/FooBarBaz` but not for `Foo/FooBarBaz`.
        for ((prev_name, _), (curr_name, curr_span)) in record_info.iter().tuple_windows() {
            if curr_name.starts_with(prev_name) {
                self.state
                    .handler
                    .emit_err(TypeCheckerError::record_prefixed_by_other_record(curr_name, prev_name, *curr_span));
            }
        }

        // Typecheck each const definition, and append to symbol table.
        input.consts.iter().for_each(|(_, c)| self.visit_const(c));

        // Typecheck each struct definition.
        input.structs.iter().for_each(|(_, function)| self.visit_struct(function));

        // Check that the struct dependency graph does not have any cycles.
        if let Err(DiGraphError::CycleDetected(path)) = self.state.struct_graph.post_order() {
            self.emit_err(TypeCheckerError::cyclic_struct_dependency(path));
        }

        // Typecheck each mapping definition.
        let mut mapping_count = 0;
        for (_, mapping) in input.mappings.iter() {
            self.visit_mapping(mapping);
            mapping_count += 1;
        }

        // Check that the number of mappings does not exceed the maximum.
        if mapping_count > self.limits.max_mappings {
            self.emit_err(TypeCheckerError::too_many_mappings(
                self.limits.max_mappings,
                input.program_id.name.span + input.program_id.network.span,
            ));
        }

        // Typecheck each function definitions.
        let mut transition_count = 0;
        for (_, function) in input.functions.iter() {
            self.visit_function(function);
            if function.variant.is_transition() {
                transition_count += 1;
            }
        }

        // Typecheck the constructor.
        // Note: Constructors are required for all **new** programs once they are supported in the AVM.
        //  However, we do not require them to exist to ensure backwards compatibility with existing programs.
        if let Some(constructor) = &input.constructor {
            self.visit_constructor(constructor);
        }

        // Check that the call graph does not have any cycles.
        if let Err(DiGraphError::CycleDetected(path)) = self.state.call_graph.post_order() {
            self.emit_err(TypeCheckerError::cyclic_function_dependency(path));
        }

        // TODO: Need similar checks for structs (all in separate PR)
        // Check that the number of transitions does not exceed the maximum.
        if transition_count > self.limits.max_functions {
            self.emit_err(TypeCheckerError::too_many_transitions(
                self.limits.max_functions,
                input.program_id.name.span + input.program_id.network.span,
            ));
        }
        // Check that each program has at least one transition function.
        // This is a snarkvm requirement.
        else if transition_count == 0 {
            self.emit_err(TypeCheckerError::no_transitions(input.program_id.name.span + input.program_id.network.span));
        }
    }

    fn visit_stub(&mut self, input: &Stub) {
        // Set the scope state.
        self.scope_state.program_name = Some(input.stub_id.name.name);
        self.scope_state.is_stub = true;

        // Cannot have constant declarations in stubs.
        if !input.consts.is_empty() {
            self.emit_err(TypeCheckerError::stubs_cannot_have_const_declarations(input.consts.first().unwrap().1.span));
        }

        // Typecheck the program's structs.
        input.structs.iter().for_each(|(_, function)| self.visit_struct_stub(function));

        // Typecheck the program's functions.
        input.functions.iter().for_each(|(_, function)| self.visit_function_stub(function));
    }

    fn visit_struct(&mut self, input: &Composite) {
        self.in_conditional_scope(|slf| {
            slf.in_scope(input.id, |slf| {
                if input.is_record && !input.const_parameters.is_empty() {
                    slf.emit_err(TypeCheckerError::unexpected_record_const_parameters(input.span));
                } else {
                    input
                        .const_parameters
                        .iter()
                        .for_each(|const_param| slf.insert_symbol_conditional_scope(const_param.identifier.name));

                    for const_param in &input.const_parameters {
                        slf.visit_type(const_param.type_());

                        // Restrictions for const parameters
                        if !matches!(
                            const_param.type_(),
                            Type::Boolean | Type::Integer(_) | Type::Address | Type::Scalar | Type::Group | Type::Field
                        ) {
                            slf.emit_err(TypeCheckerError::bad_const_generic_type(
                                const_param.type_(),
                                const_param.span(),
                            ));
                        }

                        // Add the input to the symbol table.
                        if let Err(err) = slf.state.symbol_table.insert_variable(
                            slf.scope_state.program_name.unwrap(),
                            const_param.identifier().name,
                            VariableSymbol {
                                type_: const_param.type_().clone(),
                                span: const_param.identifier.span(),
                                declaration: VariableType::ConstParameter,
                            },
                        ) {
                            slf.state.handler.emit_err(err);
                        }

                        // Add the input to the type table.
                        slf.state.type_table.insert(const_param.identifier().id(), const_param.type_().clone());
                    }
                }

                input.members.iter().for_each(|member| slf.visit_type(&member.type_));
            })
        });

        // Check for conflicting struct/record member names.
        let mut used = HashSet::new();
        // TODO: Better span to target duplicate member.
        if !input.members.iter().all(|Member { identifier, type_, span, .. }| {
            // Check that the member types are defined.
            self.assert_type_is_valid(type_, *span);
            used.insert(identifier.name)
        }) {
            self.emit_err(if input.is_record {
                TypeCheckerError::duplicate_record_variable(input.name(), input.span())
            } else {
                TypeCheckerError::duplicate_struct_member(input.name(), input.span())
            });
        }

        // For records, enforce presence of the `owner: Address` member.
        if input.is_record {
            // Ensure that the record name is legal, i.e., it does not contain the keyword `aleo`
            check_name(&self.state.handler, input.identifier, "record");

            // Ensure that the names of the record entries are all legal, i.e., they do not contain the
            // keyword `aleo`
            input.members.iter().for_each(|member| {
                check_name(&self.state.handler, member.identifier, "record entry");
            });

            let check_has_field =
                |need, expected_ty: Type| match input.members.iter().find_map(|Member { identifier, type_, .. }| {
                    (identifier.name == need).then_some((identifier, type_))
                }) {
                    Some((_, actual_ty)) if expected_ty.eq_flat_relaxed(actual_ty) => {} // All good, found + right type!
                    Some((field, _)) => {
                        self.emit_err(TypeCheckerError::record_var_wrong_type(field, expected_ty, input.span()));
                    }
                    None => {
                        self.emit_err(TypeCheckerError::required_record_variable(need, expected_ty, input.span()));
                    }
                };
            check_has_field(sym::owner, Type::Address);
        }
        // For structs, check that there is at least one member.
        else if input.members.is_empty() {
            self.emit_err(TypeCheckerError::empty_struct(input.span()));
        }

        if !(input.is_record && self.scope_state.is_stub) {
            for Member { mode, identifier, type_, span, .. } in input.members.iter() {
                // Check that the member type is not a tuple.
                if matches!(type_, Type::Tuple(_)) {
                    self.emit_err(TypeCheckerError::composite_data_type_cannot_contain_tuple(
                        if input.is_record { "record" } else { "struct" },
                        identifier.span,
                    ));
                } else if matches!(type_, Type::Future(..)) {
                    self.emit_err(TypeCheckerError::composite_data_type_cannot_contain_future(
                        if input.is_record { "record" } else { "struct" },
                        identifier.span,
                    ));
                }

                // Ensure that there are no record members.
                self.assert_member_is_not_record(identifier.span, input.identifier.name, type_);
                // If the member is a struct, add it to the struct dependency graph.
                // Note that we have already checked that each member is defined and valid.
                if let Type::Composite(struct_member_type) = type_ {
                    // Note that since there are no cycles in the program dependency graph, there are no cycles in the struct dependency graph caused by external structs.
                    self.state.struct_graph.add_edge(input.identifier.name, struct_member_type.id.name);
                } else if let Type::Array(array_type) = type_ {
                    // Get the base element type.
                    let base_element_type = array_type.base_element_type();
                    // If the base element type is a struct, then add it to the struct dependency graph.
                    if let Type::Composite(member_type) = base_element_type {
                        self.state.struct_graph.add_edge(input.identifier.name, member_type.id.name);
                    }
                }

                // If the input is a struct, then check that the member does not have a mode.
                if !input.is_record && !matches!(mode, Mode::None) {
                    self.emit_err(TypeCheckerError::struct_cannot_have_member_mode(*span));
                }
            }
        }
    }

    fn visit_mapping(&mut self, input: &Mapping) {
        self.visit_type(&input.key_type);
        self.visit_type(&input.value_type);

        // Check that a mapping's key type is valid.
        self.assert_type_is_valid(&input.key_type, input.span);
        // Check that a mapping's key type is not a future, tuple, record, or mapping.
        match input.key_type.clone() {
            Type::Future(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("key", "future", input.span)),
            Type::Tuple(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("key", "tuple", input.span)),
            Type::Composite(struct_type) => {
                if let Some(comp) =
                    self.lookup_struct(struct_type.program.or(self.scope_state.program_name), struct_type.id.name)
                {
                    if comp.is_record {
                        self.emit_err(TypeCheckerError::invalid_mapping_type("key", "record", input.span));
                    }
                } else {
                    self.emit_err(TypeCheckerError::undefined_type(&input.key_type, input.span));
                }
            }
            // Note that this is not possible since the parser does not currently accept mapping types.
            Type::Mapping(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("key", "mapping", input.span)),
            _ => {}
        }

        // Check that a mapping's value type is valid.
        self.assert_type_is_valid(&input.value_type, input.span);
        // Check that a mapping's value type is not a future, tuple, record or mapping.
        match input.value_type.clone() {
            Type::Future(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("value", "future", input.span)),
            Type::Tuple(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("value", "tuple", input.span)),
            Type::Composite(struct_type) => {
                if let Some(comp) =
                    self.lookup_struct(struct_type.program.or(self.scope_state.program_name), struct_type.id.name)
                {
                    if comp.is_record {
                        self.emit_err(TypeCheckerError::invalid_mapping_type("value", "record", input.span));
                    }
                } else {
                    self.emit_err(TypeCheckerError::undefined_type(&input.key_type, input.span));
                }
            }
            // Note that this is not possible since the parser does not currently accept mapping types.
            Type::Mapping(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("value", "mapping", input.span)),
            _ => {}
        }
    }

    fn visit_function(&mut self, function: &Function) {
        // Reset the scope state.
        self.scope_state.reset();

        // Set the scope state before traversing the function.
        self.scope_state.variant = Some(function.variant);

        // Check that the function's annotations are valid.
        for annotation in function.annotations.iter() {
            if !matches!(annotation.identifier.name, sym::test | sym::should_fail) {
                self.emit_err(TypeCheckerError::unknown_annotation(annotation, annotation.span))
            }
        }

        let get = |symbol: Symbol| -> &Annotation {
            function.annotations.iter().find(|ann| ann.identifier.name == symbol).unwrap()
        };

        let check_annotation = |symbol: Symbol, allowed_keys: &[Symbol]| -> bool {
            let count = function.annotations.iter().filter(|ann| ann.identifier.name == symbol).count();
            if count > 0 {
                let annotation = get(symbol);
                for key in annotation.map.keys() {
                    if !allowed_keys.contains(key) {
                        self.emit_err(TypeCheckerError::annotation_error(
                            format_args!("Invalid key `{key}` for annotation @{symbol}"),
                            annotation.span,
                        ));
                    }
                }
                if count > 1 {
                    self.emit_err(TypeCheckerError::annotation_error(
                        format_args!("Duplicate annotation @{symbol}"),
                        annotation.span,
                    ));
                }
            }
            count > 0
        };

        let has_test = check_annotation(sym::test, &[sym::private_key]);
        let has_should_fail = check_annotation(sym::should_fail, &[]);

        if has_test && !self.state.is_test {
            self.emit_err(TypeCheckerError::annotation_error(
                format_args!("Test annotation @test appears outside of tests"),
                get(sym::test).span,
            ));
        }

        if has_should_fail && !self.state.is_test {
            self.emit_err(TypeCheckerError::annotation_error(
                format_args!("Test annotation @should_fail appears outside of tests"),
                get(sym::should_fail).span,
            ));
        }

        if has_should_fail && !has_test {
            self.emit_err(TypeCheckerError::annotation_error(
                format_args!("Annotation @should_fail appears without @test"),
                get(sym::should_fail).span,
            ));
        }

        if has_test
            && !self.scope_state.variant.unwrap().is_script()
            && !self.scope_state.variant.unwrap().is_transition()
        {
            self.emit_err(TypeCheckerError::annotation_error(
                format_args!("Annotation @test may appear only on scripts and transitions"),
                get(sym::test).span,
            ));
        }

        if (has_test) && !function.input.is_empty() {
            self.emit_err(TypeCheckerError::annotation_error(
                "A test procedure cannot have inputs",
                function.input[0].span,
            ));
        }

        self.in_conditional_scope(|slf| {
            slf.in_scope(function.id, |slf| {
                function
                    .const_parameters
                    .iter()
                    .for_each(|const_param| slf.insert_symbol_conditional_scope(const_param.identifier.name));

                function.input.iter().for_each(|input| slf.insert_symbol_conditional_scope(input.identifier.name));

                // Store the name of the function.
                slf.scope_state.function = Some(function.name());

                // Query helper function to type check function parameters and outputs.
                slf.check_function_signature(function, false);

                if function.variant == Variant::Function && function.input.is_empty() {
                    slf.emit_err(TypeCheckerError::empty_function_arglist(function.span));
                }

                slf.visit_block(&function.block);

                // If the function has a return type, then check that it has a return.
                if function.output_type != Type::Unit && !slf.scope_state.has_return {
                    slf.emit_err(TypeCheckerError::missing_return(function.span));
                }
            })
        });

        // Make sure that async transitions call finalize.
        if self.scope_state.variant == Some(Variant::AsyncTransition)
            && !self.scope_state.has_called_finalize
            && !self.scope_state.already_contains_an_async_block
        {
            self.emit_err(TypeCheckerError::missing_async_operation_in_async_transition(function.span));
        }
    }

    fn visit_constructor(&mut self, constructor: &Constructor) {
        // Reset the scope state.
        self.scope_state.reset();
        // Set the scope state before traversing the constructor.
        self.scope_state.function = Some(sym::constructor);
        // Note: We set the variant to `AsyncFunction` since constructors have similar semantics.
        self.scope_state.variant = Some(Variant::AsyncFunction);
        self.scope_state.is_constructor = true;

        // Get the upgrade variant.
        // Note, `get_upgrade_variant` will return an error if the constructor is not well-formed.
        let result = match self.state.network {
            NetworkName::CanaryV0 => constructor.get_upgrade_variant::<CanaryV0>(),
            NetworkName::TestnetV0 => constructor.get_upgrade_variant::<TestnetV0>(),
            NetworkName::MainnetV0 => constructor.get_upgrade_variant::<MainnetV0>(),
        };
        let upgrade_variant = match result {
            Ok(upgrade_variant) => upgrade_variant,
            Err(e) => {
                self.emit_err(TypeCheckerError::custom(e, constructor.span));
                return;
            }
        };

        // Validate the number of statements.
        match (&upgrade_variant, constructor.block.statements.is_empty()) {
            (UpgradeVariant::Custom, true) => {
                self.emit_err(TypeCheckerError::custom("A 'custom' constructor cannot be empty", constructor.span));
            }
            (UpgradeVariant::NoUpgrade | UpgradeVariant::Admin { .. } | UpgradeVariant::Checksum { .. }, false) => {
                self.emit_err(TypeCheckerError::custom("A 'noupgrade', 'admin', or 'checksum' constructor must be empty. The Leo compiler will insert the appropriate code.", constructor.span));
            }
            _ => {}
        }

        // For the checksum variant, check that the mapping exists and that the type matches.
        if let UpgradeVariant::Checksum { mapping, key, key_type } = &upgrade_variant {
            // Look up the mapping type.
            let Some(VariableSymbol { type_: Type::Mapping(mapping_type), .. }) =
                self.state.symbol_table.lookup_global(*mapping)
            else {
                self.emit_err(TypeCheckerError::custom(
                    format!("The mapping '{mapping}' does not exist. Please ensure that it is imported or defined in your program."),
                    constructor.annotations[0].span,
                ));
                return;
            };
            // Check that the mapping key type matches the expected key type.
            if *mapping_type.key != *key_type {
                self.emit_err(TypeCheckerError::custom(
                    format!(
                        "The mapping '{}' key type '{}' does not match the key '{}' in the `@checksum` annotation",
                        mapping, mapping_type.key, key
                    ),
                    constructor.annotations[0].span,
                ));
            }
            // Check that the value type is a `[u8; 32]`.
            let check_value_type = |type_: &Type| -> bool {
                if let Type::Array(array_type) = type_ {
                    if !matches!(array_type.element_type.as_ref(), &Type::Integer(_)) {
                        return false;
                    }
                    if let Some(length) = array_type.length.as_u32() {
                        return length == 32;
                    }
                    return false;
                }
                false
            };
            if !check_value_type(&mapping_type.value) {
                self.emit_err(TypeCheckerError::custom(
                    format!("The mapping '{}' value type '{}' must be a '[u8; 32]'", mapping, mapping_type.value),
                    constructor.annotations[0].span,
                ));
            }
        }

        // Traverse the constructor.
        self.in_conditional_scope(|slf| {
            slf.in_scope(constructor.id, |slf| {
                slf.visit_block(&constructor.block);
            })
        });

        // Check that the constructor does not call `finalize`.
        if self.scope_state.has_called_finalize {
            self.emit_err(TypeCheckerError::custom("The constructor cannot call `finalize`.", constructor.span));
        }

        // Check that the constructor does not have an `async` block.
        if self.scope_state.already_contains_an_async_block {
            self.emit_err(TypeCheckerError::custom("The constructor cannot have an `async` block.", constructor.span));
        }
    }

    fn visit_function_stub(&mut self, input: &FunctionStub) {
        // Must not be an inline function
        if input.variant == Variant::Inline {
            self.emit_err(TypeCheckerError::stub_functions_must_not_be_inlines(input.span));
        }

        // Create future stubs.
        if input.variant == Variant::AsyncFunction {
            let finalize_input_map = &mut self.async_function_input_types;
            let resolved_inputs: Vec<Type> = input
                .input
                .iter()
                .map(|input| {
                    match &input.type_ {
                        Type::Future(f) => {
                            // Since we traverse stubs in post-order, we can assume that the corresponding finalize stub has already been traversed.
                            Type::Future(FutureType::new(
                                finalize_input_map.get(f.location.as_ref().unwrap()).unwrap().clone(),
                                f.location,
                                true,
                            ))
                        }
                        _ => input.clone().type_,
                    }
                })
                .collect();

            finalize_input_map
                .insert(Location::new(self.scope_state.program_name.unwrap(), input.identifier.name), resolved_inputs);
        }

        // Query helper function to type check function parameters and outputs.
        self.check_function_signature(&Function::from(input.clone()), /* is_stub */ true);
    }

    fn visit_struct_stub(&mut self, input: &Composite) {
        self.visit_struct(input);
    }
}

fn check_name(handler: &leo_errors::Handler, name: Identifier, item_type: &str) {
    if name.to_string().contains(&sym::aleo.to_string()) {
        handler.emit_err(TypeCheckerError::illegal_name(name, item_type, sym::aleo, name.span));
    }
}
