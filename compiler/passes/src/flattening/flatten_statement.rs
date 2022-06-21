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

use std::{cell::RefCell, iter::repeat};

use leo_ast::*;

use crate::Flattener;

impl<'a> StatementReconstructor for Flattener<'a> {
    fn reconstruct_iteration(&mut self, input: IterationStatement) -> Statement {
        if let (
            Expression::Value(ValueExpression::Integer(_, start_str_content, _)),
            Expression::Value(ValueExpression::Integer(_, stop_str_content, _)),
        ) = (input.start, input.stop)
        {
            let start = start_str_content.parse::<usize>().unwrap();
            let stop = stop_str_content.parse::<usize>().unwrap();

            Statement::Block(Block {
                // will panic if stop == usize::MAX
                statements: repeat(input.block.statements.clone())
                    .take(if start > stop {
                        start + input.inclusive as usize - stop
                    } else {
                        stop + input.inclusive as usize - start
                    })
                    .flatten()
                    .collect(),
                span: input.span,
            })
        } else {
            todo!("This operation is not yet supported.")
        }
    }

    fn reconstruct_block(&mut self, input: Block) -> Block {
        // TODO: handle getting block scope?
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

        b
    }
}
