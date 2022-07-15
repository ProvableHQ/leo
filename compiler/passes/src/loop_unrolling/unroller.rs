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

use leo_errors::emitter::Handler;

use crate::SymbolTable;

pub struct Unroller<'a> {
    /// The symbol table for the function being processed.
    pub(crate) symbol_table: RefCell<SymbolTable>,
    /// The index of the current block scope.
    pub(crate) block_index: usize,
    /// An error handler used for any errors found during unrolling.
    pub(crate) handler: &'a Handler,
    /// Are we in the midst of unrolling a loop?
    pub(crate) is_unrolling: bool,
}

impl<'a> Unroller<'a> {
    pub(crate) fn new(symbol_table: SymbolTable, handler: &'a Handler) -> Self {
        Self {
            symbol_table: RefCell::new(symbol_table),
            block_index: 0,
            handler,
            is_unrolling: false,
        }
    }

    /// Returns the index of the current block scope.
    /// Note that if we are in the midst of unrolling an IterationStatement, a new scope is created.
    pub(crate) fn get_current_block(&mut self) -> usize {
        if self.is_unrolling {
            self.symbol_table.borrow_mut().insert_block()
        } else {
            self.block_index
        }
    }
}
