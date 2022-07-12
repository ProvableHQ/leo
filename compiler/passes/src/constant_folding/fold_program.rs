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

use std::cell::RefCell;

use indexmap::IndexMap;
use leo_ast::*;
use leo_errors::FlattenError;

use crate::{ConstantFolder, Declaration, Flattener, Value, VariableSymbol};

impl<'a> ProgramReconstructor for ConstantFolder<'a> {
    fn reconstruct_function(&mut self, input: Function) -> Function {
        let f_name = input.name();

        // If the function is main we need to flatten its constants' inputs.
        let f_inputs = if input.is_main() {
            // Grab the consts and non_const inputs.
            let (non_consts, consts): (Vec<_>, Vec<_>) = input
                .input
                .iter()
                .cloned()
                .partition(|fi| matches!(fi, FunctionInput::Variable(v) if v.mode() != ParamMode::Const));

            // If there are constant inputs from the input file.
            if let Some(const_inputs) = self.constant_inputs {
                // We grab the main function's scope id.
                let id = if let Some(main) = self.symbol_table.borrow_mut().functions.get_mut(&f_name) {
                    main.input = non_consts.clone();
                    main.id
                } else {
                    self.handler.emit_err(FlattenError::no_main_function());
                    return input;
                };

                // We grab the main function's scope.
                let fn_scope = &self.symbol_table.borrow().scopes[id];

                // We go through the constants in the main function input.
                // Grabbing variable definitions for each main input with the value of
                // the specified constant from the input file.
                let mut const_input_values = IndexMap::new();
                for c in consts.into_iter() {
                    match c {
                        FunctionInput::Variable(var) => {
                            if let Some(const_value) = const_inputs.get(&var.identifier.name) {
                                const_input_values.insert(
                                    var.identifier.name,
                                    VariableSymbol {
                                        type_: var.type_,
                                        span: var.span(),
                                        declaration: Declaration::Const(Some(match const_value {
                                            InputValue::Address(value) if matches!(var.type_, Type::Address) => {
                                                Value::Address(value.clone(), var.span())
                                            }
                                            InputValue::Boolean(value) if matches!(var.type_, Type::Boolean) => {
                                                Value::Boolean(*value, var.span())
                                            }
                                            InputValue::Field(value) if matches!(var.type_, Type::Field) => {
                                                Value::Field(value.clone(), var.span())
                                            }
                                            InputValue::Group(value) if matches!(var.type_, Type::Group) => {
                                                Value::Group(Box::new(value.clone()))
                                            }
                                            InputValue::Integer(int_type, value) => match value {
                                                IntegerValue::I8(value)
                                                    if matches!(var.type_, Type::IntegerType(IntegerType::I8)) =>
                                                {
                                                    Value::I8(*value, var.span())
                                                }
                                                IntegerValue::I16(value)
                                                    if matches!(var.type_, Type::IntegerType(IntegerType::I16)) =>
                                                {
                                                    Value::I16(*value, var.span())
                                                }
                                                IntegerValue::I32(value)
                                                    if matches!(var.type_, Type::IntegerType(IntegerType::I32)) =>
                                                {
                                                    Value::I32(*value, var.span())
                                                }
                                                IntegerValue::I64(value)
                                                    if matches!(var.type_, Type::IntegerType(IntegerType::I64)) =>
                                                {
                                                    Value::I64(*value, var.span())
                                                }
                                                IntegerValue::I128(value)
                                                    if matches!(var.type_, Type::IntegerType(IntegerType::I128)) =>
                                                {
                                                    Value::I128(*value, var.span())
                                                }
                                                IntegerValue::U8(value)
                                                    if matches!(var.type_, Type::IntegerType(IntegerType::U8)) =>
                                                {
                                                    Value::U8(*value, var.span())
                                                }
                                                IntegerValue::U16(value)
                                                    if matches!(var.type_, Type::IntegerType(IntegerType::U16)) =>
                                                {
                                                    Value::U16(*value, var.span())
                                                }
                                                IntegerValue::U32(value)
                                                    if matches!(var.type_, Type::IntegerType(IntegerType::U32)) =>
                                                {
                                                    Value::U32(*value, var.span())
                                                }
                                                IntegerValue::U64(value)
                                                    if matches!(var.type_, Type::IntegerType(IntegerType::U64)) =>
                                                {
                                                    Value::U64(*value, var.span())
                                                }
                                                IntegerValue::U128(value)
                                                    if matches!(var.type_, Type::IntegerType(IntegerType::U128)) =>
                                                {
                                                    Value::U128(*value, var.span())
                                                }
                                                _ => {
                                                    self.handler.emit_err(
                                                        FlattenError::main_function_mismatching_const_input_type(
                                                            var.type_,
                                                            int_type,
                                                            var.span(),
                                                        ),
                                                    );
                                                    return input;
                                                }
                                            },
                                            InputValue::Scalar(value) if matches!(var.type_, Type::Scalar) => {
                                                Value::Scalar(value.clone(), var.span())
                                            }
                                            InputValue::String(value) if matches!(var.type_, Type::String) => {
                                                Value::String(value.clone(), var.span())
                                            }
                                            t => {
                                                self.handler.emit_err(
                                                    FlattenError::main_function_mismatching_const_input_type(
                                                        var.type_,
                                                        Type::from(t),
                                                        var.span(),
                                                    ),
                                                );
                                                return input;
                                            }
                                        })),
                                    },
                                );
                            } else {
                                self.handler.emit_err(FlattenError::input_file_does_not_have_constant(
                                    &var.identifier.name,
                                    var.span(),
                                ));
                                return input;
                            }
                        }
                    }
                }

                // We then insert those constant inputs as variables to the main function scope.
                fn_scope.borrow_mut().variables.extend(const_input_values);
            } else if !consts.is_empty() && self.constant_inputs.is_none() {
                // This case is for there being constants in the main function.
                // But no constants in the input file.
                self.handler
                    .emit_err(FlattenError::input_file_has_no_constants(input.span()));
                return input;
            }

            // We return the non constants of the main function as the new expected arguments.
            non_consts
        } else {
            input.input.clone()
        };

        // Grab our function scope.
        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table.swap(prev_st.borrow().get_fn_scope(&f_name).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));
        // Set our current block scope index to 0
        self.block_index = 0;

        // Reconstruct the function block.
        let f = Function {
            identifier: input.identifier,
            input: f_inputs,
            output: input.output,
            core_mapping: input.core_mapping.clone(),
            block: self.reconstruct_block(input.block),
            span: input.span,
        };

        // Pop back to parent scope.
        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table.swap(prev_st.get_fn_scope(&f_name).unwrap());
        self.symbol_table = RefCell::new(prev_st);

        f
    }
}
