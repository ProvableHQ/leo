// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{FromAst, Node, PartialType, Scope, Statement};
use leo_errors::{Result, Span};

use std::cell::Cell;

#[derive(Clone)]
pub struct BlockStatement<'a> {
    pub parent: Cell<Option<&'a Statement<'a>>>,
    pub span: Option<Span>,
    pub statements: Vec<Cell<&'a Statement<'a>>>,
    pub scope: &'a Scope<'a>,
}

impl<'a> Node for BlockStatement<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> FromAst<'a, leo_ast::Block> for BlockStatement<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        statement: &leo_ast::Block,
        _expected_type: Option<PartialType<'a>>,
    ) -> Result<Self> {
        let new_scope = scope.make_subscope();

        let mut output = vec![];
        for item in statement.statements.iter() {
            output.push(Cell::new(<&'a Statement<'a>>::from_ast(new_scope, item, None)?));
        }
        Ok(BlockStatement {
            parent: Cell::new(None),
            span: Some(statement.span.clone()),
            statements: output,
            scope: new_scope,
        })
    }
}

impl<'a> Into<leo_ast::Block> for &BlockStatement<'a> {
    fn into(self) -> leo_ast::Block {
        leo_ast::Block {
            statements: self.statements.iter().map(|statement| statement.get().into()).collect(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
