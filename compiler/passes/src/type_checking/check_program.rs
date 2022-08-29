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
                .find_map(|CircuitMember::CircuitVariable(v, t)| (v.name == need).then(|| (v, t)))
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

        function.input.iter().for_each(|input_var| {
            // Check that the type of input parameter is valid.
            self.assert_type_is_valid(input_var.span, &input_var.type_);
            self.assert_not_tuple(input_var.span, &input_var.type_);

            // If the function is not a program function, then check that the parameters do not have an associated mode.
            if !self.is_program_function && input_var.mode() != ParamMode::None {
                self.emit_err(TypeCheckerError::helper_function_inputs_cannot_have_modes(
                    input_var.span,
                ));
            }

            // Check for conflicting variable names.
            if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
                input_var.identifier.name,
                VariableSymbol {
                    type_: input_var.type_.clone(),
                    span: input_var.identifier.span(),
                    declaration: VariableType::Input(input_var.mode()),
                },
            ) {
                self.handler.emit_err(err);
            }
        });

        self.visit_block(&function.block);

        // Check that the return type is valid.
        self.assert_type_is_valid(function.span, &function.output);

        // Ensure there are no nested tuples in the return type.
        if let Type::Tuple(tys) = &function.output {
            for ty in &tys.0 {
                self.assert_not_tuple(function.span, ty);
            }
        }

        // Exit the scope for the function's parameters and body.
        self.exit_scope(scope_index);

        // Traverse and check the finalize block if it exists.
        if let Some(finalize) = &function.finalize {
            self.is_finalize = true;
            // The function's finalize block does not have a return statement.
            self.has_return = false;

            if !self.is_program_function {
                self.emit_err(TypeCheckerError::only_program_functions_can_have_finalize(
                    finalize.span,
                ));
            }

            // Create a new child scope for the finalize block.
            let scope_index = self.create_child_scope();

            finalize.input.iter().for_each(|input_var| {
                // Check that the type of input parameter is valid.
                self.assert_type_is_valid(input_var.span, &input_var.type_);
                self.assert_not_tuple(input_var.span, &input_var.type_);

                // Check that the input parameter is not constant or private.
                if input_var.mode() == ParamMode::Const || input_var.mode() == ParamMode::Private {
                    self.emit_err(TypeCheckerError::finalize_input_mode_must_be_public(input_var.span));
                }

                // Check for conflicting variable names.
                if let Err(err) = self.symbol_table.borrow_mut().insert_variable(
                    input_var.identifier.name,
                    VariableSymbol {
                        type_: input_var.type_.clone(),
                        span: input_var.identifier.span(),
                        declaration: VariableType::Input(input_var.mode()),
                    },
                ) {
                    self.handler.emit_err(err);
                }
            });

            // Type check the finalize block.
            self.visit_block(&finalize.block);

            // Check that the return type is valid.
            self.assert_type_is_valid(finalize.span, &finalize.output);

            // Exit the scope for the finalize block.
            self.exit_scope(scope_index);

            // Ensure there are no nested tuples in the return type.
            if let Type::Tuple(tys) = &finalize.output {
                for ty in &tys.0 {
                    self.assert_not_tuple(function.span, ty);
                }
            }

            self.is_finalize = false;
        }

        // Exit the function's scope.
        self.exit_scope(function_index);

        // Unset `is_program_function` flag.
        self.is_program_function = false;
    }
}
