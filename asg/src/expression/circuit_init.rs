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

use leo_errors::{AsgError, LeoError, Span};

use indexmap::{IndexMap, IndexSet};
use std::cell::Cell;

#[derive(Clone)]
pub struct CircuitInitExpression<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub circuit: Cell<&'a Circuit<'a>>,
    pub values: Vec<(Identifier, Cell<&'a Expression<'a>>)>,
}

impl<'a> Node for CircuitInitExpression<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for CircuitInitExpression<'a> {
    fn set_parent(&self, parent: &'a Expression<'a>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<&'a Expression<'a>> {
        self.parent.get()
    }

    fn enforce_parents(&self, expr: &'a Expression<'a>) {
        self.values.iter().for_each(|(_, element)| {
            element.get().set_parent(expr);
        })
    }

    fn get_type(&self) -> Option<Type<'a>> {
        Some(Type::Circuit(self.circuit.get()))
    }

    fn is_mut_ref(&self) -> bool {
        true
    }

    fn const_value(&self) -> Option<ConstValue<'a>> {
        let mut members = IndexMap::new();
        for (identifier, member) in self.values.iter() {
            // insert by name because accessmembers identifiers are different.
            members.insert(
                identifier.name.to_string(),
                (identifier.clone(), member.get().const_value()?),
            );
        }
        // Store circuit as well for get_type.
        Some(ConstValue::Circuit(self.circuit.get(), members))
    }

    fn is_consty(&self) -> bool {
        self.values.iter().all(|(_, value)| value.get().is_consty())
    }
}

impl<'a> FromAst<'a, leo_ast::CircuitInitExpression> for CircuitInitExpression<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::CircuitInitExpression,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<CircuitInitExpression<'a>, LeoError> {
        let circuit = scope
            .resolve_circuit(&value.name.name)
            .ok_or_else(|| AsgError::unresolved_circuit(&value.name.name, &value.name.span))?;
        match expected_type {
            Some(PartialType::Type(Type::Circuit(expected_circuit))) if expected_circuit == circuit => (),
            None => (),
            Some(x) => {
                return Err(AsgError::unexpected_type(
                    x,
                    circuit.name.borrow().name.to_string(),
                    &value.span,
                ))?;
            }
        }
        let members: IndexMap<&str, (&Identifier, Option<&leo_ast::Expression>)> = value
            .members
            .iter()
            .map(|x| (x.identifier.name.as_ref(), (&x.identifier, x.expression.as_ref())))
            .collect();

        let mut values: Vec<(Identifier, Cell<&'a Expression<'a>>)> = vec![];
        let mut defined_variables = IndexSet::<String>::new();

        {
            let circuit_members = circuit.members.borrow();
            for (name, member) in circuit_members.iter() {
                if defined_variables.contains(name) {
                    return Err(AsgError::overridden_circuit_member(
                        &circuit.name.borrow().name,
                        name,
                        &value.span,
                    ))?;
                }
                defined_variables.insert(name.clone());
                let type_: Type = if let CircuitMember::Variable(type_) = &member {
                    type_.clone()
                } else {
                    continue;
                };
                if let Some((identifier, receiver)) = members.get(&**name) {
                    let received = if let Some(receiver) = *receiver {
                        <&Expression<'a>>::from_ast(scope, receiver, Some(type_.partial()))?
                    } else {
                        <&Expression<'a>>::from_ast(
                            scope,
                            &leo_ast::Expression::Identifier((*identifier).clone()),
                            Some(type_.partial()),
                        )?
                    };
                    values.push(((*identifier).clone(), Cell::new(received)));
                } else {
                    return Err(AsgError::missing_circuit_member(
                        &circuit.name.borrow().name,
                        name,
                        &value.span,
                    ))?;
                }
            }

            for (name, (identifier, _expression)) in members.iter() {
                if circuit_members.get(*name).is_none() {
                    return Err(AsgError::extra_circuit_member(
                        &circuit.name.borrow().name,
                        name,
                        &identifier.span,
                    ))?;
                }
            }
        }

        Ok(CircuitInitExpression {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            circuit: Cell::new(circuit),
            values,
        })
    }
}

impl<'a> Into<leo_ast::CircuitInitExpression> for &CircuitInitExpression<'a> {
    fn into(self) -> leo_ast::CircuitInitExpression {
        leo_ast::CircuitInitExpression {
            name: self.circuit.get().name.borrow().clone(),
            members: self
                .values
                .iter()
                .map(|(name, value)| leo_ast::CircuitImpliedVariableDefinition {
                    identifier: name.clone(),
                    expression: Some(value.get().into()),
                })
                .collect(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
