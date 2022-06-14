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

use crate::{BlockSymbol, SymbolTable};

#[derive(Clone)]
pub struct FunctionSymbol<'a> {
    pub(crate) scope: &'a SymbolTable<'a>,
    pub(crate) type_: &'a Type,
    pub(crate) span: Span,
    pub(crate) input: &'a [FunctionInput],
    pub(crate) block: BlockSymbol<'a>,
}

impl<'a> FunctionSymbol<'a> {
    pub(crate) fn new(func: &'a Function, scope: &'a SymbolTable<'a>) -> Self {
        let scope = scope.subscope();
        let block = BlockSymbol::new(&func.block, &scope);
        Self {
            scope: &scope,
            type_: &func.output,
            span: func.span,
            input: &func.input,
            block,
        }
    }
}
