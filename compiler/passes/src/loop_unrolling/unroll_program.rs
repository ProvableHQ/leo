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

use crate::Unroller;

impl ProgramReconstructor for Unroller<'_> {
    fn reconstruct_function(&mut self, function: Function) -> Function {
        let function_name = function.name();

        // Grab our function scope.
        let prev_st = std::mem::take(&mut self.symbol_table);
        self.symbol_table
            .swap(prev_st.borrow().lookup_fn_scope(function_name).unwrap());
        self.symbol_table.borrow_mut().parent = Some(Box::new(prev_st.into_inner()));
        // Set our current block scope index to 0
        self.block_index = 0;

        // Reconstruct the function block.
        let reconstructed_function = Function {
            annotations: function.annotations,
            identifier: function.identifier,
            input: function.input,
            output: function.output,
            core_mapping: function.core_mapping,
            block: self.reconstruct_block(function.block),
            span: function.span,
        };

        // Pop back to parent scope.
        let prev_st = *self.symbol_table.borrow_mut().parent.take().unwrap();
        self.symbol_table.swap(prev_st.lookup_fn_scope(function_name).unwrap());
        self.symbol_table = RefCell::new(prev_st);

        reconstructed_function
    }
}
