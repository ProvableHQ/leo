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

use crate::{Declaration, Flattener, Value};

impl<'a> StatementReconstructor for Flattener<'a> {
    fn reconstruct_definition(&mut self, input: DefinitionStatement) -> Statement {
        let (value, const_val) = self.reconstruct_expression(input.value);
        let mut st = self.symbol_table.borrow_mut();

        if let Some(const_val) = const_val {
            input.variable_names.iter().for_each(|var| {
                // TODO variable could be in parent scope technically and needs to be updated appropriately.
                // Could be fixed by making this a method that checks st, then parent then updates.
                // TODO remove iterator variable when done.
                let mut var = st.variables.get_mut(&var.identifier.name).unwrap();
                var.declaration = match &var.declaration {
                    Declaration::Const(_) => Declaration::Const(Some(const_val.clone())),
                    Declaration::Mut(_) => Declaration::Mut(Some(const_val.clone())),
                    other => other.clone(),
                }
            });
        }

        Statement::Definition(DefinitionStatement {
            declaration_type: input.declaration_type,
            variable_names: input.variable_names.clone(),
            type_: input.type_,
            value,
            span: input.span,
        })
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> Statement {
        let start = self.reconstruct_expression(input.start).0;
        let stop = self.reconstruct_expression(input.stop).0;

        if let (
            Expression::Literal(LiteralExpression::Integer(_, start_str_content, _)),
            Expression::Literal(LiteralExpression::Integer(_, stop_str_content, _)),
        ) = (start, stop)
        {
            let start = start_str_content.parse::<usize>().unwrap();
            let stop = stop_str_content.parse::<usize>().unwrap();
            let range = if start < stop {
                start..(stop + input.inclusive as usize)
            } else {
                stop..(start - input.inclusive as usize)
            };

            Statement::Block(Block {
                // will panic if stop == usize::MAX
                statements: range
                    .into_iter()
                    .flat_map(|iter_var| {
                        self.symbol_table.borrow_mut().variables.insert(
                            input.variable.name,
                            crate::VariableSymbol {
                                type_: input.type_,
                                span: input.variable.span,
                                declaration: Declaration::Const(Some(Value::from_usize(
                                    input.type_,
                                    iter_var,
                                    input.variable.span,
                                ))),
                            },
                        );

                        input
                            .block
                            .statements
                            .clone()
                            .into_iter()
                            .map(|s| self.reconstruct_statement(s))
                            .collect::<Vec<Statement>>()
                    })
                    .collect(),
                span: input.span,
            })
        } else {
            todo!("This operation is not yet supported.")
        }
    }

    fn reconstruct_block(&mut self, input: Block) -> Block {
        let current_block = self.block_index;
        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table
            .swap(prev_st.borrow().get_block_scope(current_block).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));
        self.block_index = 0;

        let b = Block {
            statements: input
                .statements
                .into_iter()
                .map(|s| self.reconstruct_statement(s))
                .collect(),
            span: input.span,
        };

        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table.swap(prev_st.get_block_scope(current_block).unwrap());
        self.symbol_table = RefCell::new(prev_st);
        self.block_index = current_block;
        self.block_index += 1;

        b
    }
}
