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

use crate::{AsgConvertError, FromAst, InnerScope, Node, PartialType, Scope, Span, Statement};

use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct BlockStatement {
    pub parent: Option<Weak<Statement>>,
    pub span: Option<Span>,
    pub statements: Vec<Arc<Statement>>,
    pub scope: Scope,
}

impl Node for BlockStatement {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl FromAst<leo_ast::Block> for BlockStatement {
    fn from_ast(
        scope: &Scope,
        statement: &leo_ast::Block,
        _expected_type: Option<PartialType>,
    ) -> Result<Self, AsgConvertError> {
        let new_scope = InnerScope::make_subscope(scope);

        let mut output = vec![];
        for item in statement.statements.iter() {
            output.push(Arc::<Statement>::from_ast(&new_scope, item, None)?);
        }
        Ok(BlockStatement {
            parent: None,
            span: Some(statement.span.clone()),
            statements: output,
            scope: new_scope,
        })
    }
}

impl Into<leo_ast::Block> for &BlockStatement {
    fn into(self) -> leo_ast::Block {
        leo_ast::Block {
            statements: self
                .statements
                .iter()
                .map(|statement| statement.as_ref().into())
                .collect(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
