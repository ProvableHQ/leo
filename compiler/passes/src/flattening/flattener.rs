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

use leo_ast::Definitions;
use leo_errors::emitter::Handler;
use leo_span::Symbol;

use crate::SymbolTable;

pub struct Flattener<'a> {
    pub(crate) symbol_table: RefCell<SymbolTable>,
    pub(crate) constant_inputs: Option<&'a Definitions>,
    pub(crate) block_index: usize,
    pub(crate) handler: &'a Handler,
    pub(crate) next_block_non_const: bool,
    pub(crate) non_const_block: bool,
    pub(crate) negate: bool,
    pub(crate) create_iter_scopes: bool,
    pub(crate) deconstify_buffer: Option<Vec<Symbol>>,
}

impl<'a> Flattener<'a> {
    pub(crate) fn deconstify_buffered(&mut self) {
        let mut st = self.symbol_table.borrow_mut();
        let mut names = self.deconstify_buffer.take().unwrap_or_default();
        names.sort();
        names.dedup();
        for name in names {
            st.deconstify_variable(&name);
        }
    }
}

impl<'a> Flattener<'a> {
    pub(crate) fn new(
        symbol_table: SymbolTable,
        handler: &'a Handler,
        constant_inputs: Option<&'a Definitions>,
    ) -> Self {
        Self {
            symbol_table: RefCell::new(symbol_table),
            constant_inputs,
            block_index: 0,
            handler,
            next_block_non_const: false,
            non_const_block: false,
            negate: false,
            create_iter_scopes: false,
            deconstify_buffer: None,
        }
    }
}
