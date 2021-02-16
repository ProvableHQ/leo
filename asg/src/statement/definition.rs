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
    Type,
    Variable,
};

use leo_ast::{AstError, DeprecatedError};

use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

#[derive(Debug)]
pub struct DefinitionStatement {
    pub parent: Option<Weak<Statement>>,
    pub span: Option<Span>,
    pub variables: Vec<Variable>,
    pub value: Arc<Expression>,
}

impl Node for DefinitionStatement {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl FromAst<leo_ast::DefinitionStatement> for Arc<Statement> {
    fn from_ast(
        scope: &Scope,
        statement: &leo_ast::DefinitionStatement,
        _expected_type: Option<PartialType>,
    ) -> Result<Arc<Statement>, AsgConvertError> {
        let type_ = statement
            .type_
            .as_ref()
            .map(|x| scope.borrow().resolve_ast_type(&x))
            .transpose()?;

        let value = Arc::<Expression>::from_ast(scope, &statement.value, type_.clone().map(Into::into))?;

        let type_ = type_.or_else(|| value.get_type());

        let mut output_types = vec![];

        let mut variables = vec![];
        if statement.variable_names.is_empty() {
            return Err(AsgConvertError::illegal_ast_structure(
                "cannot have 0 variable names in destructuring tuple",
            ));
        }
        if statement.variable_names.len() == 1 {
            // any return type is fine
            output_types.push(type_);
        } else {
            // tuple destructure
            match type_.as_ref() {
                Some(Type::Tuple(sub_types)) if sub_types.len() == statement.variable_names.len() => {
                    output_types.extend(sub_types.clone().into_iter().map(Some).collect::<Vec<_>>());
                }
                type_ => {
                    return Err(AsgConvertError::unexpected_type(
                        &*format!("{}-ary tuple", statement.variable_names.len()),
                        type_.map(|x| x.to_string()).as_deref(),
                        &statement.span,
                    ));
                }
            }
        }

        for (variable, type_) in statement.variable_names.iter().zip(output_types.into_iter()) {
            if statement.declaration_type == leo_ast::Declare::Const {
                return Err(AsgConvertError::AstError(AstError::DeprecatedError(
                    DeprecatedError::const_statement(&statement.span),
                )));
            }

            variables.push(Arc::new(RefCell::new(InnerVariable {
                id: uuid::Uuid::new_v4(),
                name: variable.identifier.clone(),
                type_: type_
                    .ok_or_else(|| AsgConvertError::unresolved_type(&variable.identifier.name, &statement.span))?
                    .weak(),
                mutable: variable.mutable,
                const_: false,
                declaration: crate::VariableDeclaration::Definition,
                references: vec![],
                assignments: vec![],
            })));
        }

        {
            let mut scope_borrow = scope.borrow_mut();
            for variable in variables.iter() {
                scope_borrow
                    .variables
                    .insert(variable.borrow().name.name.clone(), variable.clone());
            }
        }

        let statement = Arc::new(Statement::Definition(DefinitionStatement {
            parent: None,
            span: Some(statement.span.clone()),
            variables: variables.clone(),
            value,
        }));

        variables.iter().for_each(|variable| {
            variable.borrow_mut().assignments.push(Arc::downgrade(&statement));
        });

        Ok(statement)
    }
}

impl Into<leo_ast::DefinitionStatement> for &DefinitionStatement {
    fn into(self) -> leo_ast::DefinitionStatement {
        assert!(!self.variables.is_empty());

        let mut variable_names = vec![];
        let mut type_ = None::<leo_ast::Type>;
        for variable in self.variables.iter() {
            let variable = variable.borrow();
            variable_names.push(leo_ast::VariableName {
                mutable: variable.mutable,
                identifier: variable.name.clone(),
                span: variable.name.span.clone(),
            });
            if type_.is_none() {
                type_ = Some((&variable.type_.clone().strong()).into());
            }
        }

        leo_ast::DefinitionStatement {
            declaration_type: leo_ast::Declare::Let,
            variable_names,
            type_,
            value: self.value.as_ref().into(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
