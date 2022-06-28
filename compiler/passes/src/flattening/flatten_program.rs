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

use leo_ast::*;

use crate::{Declaration, Flattener, Value, VariableSymbol};

impl<'a> ProgramReconstructor for Flattener<'a> {
    fn reconstruct_function(&mut self, input: Function) -> Function {
        let f_name = input.name();
        let f_inputs = if input.is_main() {
            let (non_consts, consts): (Vec<_>, Vec<_>) = input
                .input
                .iter()
                .cloned()
                .partition(|fi| matches!(fi, FunctionInput::Variable(v) if v.mode() != ParamMode::Const));

            if let Some(const_inputs) = self.constant_inputs {
                let id = if let Some(main) = self.symbol_table.borrow_mut().functions.get_mut(&f_name) {
                    main.input = non_consts.clone();
                    main.id
                } else {
                    todo!("no main function");
                };

                let fn_scope = &self.symbol_table.borrow().scopes[id];

                fn_scope
                    .borrow_mut()
                    .variables
                    .extend(consts.into_iter().map(|c| match c {
                        FunctionInput::Variable(var) => {
                            if let Some(const_value) = const_inputs.get(&var.identifier.name) {
                                (
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
                                            InputValue::Integer(_, value) => match value {
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
                                                _ => todo!("incorrect type"),
                                            },
                                            InputValue::Scalar(value) if matches!(var.type_, Type::Scalar) => {
                                                Value::Scalar(value.clone(), var.span())
                                            }
                                            InputValue::String(value) if matches!(var.type_, Type::String) => {
                                                Value::String(value.clone(), var.span())
                                            }
                                            _ => todo!("incorrect type"),
                                        })),
                                    },
                                )
                            } else {
                                todo!("var not found in constats")
                            }
                        }
                    }));
            } else if !consts.is_empty() && self.constant_inputs.is_none() {
                todo!("emit error")
            }

            non_consts
        } else {
            input.input.clone()
        };

        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table.swap(prev_st.borrow().get_fn_scope(&f_name).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));
        self.block_index = 0;

        let f = Function {
            identifier: input.identifier,
            input: f_inputs,
            output: input.output,
            core_mapping: input.core_mapping.clone(),
            block: self.reconstruct_block(input.block),
            span: input.span,
        };

        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table.swap(prev_st.get_fn_scope(&f_name).unwrap());
        self.symbol_table = RefCell::new(prev_st);

        f
    }
}
