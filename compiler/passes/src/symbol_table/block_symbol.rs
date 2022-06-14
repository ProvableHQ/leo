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

use leo_ast::{Block, Statement};
use leo_span::Span;

use crate::SymbolTable;

#[derive(Clone)]
pub struct BlockSymbol<'a> {
    pub(crate) scope: SymbolTable<'a>,
    pub(crate) span: Span,
    pub(crate) statements: &'a [Statement],
}

impl<'a> BlockSymbol<'a> {
    pub(crate) fn new(block: &'a Block, scope: &'a SymbolTable<'a>) -> Self {
        let scope = scope.subscope();
        Self {
            scope,
            span: block.span,
            statements: &block.statements,
        }
    }
}
