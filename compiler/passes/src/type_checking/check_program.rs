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

use crate::{CallType, TypeChecker, VariableSymbol, VariableType};

use leo_ast::*;
use leo_errors::TypeCheckerError;
use leo_span::sym;

use leo_errors::warnings::type_checker::TypeCheckerWarning;
use std::cell::RefCell;
use std::collections::HashSet;

// TODO: Generally, cleanup tyc logic.

impl<'a> ProgramVisitor<'a> for TypeChecker<'a> {
    fn visit_program(&mut self, input: &'a Program) {
        input.imports.values().for_each(|import| self.visit_import(import));

        // Visit the circuits in the program.
        input
            .circuits
            .values()
            .for_each(|circuit| self.visit_circuit(circuit));

        // TODO: Improve error message to provide a path associated with the cycle.
        // Check that the type dependency graph does not contain any cycle.
        if self.type_graph.contains_cycle() {
            self.emit_err(TypeCheckerError::recursive_type());
        }

        // Visit the functions in the program.
        input
            .functions
            .values()
            .for_each(|function| self.visit_function(function));

        // TODO: Improve error message to provide a path associated with the cycle.
        // Check that the call graph does not contain any cycle.
        if self.call_graph.contains_cycle() {
            self.emit_err(TypeCheckerError::recursive_function())
        }

        // Check that all "helper functions" and "inlined functions" are called at least once.
        self.symbol_table
            .borrow()
            .functions
            .iter()
            .filter_map(|(name, function_symbol)| match self.call_graph.contains_node(*name) {
                true => None,
                false => Some(TypeCheckerWarning::function_is_never_called(name, function_symbol.span)),
            })
            .for_each(|warning| self.emit_warning(warning));
    }

    fn visit_function(&mut self, input: &'a Function) {
        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table
            .swap(prev_st.borrow().lookup_fn_scope(input.name()).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));

        // TODO: Cleanup. Too many unwraps
        // Lookup the function's call type.
        self.function = Some((
            input.name(),
            self.symbol_table
                .borrow()
                .lookup_fn_symbol(input.name())
                .unwrap()
                .call_type,
        ));

        self.has_return = false;
        self.parent = Some(input.name());

        // Program and helper functions must have input parameters.
        // Note that the unwrap is safe since we set the call type above.
        if self.function.unwrap().1 != CallType::Inlined && input.input.is_empty() {
            self.emit_err(TypeCheckerError::function_must_have_inputs(
                self.function.unwrap().1,
                input.span(),
            ));
        }

        input.input.iter().for_each(|input_var| {
            // Check that the type of input parameter is valid.
            self.assert_type_is_valid(input_var.span, &input_var.type_);
            self.assert_not_tuple(input_var.span, &input_var.type_);

            // Note that the unwrap is safe since we set the call type above.
            match self.function.unwrap().1 {
                // Program functions cannot have constant input parameters.
                CallType::Program => {
                    if input_var.mode() == ParamMode::Const {
                        self.emit_err(TypeCheckerError::program_functions_cannot_have_const_inputs(
                            input_var.span(),
                        ));
                    }
                }
                // If the function is not a program function, then check that the parameters do not have an associated mode.
                _ => {
                    if input_var.mode() != ParamMode::None {
                        self.emit_err(TypeCheckerError::helper_function_inputs_cannot_have_modes(
                            input_var.span,
                        ));
                    }
                }
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
        self.visit_block(&input.block);

        if !self.has_return {
            self.emit_err(TypeCheckerError::function_has_no_return(input.name(), input.span()));
        } else {
            // Check that the return type is valid.
            // TODO: Span should be just for the return type.
            self.assert_type_is_valid(input.span, &input.output);
        }

        // Ensure there are no nested tuples in the return type.
        if let Type::Tuple(tys) = &input.output {
            for ty in &tys.0 {
                self.assert_not_tuple(input.span, ty);
            }
        }

        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table.swap(prev_st.lookup_fn_scope(input.name()).unwrap());
        self.symbol_table = RefCell::new(prev_st);

        // Unset the call type.
        self.function = None;
    }

    fn visit_circuit(&mut self, input: &'a Circuit) {
        // The set of members of the circuit.
        let mut used = HashSet::new();

        for member in input.members.iter() {
            // Check for conflicting circuit/record member names.
            if !used.insert(member.name()) {
                self.emit_err(if input.is_record {
                    TypeCheckerError::duplicate_record_variable(input.name(), input.span())
                } else {
                    TypeCheckerError::duplicate_circuit_member(input.name(), input.span())
                });
            }

            // If `member` is a composite type, add it to the type dependency graph.
            if let CircuitMember::CircuitVariable(_, Type::Identifier(identifier)) = member {
                self.type_graph.add_edge(input.identifier.name, identifier.name);
            }
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
}
