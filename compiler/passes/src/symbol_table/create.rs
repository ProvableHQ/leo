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

use bumpalo::Bump;
use leo_ast::*;
use leo_errors::emitter::Handler;

use crate::SymbolTable;

pub struct CreateSymbolTable<'a> {
    pub(crate) symbol_table: &'a SymbolTable<'a>,
    handler: &'a Handler,
    pub(crate) arena: &'a Bump,
}

impl<'a> CreateSymbolTable<'a> {
    pub fn new(symbol_table: &'a SymbolTable<'a>, handler: &'a Handler, arena: &'a Bump) -> Self {
        Self {
            symbol_table,
            handler,
            arena,
        }
    }
}

impl<'a> ExpressionVisitor<'a> for CreateSymbolTable<'a> {
    type AdditionalInput = ();
    type Output = ();
}

impl<'a> StatementVisitor<'a> for CreateSymbolTable<'a> {}

impl<'a> ProgramVisitor<'a> for CreateSymbolTable<'a> {
    fn visit_function(&mut self, input: &'a Function) {
        let func = self.arena.alloc(self.new_function_symbol(input));
        if let Err(err) = self.symbol_table.insert_fn(input.name(), func) {
            self.handler.emit_err(err);
        }
    }
}
