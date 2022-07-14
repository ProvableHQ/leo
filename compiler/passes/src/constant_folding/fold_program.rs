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

use crate::ConstantFolder;

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
            // TODO: Fix how inputs are handled.
            if let Some(const_inputs) = self.constant_inputs {
                // We grab the main function's scope id.
                let id = if let Some(main) = self.symbol_table.borrow_mut().functions.get_mut(&f_name) {
                    main.input = non_consts.clone();
                    main.index
                } else {
                    // self.handler.emit_err(FlattenError::no_main_function());
                    return input;
                };

                // We grab the main function's scope.
                let fn_scope = &self.symbol_table.borrow().scopes[id];

                // We go through the constants in the main function input.
                // Grabbing variable definitions for each main input with the value of
                // the specified constant from the input file.
                let mut const_input_values = IndexMap::new();

                // for c in consts.into_iter() {
                //     match c {
                //         FunctionInput::Variable(var) => {
                //             if let Some(const_value) = const_inputs.get(&var.identifier.name) {
                //                 const_input_values.insert(
                //                     var.identifier.name,
                //                     Some(match const_value {
                //                         InputValue::Address(value) if matches!(var.type_, Type::Address) => {
                //                             Value::Address(value.clone())
                //                         }
                //                         InputValue::Boolean(value) if matches!(var.type_, Type::Boolean) => {
                //                             Value::Boolean(*value)
                //                         }
                //                         InputValue::Field(value) if matches!(var.type_, Type::Field) => {
                //                             Value::Field(value.clone())
                //                         }
                //                         InputValue::Group(value) if matches!(var.type_, Type::Group) => {
                //                             Value::Group(Box::new(value.clone()))
                //                         }
                //                         InputValue::I8(value) if matches!(var.type_, Type::I8) => Value::I8(*value),
                //                         InputValue::I16(value) if matches!(var.type_, Type::I16) => Value::I16(*value),
                //                         InputValue::I32(value) if matches!(var.type_, Type::I32) => Value::I32(*value),
                //                         InputValue::I64(value) if matches!(var.type_, Type::I64) => Value::I64(*value),
                //                         InputValue::I128(value) if matches!(var.type_, Type::I128) => {
                //                             Value::I128(*value)
                //                         }
                //                         InputValue::U8(value) if matches!(var.type_, Type::U8) => Value::U8(*value),
                //                         InputValue::U16(value) if matches!(var.type_, Type::U16) => Value::U16(*value),
                //                         InputValue::U32(value) if matches!(var.type_, Type::U32) => Value::U32(*value),
                //                         InputValue::U64(value) if matches!(var.type_, Type::U64) => Value::U64(*value),
                //                         InputValue::U128(value) if matches!(var.type_, Type::U128) => {
                //                             Value::U128(*value)
                //                         }
                //                         t => {
                //                             self.handler.emit_err(
                //                                 FlattenError::main_function_mismatching_const_input_type(
                //                                     &var.type_,
                //                                     Type::from(t),
                //                                     var.span(),
                //                                 ),
                //                             );
                //                             return input;
                //                         }
                //                     }),
                //                 );
                //             } else {
                //                 self.handler.emit_err(FlattenError::input_file_does_not_have_constant(
                //                     &var.identifier.name,
                //                     var.span(),
                //                 ));
                //                 return input;
                //             }
                //         }
                //     }
                // }

                // We then insert those constant inputs as variables to the main function scope.
                fn_scope.borrow_mut().values.extend(const_input_values);
            } else if !consts.is_empty() && self.constant_inputs.is_none() {
                // This case is for there being constants in the main function.
                // But no constants in the input file.
                // self.handler
                //     .emit_err(FlattenError::input_file_has_no_constants(input.span()));
                return input;
            }

            // We return the non constants of the main function as the new expected arguments.
            non_consts
        } else {
            input.input.clone()
        };

        // Grab our function scope.
        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table.swap(prev_st.borrow().lookup_fn_scope(f_name).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));
        // Set our current block scope index to 0
        self.scope_index = 0;

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
        self.symbol_table.swap(prev_st.lookup_fn_scope(f_name).unwrap());
        self.symbol_table = RefCell::new(prev_st);

        f
    }
}
