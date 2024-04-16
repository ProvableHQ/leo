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

use crate::{DiGraphError, Location, TypeChecker};

use leo_ast::*;
use leo_errors::TypeCheckerError;
use leo_span::sym;

use snarkvm::console::network::{Network, Testnet3};

use std::collections::HashSet;

// TODO: Cleanup logic for tuples.

impl<'a> ProgramVisitor<'a> for TypeChecker<'a> {
    fn visit_program(&mut self, input: &'a Program) {
        // Typecheck the program's stubs.
        input.stubs.iter().for_each(|(symbol, stub)| {
            if symbol != &stub.stub_id.name.name {
                self.emit_err(TypeCheckerError::stub_name_mismatch(
                    symbol,
                    stub.stub_id.name,
                    stub.stub_id.network.span,
                ));
            }
            self.visit_stub(stub)
        });
        self.is_stub = false;

        // Typecheck the program scopes.
        input.program_scopes.values().for_each(|scope| self.visit_program_scope(scope));
    }

    fn visit_stub(&mut self, input: &'a Stub) {
        // Set the current program name.
        self.program_name = Some(input.stub_id.name.name);

        // Cannot have constant declarations in stubs.
        if !input.consts.is_empty() {
            self.emit_err(TypeCheckerError::stubs_cannot_have_const_declarations(input.consts.first().unwrap().1.span));
        }

        // Typecheck the program's structs.
        input.structs.iter().for_each(|(_, function)| self.visit_struct_stub(function));

        // Typecheck the program's functions.
        input.functions.iter().for_each(|(_, function)| self.visit_function_stub(function));
    }

    fn visit_function_stub(&mut self, input: &'a FunctionStub) {
        // Must not be an inline function
        if input.variant == Variant::Inline {
            self.emit_err(TypeCheckerError::stub_functions_must_not_be_inlines(input.span));
        }

        // Lookup function metadata in the symbol table.
        // Note that this unwrap is safe since function metadata is stored in a prior pass.
        let function_index = self
            .symbol_table
            .borrow()
            .lookup_fn_symbol(Location::new(self.program_name, input.identifier.name))
            .unwrap()
            .id;

        // Enter the function's scope.
        self.enter_scope(function_index);

        // Create a new child scope for the function's parameters and body.
        let scope_index = self.create_child_scope();

        // Query helper function to type check function parameters and outputs.
        self.check_function_signature(&Function::from(input.clone()));

        // Exit the scope for the function's parameters and body.
        self.exit_scope(scope_index);

        // Check that the finalize scope is valid
        if input.finalize_stub.is_some() {
            // Create a new child scope for the finalize block.
            let scope_index = self.create_child_scope();

            // Check the finalize signature.
            let function = &Function::from(input.clone());
            self.check_finalize_signature(function.finalize.as_ref().unwrap(), function);

            // Exit the scope for the finalize block.
            self.exit_scope(scope_index);
        }

        // Exit the function's scope.
        self.exit_scope(function_index);
    }

    fn visit_struct_stub(&mut self, input: &'a Composite) {
        self.visit_struct(input);
    }

    fn visit_program_scope(&mut self, input: &'a ProgramScope) {
        // Set the current program name.
        self.program_name = Some(input.program_id.name.name);

        // Typecheck each const definition, and append to symbol table.
        input.consts.iter().for_each(|(_, c)| self.visit_const(c));

        // Typecheck each struct definition.
        input.structs.iter().for_each(|(_, function)| self.visit_struct(function));

        // Check that the struct dependency graph does not have any cycles.
        if let Err(DiGraphError::CycleDetected(path)) = self.struct_graph.post_order() {
            self.emit_err(TypeCheckerError::cyclic_struct_dependency(path));
        }

        // Typecheck each mapping definition.
        let mut mapping_count = 0;
        for (_, mapping) in input.mappings.iter() {
            self.visit_mapping(mapping);
            mapping_count += 1;
        }

        // Check that the number of mappings does not exceed the maximum.
        if mapping_count > Testnet3::MAX_MAPPINGS {
            self.emit_err(TypeCheckerError::too_many_mappings(
                Testnet3::MAX_MAPPINGS,
                input.program_id.name.span + input.program_id.network.span,
            ));
        }

        // Typecheck each function definitions.
        let mut transition_count = 0;
        for (_, function) in input.functions.iter() {
            self.visit_function(function);
            if matches!(function.variant, Variant::Transition) {
                transition_count += 1;
            }
        }

        // Check that the call graph does not have any cycles.
        if let Err(DiGraphError::CycleDetected(path)) = self.call_graph.post_order() {
            self.emit_err(TypeCheckerError::cyclic_function_dependency(path));
        }

        // TODO: Need similar checks for structs (all in separate PR)
        // Check that the number of transitions does not exceed the maximum.
        if transition_count > Testnet3::MAX_FUNCTIONS {
            self.emit_err(TypeCheckerError::too_many_transitions(
                Testnet3::MAX_FUNCTIONS,
                input.program_id.name.span + input.program_id.network.span,
            ));
        }
        // Check that each program has at least one transition function.
        // This is a snarkvm requirement.
        else if transition_count == 0 {
            self.emit_err(TypeCheckerError::no_transitions(input.program_id.name.span + input.program_id.network.span));
        }
    }

    fn visit_struct(&mut self, input: &'a Composite) {
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
            let check_has_field =
                |need, expected_ty: Type| match input.members.iter().find_map(|Member { identifier, type_, .. }| {
                    (identifier.name == need).then_some((identifier, type_))
                }) {
                    Some((_, actual_ty)) if expected_ty.eq_flat(actual_ty) => {} // All good, found + right type!
                    Some((field, _)) => {
                        self.emit_err(TypeCheckerError::record_var_wrong_type(field, expected_ty, input.span()));
                    }
                    None => {
                        self.emit_err(TypeCheckerError::required_record_variable(need, expected_ty, input.span()));
                    }
                };
            check_has_field(sym::owner, Type::Address);
        }

        if !(input.is_record && self.is_stub) {
            for Member { mode, identifier, type_, span, .. } in input.members.iter() {
                // Check that the member type is not a tuple.
                if matches!(type_, Type::Tuple(_)) {
                    self.emit_err(TypeCheckerError::composite_data_type_cannot_contain_tuple(
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
                    self.struct_graph.add_edge(input.identifier.name, struct_member_type.id.name);
                } else if let Type::Array(array_type) = type_ {
                    // Get the base element type.
                    let base_element_type = array_type.base_element_type();
                    // If the base element type is a struct, then add it to the struct dependency graph.
                    if let Type::Composite(member_type) = base_element_type {
                        self.struct_graph.add_edge(input.identifier.name, member_type.id.name);
                    }
                }

                // If the input is a struct, then check that the member does not have a mode.
                if !input.is_record && !matches!(mode, Mode::None) {
                    self.emit_err(TypeCheckerError::struct_cannot_have_member_mode(*span));
                }
            }
        }
    }

    fn visit_mapping(&mut self, input: &'a Mapping) {
        // Check that a mapping's key type is valid.
        self.assert_type_is_valid(&input.key_type, input.span);
        // Check that a mapping's key type is not a tuple, record, or mapping.
        match input.key_type.clone() {
            Type::Tuple(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("key", "tuple", input.span)),
            Type::Composite(struct_type) => {
                if let Some(struct_) = self.lookup_struct(struct_type.program, struct_type.id.name) {
                    if struct_.is_record {
                        self.emit_err(TypeCheckerError::invalid_mapping_type("key", "record", input.span));
                    }
                }
            }
            // Note that this is not possible since the parser does not currently accept mapping types.
            Type::Mapping(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("key", "mapping", input.span)),
            _ => {}
        }

        // Check that a mapping's value type is valid.
        self.assert_type_is_valid(&input.value_type, input.span);
        // Check that a mapping's value type is not a tuple, record or mapping.
        match input.value_type.clone() {
            Type::Tuple(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("value", "tuple", input.span)),
            Type::Composite(struct_type) => {
                if let Some(struct_) = self.lookup_struct(struct_type.program, struct_type.id.name) {
                    if struct_.is_record {
                        self.emit_err(TypeCheckerError::invalid_mapping_type("value", "record", input.span));
                    }
                }
            }
            // Note that this is not possible since the parser does not currently accept mapping types.
            Type::Mapping(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("value", "mapping", input.span)),
            _ => {}
        }
    }

    fn visit_function(&mut self, function: &'a Function) {
        // Check that the function's annotations are valid.
        // Note that Leo does not natively support any specific annotations.
        for annotation in function.annotations.iter() {
            // TODO: Change to compiler warning.
            self.emit_err(TypeCheckerError::unknown_annotation(annotation, annotation.span))
        }

        self.variant = Some(function.variant);

        // Lookup function metadata in the symbol table.
        // Note that this unwrap is safe since function metadata is stored in a prior pass.
        let function_index = self
            .symbol_table
            .borrow()
            .lookup_fn_symbol(Location::new(self.program_name, function.identifier.name))
            .unwrap()
            .id;

        // Enter the function's scope.
        self.enter_scope(function_index);

        // The function's body does not have a return statement.
        self.has_return = false;

        // The function's body does not have a finalize statement.
        self.has_finalize = false;

        // Store the name of the function.
        self.function = Some(function.name());

        // Create a new child scope for the function's parameters and body.
        let scope_index = self.create_child_scope();

        // Query helper function to type check function parameters and outputs.
        self.check_function_signature(function);

        self.visit_block(&function.block);

        // If the function has a return type, then check that it has a return.
        if function.output_type != Type::Unit && !self.has_return {
            self.emit_err(TypeCheckerError::missing_return(function.span));
        }

        // If the function has a finalize block, then check that it has at least one finalize statement.
        if function.finalize.is_some() && !self.has_finalize {
            self.emit_err(TypeCheckerError::missing_finalize(function.span));
        }

        // Exit the scope for the function's parameters and body.
        self.exit_scope(scope_index);

        // Traverse and check the finalize block if it exists.
        if let Some(finalize) = &function.finalize {
            self.is_finalize = true;
            // The function's finalize block does not have a return statement.
            self.has_return = false;
            // The function's finalize block does not have a finalize statement.
            self.has_finalize = false;

            // Create a new child scope for the finalize block.
            let scope_index = self.create_child_scope();

            // Check the finalize signature.
            self.check_finalize_signature(finalize, function);

            // TODO: Remove if this restriction is relaxed at Aleo instructions level
            // Check that the finalize block is not empty.
            if finalize.block.statements.is_empty() {
                self.emit_err(TypeCheckerError::finalize_block_must_not_be_empty(finalize.span));
            }

            // Type check the finalize block.
            self.visit_block(&finalize.block);

            // If the function has a return type, then check that it has a return.
            if finalize.output_type != Type::Unit && !self.has_return {
                self.emit_err(TypeCheckerError::missing_return(finalize.span));
            }

            // Exit the scope for the finalize block.
            self.exit_scope(scope_index);

            self.is_finalize = false;
        }

        // Exit the function's scope.
        self.exit_scope(function_index);

        // Unset the `variant`.
        self.variant = None;
    }
}
