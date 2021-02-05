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

use leo_ast::IntegerType;

use crate::{
    AsgConvertError,
    Expression,
    ExpressionNode,
    FromAst,
    InnerVariable,
    Node,
    PartialType,
    Scope,
    Span,
    Statement,
    Variable,
};

use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

#[derive(Debug)]
pub struct IterationStatement {
    pub parent: Option<Weak<Statement>>,
    pub span: Option<Span>,
    pub variable: Variable,
    pub start: Arc<Expression>,
    pub stop: Arc<Expression>,
    pub body: Arc<Statement>,
}

impl Node for IterationStatement {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl FromAst<leo_ast::IterationStatement> for Arc<Statement> {
    fn from_ast(
        scope: &Scope,
        statement: &leo_ast::IterationStatement,
        _expected_type: Option<PartialType>,
    ) -> Result<Arc<Statement>, AsgConvertError> {
        let expected_index_type = Some(PartialType::Integer(None, Some(IntegerType::U32)));
        let start = Arc::<Expression>::from_ast(scope, &statement.start, expected_index_type.clone())?;
        let stop = Arc::<Expression>::from_ast(scope, &statement.stop, expected_index_type)?;
        let variable = Arc::new(RefCell::new(InnerVariable {
            id: uuid::Uuid::new_v4(),
            name: statement.variable.clone(),
            type_: start
                .get_type()
                .ok_or_else(|| AsgConvertError::unresolved_type(&statement.variable.name, &statement.span))?,
            mutable: false,
            declaration: crate::VariableDeclaration::IterationDefinition,
            references: vec![],
            assignments: vec![],
        }));
        scope
            .borrow_mut()
            .variables
            .insert(statement.variable.name.clone(), variable.clone());

        let statement = Arc::new(Statement::Iteration(IterationStatement {
            parent: None,
            span: Some(statement.span.clone()),
            variable: variable.clone(),
            stop,
            start,
            body: Arc::new(Statement::Block(crate::BlockStatement::from_ast(
                scope,
                &statement.block,
                None,
            )?)),
        }));
        variable.borrow_mut().assignments.push(Arc::downgrade(&statement));
        Ok(statement)
    }
}

impl Into<leo_ast::IterationStatement> for &IterationStatement {
    fn into(self) -> leo_ast::IterationStatement {
        leo_ast::IterationStatement {
            variable: self.variable.borrow().name.clone(),
            start: self.start.as_ref().into(),
            stop: self.stop.as_ref().into(),
            block: match self.body.as_ref() {
                Statement::Block(block) => block.into(),
                _ => unimplemented!(),
            },
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
