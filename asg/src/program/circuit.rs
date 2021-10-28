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

use crate::{Function, Identifier, Node, Scope, Type};
use leo_errors::{AsgError, Result, Span};

use indexmap::IndexMap;
use std::cell::RefCell;

#[derive(Clone)]
pub enum CircuitMember<'a> {
    Variable(Type<'a>),
    Function(&'a Function<'a>),
}

#[derive(Clone)]
pub struct Circuit<'a> {
    pub id: u32,
    pub name: RefCell<Identifier>,
    pub scope: &'a Scope<'a>,
    pub span: Option<Span>,
    pub members: RefCell<IndexMap<String, CircuitMember<'a>>>,
}

impl<'a> PartialEq for Circuit<'a> {
    fn eq(&self, other: &Circuit) -> bool {
        if self.name != other.name {
            return false;
        }
        self.id == other.id
    }
}

impl<'a> Eq for Circuit<'a> {}

impl<'a> Node for Circuit<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> Circuit<'a> {
    pub(super) fn init(scope: &'a Scope<'a>, value: &leo_ast::Circuit) -> Result<&'a Circuit<'a>> {
        let new_scope = scope.make_subscope();

        let circuit = scope.context.alloc_circuit(Circuit {
            id: scope.context.get_id(),
            name: RefCell::new(value.circuit_name.clone()),
            members: RefCell::new(IndexMap::new()),
            span: Some(value.circuit_name.span.clone()),
            scope: new_scope,
        });

        let mut members = circuit.members.borrow_mut();
        for member in value.members.iter() {
            if let leo_ast::CircuitMember::CircuitVariable(name, type_) = member {
                if members.contains_key(name.name.as_ref()) {
                    return Err(
                        AsgError::redefined_circuit_member(&value.circuit_name.name, &name.name, &name.span).into(),
                    );
                }
                members.insert(
                    name.name.to_string(),
                    CircuitMember::Variable(new_scope.resolve_ast_type(type_, &name.span)?),
                );
            }
        }

        Ok(circuit)
    }

    pub(super) fn init_member(scope: &'a Scope<'a>, value: &leo_ast::Circuit) -> Result<&'a Circuit<'a>> {
        let new_scope = scope.make_subscope();
        let circuits = scope.circuits.borrow();

        let circuit = circuits.get(value.circuit_name.name.as_ref()).unwrap();

        let mut members = circuit.members.borrow_mut();
        for member in value.members.iter() {
            if let leo_ast::CircuitMember::CircuitFunction(function) = member {
                if members.contains_key(function.identifier.name.as_ref()) {
                    return Err(AsgError::redefined_circuit_member(
                        &value.circuit_name.name,
                        &function.identifier.name,
                        &function.identifier.span,
                    )
                    .into());
                }
                let asg_function = Function::init(new_scope, function)?;
                asg_function.circuit.replace(Some(circuit));
                if asg_function.is_test() {
                    return Err(AsgError::circuit_test_function(&function.identifier.span).into());
                }
                members.insert(
                    function.identifier.name.to_string(),
                    CircuitMember::Function(asg_function),
                );
            }
        }

        Ok(circuit)
    }

    pub(super) fn fill_from_ast(self: &'a Circuit<'a>, value: &leo_ast::Circuit) -> Result<()> {
        for member in value.members.iter() {
            match member {
                leo_ast::CircuitMember::CircuitVariable(..) => {}
                leo_ast::CircuitMember::CircuitFunction(function) => {
                    let asg_function = match *self
                        .members
                        .borrow()
                        .get(function.identifier.name.as_ref())
                        .expect("missing header for defined circuit function")
                    {
                        CircuitMember::Function(f) => f,
                        _ => unimplemented!(),
                    };
                    Function::fill_from_ast(asg_function, function)?;
                }
            }
        }
        Ok(())
    }
}

impl<'a> Into<leo_ast::Circuit> for &Circuit<'a> {
    fn into(self) -> leo_ast::Circuit {
        let members = self
            .members
            .borrow()
            .iter()
            .map(|(name, member)| match &member {
                CircuitMember::Variable(type_) => {
                    leo_ast::CircuitMember::CircuitVariable(Identifier::new((&**name).into()), type_.into())
                }
                CircuitMember::Function(func) => leo_ast::CircuitMember::CircuitFunction(Box::new((*func).into())),
            })
            .collect();
        leo_ast::Circuit {
            circuit_name: self.name.borrow().clone(),
            members,
        }
    }
}
