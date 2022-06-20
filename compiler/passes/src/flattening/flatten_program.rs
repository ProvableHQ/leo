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

use leo_ast::*;

use crate::{Declaration, Flattener, Value, VariableSymbol};

impl<'a> ProgramReconstructor for Flattener<'a> {
    fn reconstruct_function(&mut self, input: Function) -> Function {
        let f_inputs = if input.is_main() {
            let (non_consts, consts): (Vec<_>, Vec<_>) = input
                .input
                .iter()
                .cloned()
                .partition(|fi| matches!(fi, FunctionInput::Variable(v) if v.mode() != ParamMode::Const));

            let id = if let Some(main) = self.symbol_table.borrow_mut().functions.get_mut(&input.identifier.name) {
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
                            span: var.span,
                            declaration: Declaration::Const(Some(match var.type_ {
                                Type::Address => Value::Address(Default::default()),
                                Type::Boolean => Value::Boolean(Default::default()),
                                Type::Field => Value::Field(Default::default()),
                                Type::Group => {
                                    Value::Group(Box::new(GroupValue::Single(Default::default(), Default::default())))
                                }
                                Type::Scalar => Value::Scalar(Default::default()),
                                Type::String => Value::String(Default::default()),
                                Type::IntegerType(int_type) => match int_type {
                                    IntegerType::U8 => Value::U8(0),
                                    IntegerType::U16 => Value::U16(0),
                                    IntegerType::U32 => Value::U32(0),
                                    IntegerType::U64 => Value::U64(0),
                                    IntegerType::U128 => Value::U128(0),
                                    IntegerType::I8 => Value::I8(0),
                                    IntegerType::I16 => Value::I16(0),
                                    IntegerType::I32 => Value::I32(0),
                                    IntegerType::I64 => Value::I64(0),
                                    IntegerType::I128 => Value::I128(0),
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

        Function {
            identifier: input.identifier,
            input: f_inputs,
            output: input.output,
            core_mapping: input.core_mapping.clone(),
            block: self.reconstruct_block(input.block),
            span: input.span,
        }
    }
}
