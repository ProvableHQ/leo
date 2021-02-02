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

use crate::{AsgConvertError, Expression, FromAst, Node, PartialType, Scope, Span, Statement, Type};

use std::sync::{Arc, Weak};

pub struct ReturnStatement {
    pub parent: Option<Weak<Statement>>,
    pub span: Option<Span>,
    pub expression: Arc<Expression>,
}

impl Node for ReturnStatement {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl FromAst<leo_ast::ReturnStatement> for ReturnStatement {
    fn from_ast(
        scope: &Scope,
        statement: &leo_ast::ReturnStatement,
        _expected_type: Option<PartialType>,
    ) -> Result<Self, AsgConvertError> {
        let return_type: Option<Type> = scope
            .borrow()
            .resolve_current_function()
            .map(|x| x.output.clone())
            .map(Into::into);
        Ok(ReturnStatement {
            parent: None,
            span: Some(statement.span.clone()),
            expression: Arc::<Expression>::from_ast(scope, &statement.expression, return_type.map(Into::into))?,
        })
    }
}

impl Into<leo_ast::ReturnStatement> for &ReturnStatement {
    fn into(self) -> leo_ast::ReturnStatement {
        leo_ast::ReturnStatement {
            expression: self.expression.as_ref().into(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
