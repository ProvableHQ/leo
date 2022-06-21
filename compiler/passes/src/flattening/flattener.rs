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
use leo_ast::LiteralExpression;
use leo_errors::emitter::Handler;
use leo_span::Symbol;

use crate::SymbolTable;

pub struct Flattener<'a> {
    pub(crate) symbol_table: RefCell<SymbolTable>,
    pub(crate) block_index: usize,
    pub(crate) var_references: IndexMap<Symbol, LiteralExpression>,
    pub(crate) _handler: &'a Handler,
}

impl<'a> Flattener<'a> {
    pub(crate) fn new(symbol_table: SymbolTable, handler: &'a Handler) -> Self {
        Self {
            symbol_table: RefCell::new(symbol_table),
            block_index: 0,
            var_references: IndexMap::new(),
            _handler: handler,
        }
    }
}
