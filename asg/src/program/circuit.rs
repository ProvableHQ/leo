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

use crate::{AsgConvertError, Function, Identifier, Node, Scope, Span, Type};

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
    pub core_mapping: RefCell<Option<String>>,
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
    pub(super) fn init(scope: &'a Scope<'a>, value: &leo_ast::Circuit) -> Result<&'a Circuit<'a>, AsgConvertError> {
        let new_scope = scope.make_subscope();

        let circuit = scope.alloc_circuit(Circuit {
            id: scope.context.get_id(),
            name: RefCell::new(value.circuit_name.clone()),
            members: RefCell::new(IndexMap::new()),
            core_mapping: RefCell::new(None),
            span: Some(value.circuit_name.span.clone()),
            scope: new_scope,
        });
        new_scope.circuit_self.replace(Some(circuit));

        let mut members = circuit.members.borrow_mut();
        for member in value.members.iter() {
            match member {
                leo_ast::CircuitMember::CircuitVariable(name, type_) => {
                    if members.contains_key(&name.name) {
                        return Err(AsgConvertError::redefined_circuit_member(
                            &value.circuit_name.name,
                            &name.name,
                            &name.span,
                        ));
                    }
                    members.insert(
                        name.name.clone(),
                        CircuitMember::Variable(new_scope.resolve_ast_type(type_)?),
                    );
                }
                leo_ast::CircuitMember::CircuitFunction(function) => {
                    if members.contains_key(&function.identifier.name) {
                        return Err(AsgConvertError::redefined_circuit_member(
                            &value.circuit_name.name,
                            &function.identifier.name,
                            &function.identifier.span,
                        ));
                    }
                    let asg_function = Function::init(new_scope, function)?;
                    asg_function.circuit.replace(Some(circuit));
                    if asg_function.is_test() {
                        return Err(AsgConvertError::circuit_test_function(&function.identifier.span));
                    }
                    members.insert(function.identifier.name.clone(), CircuitMember::Function(asg_function));
                }
            }
        }

        Ok(circuit)
    }

    pub(super) fn fill_from_ast(self: &'a Circuit<'a>, value: &leo_ast::Circuit) -> Result<(), AsgConvertError> {
        for member in value.members.iter() {
            match member {
                leo_ast::CircuitMember::CircuitVariable(..) => {}
                leo_ast::CircuitMember::CircuitFunction(function) => {
                    let asg_function = match *self
                        .members
                        .borrow()
                        .get(&function.identifier.name)
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
                    leo_ast::CircuitMember::CircuitVariable(Identifier::new(name.clone()), type_.into())
                }
                CircuitMember::Function(func) => leo_ast::CircuitMember::CircuitFunction((*func).into()),
            })
            .collect();
        leo_ast::Circuit {
            circuit_name: self.name.borrow().clone(),
            members,
        }
    }
}
