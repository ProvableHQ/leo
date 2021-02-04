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
    ConstValue,
    Constant,
    DefinitionStatement,
    Expression,
    ExpressionNode,
    FromAst,
    Node,
    PartialType,
    Scope,
    Span,
    Statement,
    Type,
    Variable,
};

use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

pub struct VariableRef {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub variable: Variable,
}

impl Node for VariableRef {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for VariableRef {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, _expr: &Arc<Expression>) {}

    fn get_type(&self) -> Option<Type> {
        Some(self.variable.borrow().type_.clone().strong().clone())
    }

    fn is_mut_ref(&self) -> bool {
        self.variable.borrow().mutable
    }

    // todo: we can use use hacky ssa here to catch more cases, or just enforce ssa before asg generation finished
    fn const_value(&self) -> Option<ConstValue> {
        let variable = self.variable.borrow();
        if variable.mutable || variable.assignments.len() != 1 {
            return None;
        }
        let assignment = variable
            .assignments
            .get(0)
            .unwrap()
            .upgrade()
            .expect("stale assignment for variable");
        match &*assignment {
            Statement::Definition(DefinitionStatement { variables, value, .. }) => {
                if variables.len() == 1 {
                    let defined_variable = variables.get(0).unwrap().borrow();
                    assert_eq!(variable.id, defined_variable.id);

                    value.const_value()
                } else {
                    for defined_variable in variables.iter() {
                        let defined_variable = defined_variable.borrow();
                        if defined_variable.id == variable.id {
                            return value.const_value();
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
        let assignment = variable
            .assignments
            .get(0)
            .unwrap()
            .upgrade()
            .expect("stale assignment for variable");

        match &*assignment {
            Statement::Definition(DefinitionStatement { variables, value, .. }) => {
                if variables.len() == 1 {
                    let defined_variable = variables.get(0).unwrap().borrow();
                    assert_eq!(variable.id, defined_variable.id);

                    value.is_consty()
                } else {
                    for defined_variable in variables.iter() {
                        let defined_variable = defined_variable.borrow();
                        if defined_variable.id == variable.id {
                            return value.is_consty();
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

impl FromAst<leo_ast::Identifier> for Arc<Expression> {
    fn from_ast(
        scope: &Scope,
        value: &leo_ast::Identifier,
        expected_type: Option<PartialType>,
    ) -> Result<Arc<Expression>, AsgConvertError> {
        let variable = if value.name == "input" {
            if let Some(function) = scope.borrow().resolve_current_function() {
                if !function.has_input {
                    return Err(AsgConvertError::unresolved_reference(&value.name, &value.span));
                }
            } else {
                return Err(AsgConvertError::unresolved_reference(&value.name, &value.span));
            }
            if let Some(input) = scope.borrow().resolve_input() {
                input.container
            } else {
                return Err(AsgConvertError::InternalError(
                    "attempted to reference input when none is in scope".to_string(),
                ));
            }
        } else {
            match scope.borrow().resolve_variable(&value.name) {
                Some(v) => v,
                None => {
                    if value.name.starts_with("aleo1") {
                        return Ok(Arc::new(Expression::Constant(Constant {
                            parent: RefCell::new(None),
                            span: Some(value.span.clone()),
                            value: ConstValue::Address(value.name.clone()),
                        })));
                    }
                    return Err(AsgConvertError::unresolved_reference(&value.name, &value.span));
                }
            }
        };

        let variable_ref = VariableRef {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            variable: variable.clone(),
        };
        let expression = Arc::new(Expression::VariableRef(variable_ref));

        if let Some(expected_type) = expected_type {
            let type_ = expression
                .get_type()
                .ok_or_else(|| AsgConvertError::unresolved_reference(&value.name, &value.span))?;
            if !expected_type.matches(&type_) {
                return Err(AsgConvertError::unexpected_type(
                    &expected_type.to_string(),
                    Some(&*type_.to_string()),
                    &value.span,
                ));
            }
        }

        let mut variable_ref = variable.borrow_mut();
        variable_ref.references.push(Arc::downgrade(&expression));

        Ok(expression)
    }
}

impl Into<leo_ast::Identifier> for &VariableRef {
    fn into(self) -> leo_ast::Identifier {
        self.variable.borrow().name.clone()
    }
}
