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

use crate::SymbolTable;

#[derive(Debug, Default)]
pub struct CreateSymbolTable<'a> {
    symbol_table: SymbolTable<'a>,
}

impl<'a> CreateSymbolTable<'a> {
    pub fn symbol_table(self) -> SymbolTable<'a> {
        self.symbol_table
    }
}

impl<'a> ExpressionVisitor<'a> for CreateSymbolTable<'a> {}

impl<'a> StatementVisitor<'a> for CreateSymbolTable<'a> {}

impl<'a> ProgramVisitor<'a> for CreateSymbolTable<'a> {
    fn visit_function(&mut self, input: &'a Function) -> VisitResult {
        if let Err(err) = self.symbol_table.insert_fn(input.name(), input) {
            todo!("{:?}", err)
        }
        VisitResult::SkipChildren
    }
}
