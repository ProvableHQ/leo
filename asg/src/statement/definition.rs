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
    AsgId, Expression, ExpressionNode, FromAst, InnerVariable, Node, PartialType, Scope, Statement, Type, Variable,
};
use leo_errors::{AsgError, Result, Span};

use std::cell::{Cell, RefCell};

#[derive(Clone)]
pub struct DefinitionStatement<'a> {
    pub id: AsgId,
    pub parent: Cell<Option<&'a Statement<'a>>>,
    pub span: Option<Span>,
    pub variables: Vec<&'a Variable<'a>>,
    pub value: Cell<&'a Expression<'a>>,
}

impl<'a> DefinitionStatement<'a> {
    pub fn split(&self, scope: &'a Scope<'a>) -> Vec<(String, Self)> {
        self.variables
            .iter()
            .map(|variable| {
                (
                    variable.borrow().name.name.to_string(),
                    DefinitionStatement {
                        id: scope.context.get_id(),
                        parent: self.parent.clone(),
                        span: self.span.clone(),
                        variables: vec![variable],
                        value: self.value.clone(),
                    },
                )
            })
            .collect()
    }
}

impl<'a> Node for DefinitionStatement<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    fn asg_id(&self) -> AsgId {
        self.id
    }
}

impl<'a> FromAst<'a, leo_ast::DefinitionStatement> for &'a Statement<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        statement: &leo_ast::DefinitionStatement,
        _expected_type: Option<PartialType<'a>>,
    ) -> Result<Self> {
        let type_ = statement
            .type_
            .as_ref()
            .map(|x| scope.resolve_ast_type(x, &statement.span))
            .transpose()?;

        let value = <&Expression<'a>>::from_ast(scope, &statement.value, type_.clone().map(Into::into))?;

        if matches!(statement.declaration_type, leo_ast::Declare::Const) && !value.is_consty() {
            let var_names = statement
                .variable_names
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ,");

            return Err(AsgError::invalid_const_assign(var_names, &statement.span).into());
        }

        let type_ = type_.or_else(|| value.get_type());

        let mut output_types = vec![];

        let mut variables = vec![];
        if statement.variable_names.is_empty() {
            return Err(AsgError::illegal_ast_structure(
                "cannot have 0 variable names in destructuring tuple",
                &statement.span,
            )
            .into());
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
                    return Err(AsgError::unexpected_type(
                        format!("{}-ary tuple", statement.variable_names.len()),
                        type_.map(|x| x.to_string()).unwrap_or_else(|| "unknown".to_string()),
                        &statement.span,
                    )
                    .into());
                }
            }
        }

        for (variable, type_) in statement.variable_names.iter().zip(output_types.into_iter()) {
            let name = variable.identifier.name.as_ref();
            if scope.resolve_global_const(name).is_some() {
                return Err(
                    AsgError::function_variable_cannot_shadow_global_const(name, &variable.identifier.span).into(),
                );
            } else if scope.resolve_variable(name).is_some() {
                return Err(AsgError::function_variable_cannot_shadow_other_function_variable(
                    name,
                    &variable.identifier.span,
                )
                .into());
            }

            variables.push(&*scope.context.alloc_variable(RefCell::new(InnerVariable {
                id: scope.context.get_id(),
                name: variable.identifier.clone(),
                type_: type_.ok_or_else(|| AsgError::unresolved_type(&variable.identifier.name, &statement.span))?,
                mutable: variable.mutable,
                const_: false,
                declaration: crate::VariableDeclaration::Definition,
                references: vec![],
                assignments: vec![],
            })));
        }

        for variable in variables.iter() {
            let mut variables = scope.variables.borrow_mut();
            let var_name = variable.borrow().name.name.to_string();
            if variables.contains_key(&var_name) {
                return Err(AsgError::duplicate_variable_definition(var_name, &statement.span).into());
            }

            variables.insert(var_name, *variable);
        }

        let statement = scope
            .context
            .alloc_statement(Statement::Definition(DefinitionStatement {
                id: scope.context.get_id(),
                parent: Cell::new(None),
                span: Some(statement.span.clone()),
                variables: variables.clone(),
                value: Cell::new(value),
            }));

        for variable in variables {
            variable.borrow_mut().assignments.push(statement);
        }

        Ok(statement)
    }
}

impl<'a> Into<leo_ast::DefinitionStatement> for &DefinitionStatement<'a> {
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
                type_ = Some((&variable.type_.clone()).into());
            }
        }

        leo_ast::DefinitionStatement {
            declaration_type: leo_ast::Declare::Let,
            variable_names,
            type_,
            value: self.value.get().into(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
