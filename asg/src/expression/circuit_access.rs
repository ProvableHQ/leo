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

use crate::AsgConvertError;
use crate::Circuit;
use crate::CircuitMember;
use crate::ConstValue;
use crate::Expression;
use crate::ExpressionNode;
use crate::FromAst;
use crate::Identifier;
use crate::Node;
use crate::PartialType;
use crate::Scope;
use crate::Span;
use crate::Type;

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
            let member = members.get(&self.member.name)?;
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

    fn const_value(&self) -> Option<ConstValue> {
        None
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
    ) -> Result<CircuitAccessExpression<'a>, AsgConvertError> {
        let target = <&'a Expression<'a>>::from_ast(scope, &*value.circuit, None)?;
        let circuit = match target.get_type() {
            Some(Type::Circuit(circuit)) => circuit,
            x => {
                return Err(AsgConvertError::unexpected_type(
                    "circuit",
                    x.map(|x| x.to_string()).as_deref(),
                    &value.span,
                ));
            }
        };

        // scoping refcell reference
        let found_member = {
            if let Some(member) = circuit.members.borrow().get(&value.name.name) {
                if let Some(expected_type) = &expected_type {
                    if let CircuitMember::Variable(type_) = &member {
                        let type_: Type = type_.clone();
                        if !expected_type.matches(&type_) {
                            return Err(AsgConvertError::unexpected_type(
                                &expected_type.to_string(),
                                Some(&type_.to_string()),
                                &value.span,
                            ));
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
                circuit
                    .members
                    .borrow_mut()
                    .insert(value.name.name.clone(), CircuitMember::Variable(expected_type.clone()));
            } else {
                return Err(AsgConvertError::input_ref_needs_type(
                    &circuit.name.borrow().name,
                    &value.name.name,
                    &value.span,
                ));
            }
        } else {
            return Err(AsgConvertError::unresolved_circuit_member(
                &circuit.name.borrow().name,
                &value.name.name,
                &value.span,
            ));
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
    ) -> Result<CircuitAccessExpression<'a>, AsgConvertError> {
        let circuit = match &*value.circuit {
            leo_ast::Expression::Identifier(name) => scope
                .resolve_circuit(&name.name)
                .ok_or_else(|| AsgConvertError::unresolved_circuit(&name.name, &name.span))?,
            _ => {
                return Err(AsgConvertError::unexpected_type(
                    "circuit",
                    Some("unknown"),
                    &value.span,
                ));
            }
        };

        if let Some(expected_type) = expected_type {
            return Err(AsgConvertError::unexpected_type(
                &expected_type.to_string(),
                Some("none"),
                &value.span,
            ));
        }

        if let Some(CircuitMember::Function(_)) = circuit.members.borrow().get(&value.name.name) {
            // okay
        } else {
            return Err(AsgConvertError::unresolved_circuit_member(
                &circuit.name.borrow().name,
                &value.name.name,
                &value.span,
            ));
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
