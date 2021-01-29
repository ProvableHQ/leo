// Copyright (C) 2019-2020 Aleo Systems Inc.
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
    Span,
    Type,
};

use indexmap::{IndexMap, IndexSet};
use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

pub struct CircuitInitExpression {
    pub parent: RefCell<Option<Weak<Expression>>>,
    pub span: Option<Span>,
    pub circuit: Arc<Circuit>,
    pub values: Vec<(Identifier, Arc<Expression>)>,
}

impl Node for CircuitInitExpression {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl ExpressionNode for CircuitInitExpression {
    fn set_parent(&self, parent: Weak<Expression>) {
        self.parent.replace(Some(parent));
    }

    fn get_parent(&self) -> Option<Arc<Expression>> {
        self.parent.borrow().as_ref().map(Weak::upgrade).flatten()
    }

    fn enforce_parents(&self, expr: &Arc<Expression>) {
        self.values.iter().for_each(|(_, element)| {
            element.set_parent(Arc::downgrade(expr));
        })
    }

    fn get_type(&self) -> Option<Type> {
        Some(Type::Circuit(self.circuit.clone()))
    }

    fn is_mut_ref(&self) -> bool {
        false
    }

    fn const_value(&self) -> Option<ConstValue> {
        None
    }

    fn is_consty(&self) -> bool {
        self.values.iter().all(|(_, value)| value.is_consty())
    }
}

impl FromAst<leo_ast::CircuitInitExpression> for CircuitInitExpression {
    fn from_ast(
        scope: &Scope,
        value: &leo_ast::CircuitInitExpression,
        expected_type: Option<PartialType>,
    ) -> Result<CircuitInitExpression, AsgConvertError> {
        let circuit = scope
            .borrow()
            .resolve_circuit(&value.name.name)
            .ok_or_else(|| AsgConvertError::unresolved_circuit(&value.name.name, &value.name.span))?;
        match expected_type {
            Some(PartialType::Type(Type::Circuit(expected_circuit))) if expected_circuit == circuit => (),
            None => (),
            Some(x) => {
                return Err(AsgConvertError::unexpected_type(
                    &x.to_string(),
                    Some(&circuit.name.borrow().name),
                    &value.span,
                ));
            }
        }
        let members: IndexMap<&String, (&Identifier, &leo_ast::Expression)> = value
            .members
            .iter()
            .map(|x| (&x.identifier.name, (&x.identifier, &x.expression)))
            .collect();

        let mut values: Vec<(Identifier, Arc<Expression>)> = vec![];
        let mut defined_variables = IndexSet::<String>::new();

        {
            let circuit_members = circuit.members.borrow();
            for (name, member) in circuit_members.iter() {
                if defined_variables.contains(name) {
                    return Err(AsgConvertError::overridden_circuit_member(
                        &circuit.name.borrow().name,
                        name,
                        &value.span,
                    ));
                }
                defined_variables.insert(name.clone());
                let type_: Type = if let CircuitMember::Variable(type_) = &member {
                    type_.clone().into()
                } else {
                    continue;
                };
                if let Some((identifier, receiver)) = members.get(&name) {
                    let received = Arc::<Expression>::from_ast(scope, *receiver, Some(type_.partial()))?;
                    values.push(((*identifier).clone(), received));
                } else {
                    return Err(AsgConvertError::missing_circuit_member(
                        &circuit.name.borrow().name,
                        name,
                        &value.span,
                    ));
                }
            }

            for (name, (identifier, _expression)) in members.iter() {
                if circuit_members.get(*name).is_none() {
                    return Err(AsgConvertError::extra_circuit_member(
                        &circuit.name.borrow().name,
                        *name,
                        &identifier.span,
                    ));
                }
            }
        }

        Ok(CircuitInitExpression {
            parent: RefCell::new(None),
            span: Some(value.span.clone()),
            circuit,
            values,
        })
    }
}

impl Into<leo_ast::CircuitInitExpression> for &CircuitInitExpression {
    fn into(self) -> leo_ast::CircuitInitExpression {
        leo_ast::CircuitInitExpression {
            name: self.circuit.name.borrow().clone(),
            members: self
                .values
                .iter()
                .map(|(name, value)| leo_ast::CircuitVariableDefinition {
                    identifier: name.clone(),
                    expression: value.as_ref().into(),
                })
                .collect(),
            span: self.span.clone().unwrap_or_default(),
        }
    }
}
