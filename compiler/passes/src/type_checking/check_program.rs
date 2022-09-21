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

use crate::{TypeChecker, VariableSymbol, VariableType};

use leo_ast::*;
use leo_errors::TypeCheckerError;

use leo_span::sym;

use std::collections::HashSet;

// TODO: Generally, cleanup tyc logic.

impl<'a> ProgramVisitor<'a> for TypeChecker<'a> {
    fn visit_circuit(&mut self, input: &'a Circuit) {
        // Check for conflicting circuit/record member names.
        let mut used = HashSet::new();
        if !input
            .members
            .iter()
            .all(|CircuitMember::CircuitVariable(ident, type_)| {
                // TODO: Better spans.
                // Check that the member types are valid.
                self.assert_type_is_valid(input.span, type_);
                used.insert(ident.name)
            })
        {
            self.emit_err(if input.is_record {
                TypeCheckerError::duplicate_record_variable(input.name(), input.span())
            } else {
                TypeCheckerError::duplicate_circuit_member(input.name(), input.span())
            });
        }

        // For records, enforce presence of `owner: Address` and `gates: u64` members.
        if input.is_record {
            let check_has_field = |need, expected_ty: Type| match input
                .members
                .iter()
                .find_map(|CircuitMember::CircuitVariable(v, t)| (v.name == need).then_some((v, t)))
            {
                Some((_, actual_ty)) if expected_ty.eq_flat(actual_ty) => {} // All good, found + right type!
                Some((field, _)) => {
                    self.emit_err(TypeCheckerError::record_var_wrong_type(
                        field,
                        expected_ty,
                        input.span(),
                    ));
                }
                None => {
                    self.emit_err(TypeCheckerError::required_record_variable(
                        need,
                        expected_ty,
                        input.span(),
                    ));
                }
            };
            check_has_field(sym::owner, Type::Address);
            check_has_field(sym::gates, Type::Integer(IntegerType::U64));
        }

        for CircuitMember::CircuitVariable(v, type_) in input.members.iter() {
            // Ensure there are no tuple typed members.
            self.assert_not_tuple(v.span, type_);
            // Ensure that there are no record members.
            self.assert_member_is_not_record(v.span, input.identifier.name, type_);
        }
    }

    fn visit_mapping(&mut self, input: &'a Mapping) {
        // Check that a mapping's key type is valid.
        self.assert_type_is_valid(input.span, &input.key_type);
        // Check that a mapping's key type is not tuple types or mapping types.
        match input.key_type {
            Type::Tuple(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("key", "tuple", input.span)),
            // Note that this is not possible since the parser does not currently accept mapping types.
            Type::Mapping(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("key", "mapping", input.span)),
            _ => {}
        }

        // Check that a mapping's value type is valid.
        self.assert_type_is_valid(input.span, &input.value_type);
        // Check that a mapping's value type is not tuple types or mapping types.
        match input.value_type {
            Type::Tuple(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("value", "tuple", input.span)),
            // Note that this is not possible since the parser does not currently accept mapping types.
            Type::Mapping(_) => self.emit_err(TypeCheckerError::invalid_mapping_type("value", "mapping", input.span)),
            _ => {}
        }
    }

    fn visit_function(&mut self, function: &'a Function) {
        // Check that the function's annotations are valid.
        for annotation in function.annotations.iter() {
            match annotation.identifier.name {
                // Set `is_program_function` to true if the corresponding annotation is found.
                sym::program => self.is_program_function = true,
                _ => self.emit_err(TypeCheckerError::unknown_annotation(annotation, annotation.span)),
            }
        }

        // Lookup function metadata in the symbol table.
        // Note that this unwrap is safe since function metadata is stored in a prior pass.
        let function_index = self
            .symbol_table
            .borrow()
            .lookup_fn_symbol(function.identifier.name)
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

        // Type check the function's parameters.
        function.input.iter().for_each(|input_var| {
            // Check that the type of input parameter is valid.
            self.assert_type_is_valid(input_var.span(), &input_var.type_());
            self.assert_not_tuple(input_var.span(), &input_var.type_());

            match self.is_program_function {
                // If the function is a program function, then check that the parameter mode is not a constant.
                true if input_var.mode() == Mode::Const => self.emit_err(
                    TypeCheckerError::program_function_inputs_cannot_be_const(input_var.span()),
                ),
                // If the function is not a program function, then check that the parameters do not have an associated mode.
                false if input_var.mode() != Mode::None => self.emit_err(
                    TypeCheckerError::helper_function_inputs_cannot_have_modes(input_var.span()),
                ),
                _ => {} // Do nothing.
            }

            // Check for conflicting variable names.
            if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
                input_var.identifier().name,
                VariableSymbol {
                    type_: input_var.type_(),
                    span: input_var.identifier().span(),
                    declaration: VariableType::Input(input_var.mode()),
                },
            ) {
                self.handler.emit_err(err);
            }
        });

        // Type check the function's return type.
        function.output.iter().for_each(|output_type| {
            match output_type {
                Output::External(_) => {} // Do not type check external record function outputs.
                Output::Internal(output_type) => {
                    // Check that the type of output is valid.
                    self.assert_type_is_valid(output_type.span, &output_type.type_);

                    // Check that the mode of the output is valid.
                    if output_type.mode == Mode::Const {
                        self.emit_err(TypeCheckerError::cannot_have_constant_output_mode(output_type.span));
                    }
                }
            }
        });

        self.visit_block(&function.block);

        // Check that the return type is valid.
        self.assert_type_is_valid(function.span, &function.output_type);

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

            // Check that the function is a program function.
            if !self.is_program_function {
                self.emit_err(TypeCheckerError::only_program_functions_can_have_finalize(
                    finalize.span,
                ));
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
                // Check that the type of input parameter is valid.
                self.assert_type_is_valid(input_var.span(), &input_var.type_());
                self.assert_not_tuple(input_var.span(), &input_var.type_());

                // Check that the input parameter is not constant or private.
                if input_var.mode() == Mode::Const || input_var.mode() == Mode::Private {
                    self.emit_err(TypeCheckerError::finalize_input_mode_must_be_public(input_var.span()));
                }

                // Check for conflicting variable names.
                if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
                    input_var.identifier().name,
                    VariableSymbol {
                        type_: input_var.type_(),
                        span: input_var.identifier().span(),
                        declaration: VariableType::Input(input_var.mode()),
                    },
                ) {
                    self.handler.emit_err(err);
                }
            });

            // Type check the function's return type.
            finalize.output.iter().for_each(|output_type| {
                // Check that the type of output is valid.
                self.assert_type_is_valid(output_type.span(), &output_type.type_());

                // Check that the mode of the output is valid.
                if output_type.mode() == Mode::Const {
                    self.emit_err(TypeCheckerError::finalize_input_mode_must_be_public(output_type.span()));
                }
            });

            // TODO: Remove when this restriction is removed.
            // Check that the finalize block is not empty.
            if finalize.block.statements.is_empty() {
                self.emit_err(TypeCheckerError::finalize_block_must_not_be_empty(finalize.span));
            }

            // Type check the finalize block.
            self.visit_block(&finalize.block);

            // Check that the return type is valid.
            self.assert_type_is_valid(finalize.span, &finalize.output_type);

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

        // Unset `is_program_function` flag.
        self.is_program_function = false;
    }
}
