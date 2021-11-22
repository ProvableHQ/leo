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

use crate::{AsgId, Expression, FromAst, Node, PartialType, Scope, Statement};
use leo_errors::{Result, Span};

use std::cell::Cell;

#[derive(Clone)]
pub struct ExpressionStatement<'a> {
    pub id: AsgId,
    pub parent: Cell<Option<&'a Statement<'a>>>,
    pub span: Option<Span>,
    pub expression: Cell<&'a Expression<'a>>,
}

impl<'a> Node for ExpressionStatement<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    fn asg_id(&self) -> AsgId {
        self.id
    }
}

impl<'a> FromAst<'a, leo_ast::ExpressionStatement> for ExpressionStatement<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        statement: &leo_ast::ExpressionStatement,
        _expected_type: Option<PartialType<'a>>,
    ) -> Result<Self> {
        let expression = <&Expression<'a>>::from_ast(scope, &statement.expression, None)?;

        Ok(ExpressionStatement {
            id: scope.context.get_id(),
            parent: Cell::new(None),
            span: Some(statement.span.clone()),
            expression: Cell::new(expression),
        })
    }
}

impl<'a> Into<leo_ast::ExpressionStatement> for &ExpressionStatement<'a> {
    fn into(self) -> leo_ast::ExpressionStatement {
        leo_ast::ExpressionStatement {
            expression: self.expression.get().into(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
