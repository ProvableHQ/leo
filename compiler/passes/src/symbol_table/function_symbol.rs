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
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>

use leo_ast::{Function, FunctionInput, Type};
use leo_span::Span;

use crate::{BlockSymbol, CreateSymbolTable, SymbolTable};

#[derive(Clone)]
pub struct FunctionSymbol<'a> {
    pub(crate) scope: &'a SymbolTable<'a>,
    pub(crate) type_: &'a Type,
    pub(crate) span: Span,
    pub(crate) input: &'a [FunctionInput],
    pub(crate) block: BlockSymbol<'a>,
}

impl<'a> CreateSymbolTable<'a> {
    pub(crate) fn new_function_symbol(&self, func: &'a Function) -> FunctionSymbol<'a> {
        let scope = self.arena.alloc(self.symbol_table.subscope());
        let block = self.new_block_symbol(&func.block, scope);
        FunctionSymbol {
            scope,
            type_: &func.output,
            span: func.span,
            input: &func.input,
            block,
        }
    }
}
