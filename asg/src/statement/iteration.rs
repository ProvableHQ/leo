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

use std::cell::{Cell, RefCell};

#[derive(Clone)]
pub struct IterationStatement<'a> {
    pub parent: Cell<Option<&'a Statement<'a>>>,
    pub span: Option<Span>,
    pub variable: &'a Variable<'a>,
    pub start: Cell<&'a Expression<'a>>,
    pub stop: Cell<&'a Expression<'a>>,
    pub body: Cell<&'a Statement<'a>>,
}

impl<'a> Node for IterationStatement<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> FromAst<'a, leo_ast::IterationStatement> for &'a Statement<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        statement: &leo_ast::IterationStatement,
        _expected_type: Option<PartialType<'a>>,
    ) -> Result<Self, AsgConvertError> {
        let expected_index_type = Some(PartialType::Integer(None, Some(IntegerType::U32)));
        let start = <&Expression<'a>>::from_ast(scope, &statement.start, expected_index_type.clone())?;
        let stop = <&Expression<'a>>::from_ast(scope, &statement.stop, expected_index_type)?;

        // Return an error if start or stop is not constant.
        if !start.is_consty() {
            return Err(AsgConvertError::unexpected_nonconst(
                &start.span().cloned().unwrap_or_default(),
            ));
        }
        if !stop.is_consty() {
            return Err(AsgConvertError::unexpected_nonconst(
                &stop.span().cloned().unwrap_or_default(),
            ));
        }

        let variable = scope.alloc_variable(RefCell::new(InnerVariable {
            id: scope.context.get_id(),
            name: statement.variable.clone(),
            type_: start
                .get_type()
                .ok_or_else(|| AsgConvertError::unresolved_type(&statement.variable.name, &statement.span))?,
            mutable: false,
            const_: true,
            declaration: crate::VariableDeclaration::IterationDefinition,
            references: vec![],
            assignments: vec![],
        }));
        scope
            .variables
            .borrow_mut()
            .insert(statement.variable.name.clone(), variable);

        let statement = scope.alloc_statement(Statement::Iteration(IterationStatement {
            parent: Cell::new(None),
            span: Some(statement.span.clone()),
            variable,
            stop: Cell::new(stop),
            start: Cell::new(start),
            body: Cell::new(scope.alloc_statement(Statement::Block(crate::BlockStatement::from_ast(
                scope,
                &statement.block,
                None,
            )?))),
        }));
        variable.borrow_mut().assignments.push(statement);
        Ok(statement)
    }
}

impl<'a> Into<leo_ast::IterationStatement> for &IterationStatement<'a> {
    fn into(self) -> leo_ast::IterationStatement {
        leo_ast::IterationStatement {
            variable: self.variable.borrow().name.clone(),
            start: self.start.get().into(),
            stop: self.stop.get().into(),
            block: match self.body.get() {
                Statement::Block(block) => block.into(),
                _ => unimplemented!(),
            },
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
