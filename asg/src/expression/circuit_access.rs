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
    Circuit,
    CircuitMember,
    ConstValue,
    Expression,
    ExpressionNode,
    FromAst,
    Identifier,
    Node,
    PartialType,
    Scope,
    Type,
};

use leo_errors::{AsgError, Result, Span};
use std::cell::Cell;

#[derive(Clone)]
pub struct CircuitAccessExpression<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub circuit: Cell<&'a Circuit<'a>>,
    pub target: Cell<Option<&'a Expression<'a>>>,
    pub member: Identifier,
}

impl<'a> Node for CircuitAccessExpression<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for CircuitAccessExpression<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        if let Some(target) = self.target.get() {
            target.set_parent(expr);
        }
    }

    fn get_type(&self) -> Option<Type<'a>> {
        if self.target.get().is_none() {
            None // function target only for static
        } else {
            let members = self.circuit.get().members.borrow();
            let member = members.get(self.member.name.as_ref())?;
            match member {
                CircuitMember::Variable(type_) => Some(type_.clone()),
                CircuitMember::Function(_) => None,
            }
        }
    }

    fn is_mut_ref(&self) -> bool {
        if let Some(target) = self.target.get() {
            target.is_mut_ref()
        } else {
            false
        }
    }

    fn const_value(&self) -> Option<ConstValue<'a>> {
        match self.target.get()?.const_value()? {
            ConstValue::Circuit(_, members) => {
                let (_, const_value) = members.get(&self.member.name.to_string())?.clone();
                Some(const_value)
            }
            _ => None,
        }
    }

    fn is_consty(&self) -> bool {
        self.target.get().map(|x| x.is_consty()).unwrap_or(true)
    }
}

impl<'a> FromAst<'a, leo_ast::CircuitMemberAccessExpression> for CircuitAccessExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::CircuitMemberAccessExpression,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<CircuitAccessExpression<'a>> {
        let target = <&'a Expression<'a>>::from_ast(scope, &*value.circuit, None)?;
        let circuit = match target.get_type() {
            Some(Type::Circuit(circuit)) => circuit,
            x => {
                return Err(AsgError::unexpected_type(
                    "circuit",
                    x.map(|x| x.to_string()).unwrap_or_else(|| "unknown".to_string()),
                    &value.span,
                )
                .into());
            }
        };

        // scoping refcell reference
        let found_member = {
            if let Some(member) = circuit.members.borrow().get(value.name.name.as_ref()) {
                if let Some(expected_type) = &expected_type {
                    if let CircuitMember::Variable(type_) = &member {
                        let type_: Type = type_.clone();
                        if !expected_type.matches(&type_) {
                            return Err(AsgError::unexpected_type(expected_type, type_, &value.span).into());
                        }
                    } // used by call expression
                }
                true
            } else {
                false
            }
        };

        if found_member {
            // skip
        } else if circuit.is_input_pseudo_circuit() {
            // add new member to implicit input
            if let Some(expected_type) = expected_type.map(PartialType::full).flatten() {
                circuit.members.borrow_mut().insert(
                    value.name.name.to_string(),
                    CircuitMember::Variable(expected_type.clone()),
                );
            } else {
                return Err(
                    AsgError::input_ref_needs_type(&circuit.name.borrow().name, &value.name.name, &value.span).into(),
                );
            }
        } else {
            return Err(AsgError::unresolved_circuit_member(
                &circuit.name.borrow().name,
                &value.name.name,
                &value.span,
            )
            .into());
        }

        Ok(CircuitAccessExpression {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            target: Cell::new(Some(target)),
            circuit: Cell::new(circuit),
            member: value.name.clone(),
        })
    }
}

impl<'a> FromAst<'a, leo_ast::CircuitStaticFunctionAccessExpression> for CircuitAccessExpression<'a> {
    fn from_ast(
        scope: &Scope<'a>,
        value: &leo_ast::CircuitStaticFunctionAccessExpression,
        expected_type: Option<PartialType>,
    ) -> Result<CircuitAccessExpression<'a>> {
        let circuit = match &*value.circuit {
            leo_ast::Expression::Identifier(name) => scope
                .resolve_circuit(&name.name)
                .ok_or_else(|| AsgError::unresolved_circuit(&name.name, &name.span))?,
            _ => {
                return Err(AsgError::unexpected_type("circuit", "unknown", &value.span).into());
            }
        };

        if let Some(expected_type) = expected_type {
            return Err(AsgError::unexpected_type(expected_type, "none", &value.span).into());
        }

        if let Some(CircuitMember::Function(_)) = circuit.members.borrow().get(value.name.name.as_ref()) {
            // okay
        } else {
            return Err(AsgError::unresolved_circuit_member(
                &circuit.name.borrow().name,
                &value.name.name,
                &value.span,
            )
            .into());
        }

        Ok(CircuitAccessExpression {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            target: Cell::new(None),
            circuit: Cell::new(circuit),
            member: value.name.clone(),
        })
    }
}

impl<'a> Into<leo_ast::Expression> for &CircuitAccessExpression<'a> {
    fn into(self) -> leo_ast::Expression {
        if let Some(target) = self.target.get() {
            leo_ast::Expression::CircuitMemberAccess(leo_ast::CircuitMemberAccessExpression {
                circuit: Box::new(target.into()),
                name: self.member.clone(),
                span: self.span.clone().unwrap_or_default(),
            })
        } else {
            leo_ast::Expression::CircuitStaticFunctionAccess(leo_ast::CircuitStaticFunctionAccessExpression {
                circuit: Box::new(leo_ast::Expression::Identifier(
                    self.circuit.get().name.borrow().clone(),
                )),
                name: self.member.clone(),
                span: self.span.clone().unwrap_or_default(),
            })
        }
    }
}
