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

use crate::{AsgConvertError, Expression, FromAst, Node, PartialType, Scope, Span, Statement};

use std::sync::{Arc, Weak};

pub struct ExpressionStatement {
    pub parent: Option<Weak<Statement>>,
    pub span: Option<Span>,
    pub expression: Arc<Expression>,
}

impl Node for ExpressionStatement {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl FromAst<leo_ast::ExpressionStatement> for ExpressionStatement {
    fn from_ast(
        scope: &Scope,
        statement: &leo_ast::ExpressionStatement,
        _expected_type: Option<PartialType>,
    ) -> Result<Self, AsgConvertError> {
        let expression = Arc::<Expression>::from_ast(scope, &statement.expression, None)?;

        Ok(ExpressionStatement {
            parent: None,
            span: Some(statement.span.clone()),
            expression,
        })
    }
}

impl Into<leo_ast::ExpressionStatement> for &ExpressionStatement {
    fn into(self) -> leo_ast::ExpressionStatement {
        leo_ast::ExpressionStatement {
            expression: self.expression.as_ref().into(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
