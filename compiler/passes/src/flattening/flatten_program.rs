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
                    FunctionInput::Variable(var) => (
                        var.identifier.name,
                        VariableSymbol {
                            type_: var.type_,
                            span: var.span(),
                            declaration: Declaration::Const(Some(match var.type_ {
                                Type::Address => Value::Address(Default::default(), var.span()),
                                Type::Boolean => Value::Boolean(Default::default(), var.span()),
                                Type::Field => Value::Field(Default::default(), var.span()),
                                Type::Group => {
                                    Value::Group(Box::new(GroupLiteral::Single(Default::default(), var.span())))
                                }
                                Type::Scalar => Value::Scalar(Default::default(), var.span()),
                                Type::String => Value::String(Default::default(), var.span()),
                                Type::IntegerType(int_type) => match int_type {
                                    IntegerType::U8 => Value::U8(0, var.span()),
                                    IntegerType::U16 => Value::U16(0, var.span()),
                                    IntegerType::U32 => Value::U32(0, var.span()),
                                    IntegerType::U64 => Value::U64(0, var.span()),
                                    IntegerType::U128 => Value::U128(0, var.span()),
                                    IntegerType::I8 => Value::I8(0, var.span()),
                                    IntegerType::I16 => Value::I16(0, var.span()),
                                    IntegerType::I32 => Value::I32(0, var.span()),
                                    IntegerType::I64 => Value::I64(0, var.span()),
                                    IntegerType::I128 => Value::I128(0, var.span()),
                                },
                                Type::Identifier(_) => unreachable!(),
                                Type::Err => unreachable!(),
                            })),
                        },
                    ),
                }));

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
