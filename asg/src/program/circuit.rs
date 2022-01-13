// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{AsgId, Expression, ExpressionNode as _, FromAst as _, Function, Identifier, Node, Scope, Type};

use leo_errors::{AsgError, Result};
use leo_span::{Span, Symbol};

use indexmap::IndexMap;
use std::cell::RefCell;

#[derive(Clone)]
pub enum CircuitMember<'a> {
    Const(&'a Expression<'a>),
    Variable(Type<'a>),
    Function(&'a Function<'a>),
}

impl<'a> CircuitMember<'a> {
    pub fn get_type(&self) -> Option<Type<'a>> {
        use CircuitMember::*;

        match self {
            Const(expr) => expr.get_type(),
            Variable(type_) => Some(type_.clone()),
            Function(function) => Some(function.output.clone()),
        }
    }
}

#[derive(Clone)]
pub struct Circuit<'a> {
    pub id: AsgId,
    pub name: RefCell<Identifier>,
    pub scope: &'a Scope<'a>,
    pub span: Option<Span>,
    pub members: RefCell<IndexMap<Symbol, CircuitMember<'a>>>,
}

impl PartialEq for Circuit<'_> {
    fn eq(&self, other: &Circuit) -> bool {
        if self.name != other.name {
            return false;
        }
        self.id == other.id
    }
}

impl Eq for Circuit<'_> {}

impl Node for Circuit<'_> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    fn asg_id(&self) -> AsgId {
        self.id
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

        Ok(circuit)
    }

    pub(super) fn init_member(scope: &'a Scope<'a>, value: &leo_ast::Circuit) -> Result<&'a Circuit<'a>> {
        let new_scope = scope.make_subscope();
        let circuits = scope.circuits.borrow();

        let circuit = circuits.get(&value.circuit_name.name).unwrap();

        let mut members = circuit.members.borrow_mut();

        for member in value.members.iter() {
            match member {
                leo_ast::CircuitMember::CircuitConst(name, type_, const_value) => {
                    if members.contains_key(&name.name) {
                        return Err(AsgError::redefined_circuit_member(
                            &value.circuit_name.name,
                            &name.name,
                            &name.span,
                        )
                        .into());
                    }
                    let type_ = new_scope.resolve_ast_type(type_, &name.span)?;
                    members.insert(
                        name.name,
                        CircuitMember::Const(<&Expression<'a>>::from_ast(new_scope, const_value, Some(type_.into()))?),
                    );
                }
                leo_ast::CircuitMember::CircuitFunction(function) => {
                    if members.contains_key(&function.identifier.name) {
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
                    members.insert(function.identifier.name, CircuitMember::Function(asg_function));
                }
                leo_ast::CircuitMember::CircuitVariable(name, type_) => {
                    if members.contains_key(&name.name) {
                        return Err(AsgError::redefined_circuit_member(
                            &value.circuit_name.name,
                            &name.name,
                            &name.span,
                        )
                        .into());
                    }
                    members.insert(
                        name.name,
                        CircuitMember::Variable(new_scope.resolve_ast_type(type_, &name.span)?),
                    );
                }
            }
        }

        Ok(circuit)
    }

    pub(super) fn fill_from_ast(self: &'a Circuit<'a>, value: &leo_ast::Circuit) -> Result<()> {
        for member in value.members.iter() {
            match member {
                leo_ast::CircuitMember::CircuitConst(..) => {}
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
                CircuitMember::Const(value) => leo_ast::CircuitMember::CircuitConst(
                    Identifier::new(*name),
                    value.get_type().as_ref().unwrap().into(),
                    (*value).into(),
                ),
                CircuitMember::Variable(type_) => {
                    leo_ast::CircuitMember::CircuitVariable(Identifier::new(*name), type_.into())
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
