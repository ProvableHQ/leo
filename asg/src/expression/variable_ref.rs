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
    ConstValue,
    Constant,
    DefinitionStatement,
    Expression,
    ExpressionNode,
    FromAst,
    Node,
    PartialType,
    Scope,
    Statement,
    Type,
    Variable,
};

use leo_errors::{AsgError, LeoError, Span};

use std::cell::Cell;

#[derive(Clone)]
pub struct VariableRef<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub variable: &'a Variable<'a>,
}

impl<'a> Node for VariableRef<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for VariableRef<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, _expr: &'a Expression<'a>) {}

    fn get_type(&self) -> Option<Type<'a>> {
        Some(self.variable.borrow().type_.clone())
    }

    fn is_mut_ref(&self) -> bool {
        self.variable.borrow().mutable
    }

    // todo: we can use use hacky ssa here to catch more cases, or just enforce ssa before asg generation finished
    fn const_value(&self) -> Option<ConstValue<'a>> {
        let variable = self.variable.borrow();
        if variable.mutable || variable.assignments.len() != 1 {
            return None;
        }
        let assignment = variable.assignments.get(0).unwrap();
        match &*assignment {
            Statement::Definition(DefinitionStatement { variables, value, .. }) => {
                if variables.len() == 1 {
                    let defined_variable = variables.get(0).unwrap().borrow();
                    assert_eq!(variable.id, defined_variable.id);

                    value.get().const_value()
                } else {
                    for (i, defined_variable) in variables.iter().enumerate() {
                        let defined_variable = defined_variable.borrow();
                        if defined_variable.id == variable.id {
                            match value.get().const_value() {
                                Some(ConstValue::Tuple(values)) => return values.get(i).cloned(),
                                None => return None,
                                _ => (),
                            }
                        }
                    }
                    panic!("no corresponding tuple variable found during const destructuring (corrupt asg?)");
                }
            }
            _ => None, //todo unroll loops during asg phase
        }
    }

    fn is_consty(&self) -> bool {
        let variable = self.variable.borrow();
        if variable.const_ {
            return true;
        }
        if variable.mutable || variable.assignments.len() != 1 {
            return false;
        }
        let assignment = variable.assignments.get(0).unwrap();

        match &*assignment {
            Statement::Definition(DefinitionStatement { variables, value, .. }) => {
                if variables.len() == 1 {
                    let defined_variable = variables.get(0).unwrap().borrow();
                    assert_eq!(variable.id, defined_variable.id);

                    value.get().is_consty()
                } else {
                    for defined_variable in variables.iter() {
                        let defined_variable = defined_variable.borrow();
                        if defined_variable.id == variable.id {
                            return value.get().is_consty();
                        }
                    }
                    panic!("no corresponding tuple variable found during const destructuring (corrupt asg?)");
                }
            }
            Statement::Iteration(_) => true,
            _ => false,
        }
    }
}

impl<'a> FromAst<'a, leo_ast::Identifier> for &'a Expression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::Identifier,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<&'a Expression<'a>, LeoError> {
        let variable = if value.name.as_ref() == "input" {
            if let Some(input) = scope.resolve_input() {
                input.container
            } else {
                return Err(AsgError::illegal_input_variable_reference(
                    "attempted to reference input when none is in scope",
                    &value.span,
                )
                .into());
            }
        } else {
            match scope.resolve_variable(&value.name) {
                Some(v) => v,
                None => {
                    if value.name.starts_with("aleo1") {
                        return Ok(scope.context.alloc_expression(Expression::Constant(Constant {
                            parent: Cell::new(None),
                            span: Some(value.span.clone()),
                            value: ConstValue::Address(value.name.clone()),
                        })));
                    }
                    return Err(AsgError::unresolved_reference(&value.name, &value.span).into());
                }
            }
        };

        let variable_ref = VariableRef {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            variable,
        };
        let expression = scope.context.alloc_expression(Expression::VariableRef(variable_ref));

        if let Some(expected_type) = expected_type {
            let type_ = expression
                .get_type()
                .ok_or_else(|| AsgError::unresolved_reference(&value.name, &value.span))?;
            if !expected_type.matches(&type_) {
                return Err(AsgError::unexpected_type(expected_type, type_, &value.span).into());
            }
        }

        let mut variable_ref = variable.borrow_mut();
        variable_ref.references.push(expression);

        Ok(expression)
    }
}

impl<'a> Into<leo_ast::Identifier> for &VariableRef<'a> {
    fn into(self) -> leo_ast::Identifier {
        self.variable.borrow().name.clone()
    }
}
