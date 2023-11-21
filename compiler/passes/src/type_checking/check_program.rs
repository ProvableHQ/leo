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

use crate::{DiGraphError, TypeChecker, VariableSymbol, VariableType};

use leo_ast::*;
use leo_errors::TypeCheckerError;
use leo_span::sym;

use snarkvm::console::network::{Network, Testnet3};

use std::collections::HashSet;

// TODO: Cleanup logic for tuples.

impl<'a> ProgramVisitor<'a> for TypeChecker<'a> {
    fn visit_program(&mut self, input: &'a Program) {
        match self.is_imported {
            // If the program is imported, then it is not allowed to import any other programs.
            true => {
                input.imports.values().for_each(|(_, span)| {
                    self.emit_err(TypeCheckerError::imported_program_cannot_import_program(*span))
                });
            }
            // Otherwise, typecheck the imported programs.
            false => {
                // Set `self.is_imported`.
                let previous_is_imported = core::mem::replace(&mut self.is_imported, true);

                // Typecheck the imported programs.
                input.imports.values().for_each(|import| self.visit_import(&import.0));

                // Set `self.is_imported` to its previous state.
                self.is_imported = previous_is_imported;
            }
        }

        // Typecheck the program scopes.
        input.program_scopes.values().for_each(|scope| self.visit_program_scope(scope));
    }

    fn visit_program_scope(&mut self, input: &'a ProgramScope) {
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
    }

    fn visit_struct(&mut self, input: &'a Struct) {
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
            if let Type::Identifier(member_type) = type_ {
                self.struct_graph.add_edge(input.identifier.name, member_type.name);
            } else if let Type::Array(array_type) = type_ {
                // Get the base element type.
                let base_element_type = array_type.base_element_type();
                // If the base element type is a struct, then add it to the struct dependency graph.
                if let Type::Identifier(member_type) = base_element_type {
                    self.struct_graph.add_edge(input.identifier.name, member_type.name);
                }
            }

            // If the input is a struct, then check that the member does not have a mode.
            if !input.is_record && !matches!(mode, Mode::None) {
                self.emit_err(TypeCheckerError::struct_cannot_have_member_mode(*span));
            }
        }
    }

    fn visit_mapping(&mut self, input: &'a Mapping) {
        // Check that a mapping's key type is valid.
        self.assert_type_is_valid(&input.key_type, input.span);
        // Check that a mapping's key type is not a tuple, record, or mapping.
        match input.key_type {
            Type::Tuple(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("key", "tuple", input.span)),
            Type::Identifier(identifier) => {
                if let Some(struct_) = self.symbol_table.borrow().lookup_struct(identifier.name) {
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
        match input.value_type {
            Type::Tuple(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("value", "tuple", input.span)),
            Type::Identifier(identifier) => {
                if let Some(struct_) = self.symbol_table.borrow().lookup_struct(identifier.name) {
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
        let function_index = self.symbol_table.borrow().lookup_fn_symbol(function.identifier.name).unwrap().id;

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

        // Type check the function's parameters.
        function.input.iter().for_each(|input_var| {
            // Check that the type of input parameter is defined.
            self.assert_type_is_valid(&input_var.type_(), input_var.span());
            // Check that the type of the input parameter is not a tuple.
            if matches!(input_var.type_(), Type::Tuple(_)) {
                self.emit_err(TypeCheckerError::function_cannot_take_tuple_as_input(input_var.span()))
            }

            // Note that this unwrap is safe since we assign to `self.variant` above.
            match self.variant.unwrap() {
                // If the function is a transition function, then check that the parameter mode is not a constant.
                Variant::Transition if input_var.mode() == Mode::Constant => {
                    self.emit_err(TypeCheckerError::transition_function_inputs_cannot_be_const(input_var.span()))
                }
                // If the function is not a transition function, then check that the parameters do not have an associated mode.
                Variant::Standard | Variant::Inline if input_var.mode() != Mode::None => {
                    self.emit_err(TypeCheckerError::regular_function_inputs_cannot_have_modes(input_var.span()))
                }
                _ => {} // Do nothing.
            }

            // Check for conflicting variable names.
            if let Err(err) =
                self.symbol_table.borrow_mut().insert_variable(input_var.identifier().name, VariableSymbol {
                    type_: input_var.type_(),
                    span: input_var.identifier().span(),
                    declaration: VariableType::Input(input_var.mode()),
                })
            {
                self.handler.emit_err(err);
            }
        });

        // Type check the function's return type.
        // Note that checking that each of the component types are defined is sufficient to check that `output_type` is defined.
        function.output.iter().for_each(|output| {
            match output {
                Output::External(external) => {
                    // If the function is not a transition function, then it cannot output a record.
                    // Note that an external output must always be a record.
                    if !matches!(function.variant, Variant::Transition) {
                        self.emit_err(TypeCheckerError::function_cannot_output_record(external.span()));
                    }
                    // Otherwise, do not type check external record function outputs.
                    // TODO: Verify that this is not needed when the import system is updated.
                }
                Output::Internal(function_output) => {
                    // Check that the type of output is defined.
                    if self.assert_type_is_valid(&function_output.type_, function_output.span) {
                        // If the function is not a transition function, then it cannot output a record.
                        if let Type::Identifier(identifier) = function_output.type_ {
                            if !matches!(function.variant, Variant::Transition)
                                && self.symbol_table.borrow().lookup_struct(identifier.name).unwrap().is_record
                            {
                                self.emit_err(TypeCheckerError::function_cannot_output_record(function_output.span));
                            }
                        }
                    }
                    // Check that the type of the output is not a tuple. This is necessary to forbid nested tuples.
                    if matches!(&function_output.type_, Type::Tuple(_)) {
                        self.emit_err(TypeCheckerError::nested_tuple_type(function_output.span))
                    }
                    // Check that the mode of the output is valid.
                    // For functions, only public and private outputs are allowed
                    if function_output.mode == Mode::Constant {
                        self.emit_err(TypeCheckerError::cannot_have_constant_output_mode(function_output.span));
                    }
                }
            }
        });

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
            // The function;s finalize block does not have a finalize statement.
            self.has_finalize = false;

            // Check that the function is a transition function.
            if !matches!(function.variant, Variant::Transition) {
                self.emit_err(TypeCheckerError::only_transition_functions_can_have_finalize(finalize.span));
            }

            // Check that the name of the finalize block matches the function name.
            if function.identifier.name != finalize.identifier.name {
                self.emit_err(TypeCheckerError::finalize_name_mismatch(
                    function.identifier.name,
                    finalize.identifier.name,
                    finalize.span,
                ));
            }

            // Create a new child scope for the finalize block.
            let scope_index = self.create_child_scope();

            finalize.input.iter().for_each(|input_var| {
                // Check that the type of input parameter is defined.
                if self.assert_type_is_valid(&input_var.type_(), input_var.span()) {
                    // Check that the input parameter is not a tuple.
                    if matches!(input_var.type_(), Type::Tuple(_)) {
                        self.emit_err(TypeCheckerError::finalize_cannot_take_tuple_as_input(input_var.span()))
                    }
                    // Check that the input parameter is not a record.
                    if let Type::Identifier(identifier) = input_var.type_() {
                        // Note that this unwrap is safe, as the type is defined.
                        if self.symbol_table.borrow().lookup_struct(identifier.name).unwrap().is_record {
                            self.emit_err(TypeCheckerError::finalize_cannot_take_record_as_input(input_var.span()))
                        }
                    }
                    // Check that the input parameter is not constant or private.
                    if input_var.mode() == Mode::Constant || input_var.mode() == Mode::Private {
                        self.emit_err(TypeCheckerError::finalize_input_mode_must_be_public(input_var.span()));
                    }
                    // Check for conflicting variable names.
                    if let Err(err) =
                        self.symbol_table.borrow_mut().insert_variable(input_var.identifier().name, VariableSymbol {
                            type_: input_var.type_(),
                            span: input_var.identifier().span(),
                            declaration: VariableType::Input(input_var.mode()),
                        })
                    {
                        self.handler.emit_err(err);
                    }
                }
            });

            // Check that the finalize block's return type is a unit type.
            // Note: This is a temporary restriction to be compatible with the current version of snarkVM.
            // Note: This restriction may be lifted in the future.
            // Note: This check is still compatible with the other checks below.
            if finalize.output_type != Type::Unit {
                self.emit_err(TypeCheckerError::finalize_cannot_return_value(finalize.span));
            }

            // Type check the finalize block's return type.
            // Note that checking that each of the component types are defined is sufficient to guarantee that the `output_type` is defined.
            finalize.output.iter().for_each(|output_type| {
                // Check that the type of output is defined.
                if self.assert_type_is_valid(&output_type.type_(), output_type.span()) {
                    // Check that the output is not a tuple. This is necessary to forbid nested tuples.
                    if matches!(&output_type.type_(), Type::Tuple(_)) {
                        self.emit_err(TypeCheckerError::nested_tuple_type(output_type.span()))
                    }
                    // Check that the output is not a record.
                    if let Type::Identifier(identifier) = output_type.type_() {
                        // Note that this unwrap is safe, as the type is defined.
                        if self.symbol_table.borrow().lookup_struct(identifier.name).unwrap().is_record {
                            self.emit_err(TypeCheckerError::finalize_cannot_output_record(output_type.span()))
                        }
                    }
                    // Check that the mode of the output is valid.
                    // Note that a finalize block can have only public outputs.
                    if matches!(output_type.mode(), Mode::Constant | Mode::Private) {
                        self.emit_err(TypeCheckerError::finalize_output_mode_must_be_public(output_type.span()));
                    }
                }
            });

            // TODO: Remove if this restriction is relaxed at Aleo instructions level.
            // Check that the finalize block is not empty.
            if finalize.block.statements.is_empty() {
                self.emit_err(TypeCheckerError::finalize_block_must_not_be_empty(finalize.span));
            }

            // Type check the finalize block.
            self.visit_block(&finalize.block);

            // Check that the return type is defined. Note that the component types are already checked.
            self.assert_type_is_valid(&finalize.output_type, finalize.span);

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
