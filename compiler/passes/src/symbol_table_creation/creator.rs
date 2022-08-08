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

use crate::SymbolTable;

use leo_ast::*;
use leo_errors::emitter::Handler;

/// A compiler pass during which the `SymbolTable` is created.
/// Note that this pass only creates the initial entries for functions and circuits.
/// The table is populated further during the type checking pass.
pub struct SymbolTableCreator<'a> {
    /// The `SymbolTable` constructed by this compiler pass.
    pub(crate) symbol_table: SymbolTable,
    /// The error handler.
    handler: &'a Handler,
}

impl<'a> SymbolTableCreator<'a> {
    pub fn new(handler: &'a Handler) -> Self {
        Self {
            symbol_table: Default::default(),
            handler,
        }
    }
}

impl<'a> ExpressionVisitor<'a> for SymbolTableCreator<'a> {
    type AdditionalInput = ();
    type Output = ();
}

impl<'a> StatementVisitor<'a> for SymbolTableCreator<'a> {}

impl<'a> ProgramVisitor<'a> for SymbolTableCreator<'a> {
    fn visit_function(&mut self, input: &'a Function) {
        if let Err(err) = self.symbol_table.insert_fn(input.name(), input) {
            self.handler.emit_err(err);
        }
    }

    fn visit_circuit(&mut self, input: &'a Circuit) {
        if let Err(err) = self.symbol_table.insert_circuit(input.name(), input) {
            self.handler.emit_err(err);
        }
    }

    fn visit_import(&mut self, input: &'a Program) {
        self.visit_program(input)
    }
}
