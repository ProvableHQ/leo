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

use crate::{Expression, ExpressionNode, FromAst, Function, Identifier, Node, Scope, Type};
use leo_errors::{AsgError, Result, Span};

use indexmap::IndexMap;
use std::cell::RefCell;

#[derive(Clone)]
pub enum StructMember<'a> {
    Const(&'a Expression<'a>),
    Variable(Type<'a>),
    Function(&'a Function<'a>),
}

impl<'a> StructMember<'a> {
    pub fn get_type(&self) -> Option<Type<'a>> {
        use StructMember::*;

        match self {
            Const(expr) => expr.get_type(),
            Variable(type_) => Some(type_.clone()),
            Function(function) => Some(function.output.clone()),
        }
    }
}

#[derive(Clone)]
pub struct Struct<'a> {
    pub id: u32,
    pub name: RefCell<Identifier>,
    pub scope: &'a Scope<'a>,
    pub span: Option<Span>,
    pub members: RefCell<IndexMap<String, StructMember<'a>>>,
}

impl<'a> PartialEq for Struct<'a> {
    fn eq(&self, other: &Struct) -> bool {
        if self.name != other.name {
            return false;
        }
        self.id == other.id
    }
}

impl<'a> Eq for Struct<'a> {}

impl<'a> Node for Struct<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> Struct<'a> {
    pub(super) fn init(scope: &'a Scope<'a>, value: &leo_ast::Struct) -> Result<&'a Struct<'a>> {
        let new_scope = scope.make_subscope();

        let structure = scope.context.alloc_struct(Struct {
            id: scope.context.get_id(),
            name: RefCell::new(value.struct_name.clone()),
            members: RefCell::new(IndexMap::new()),
            span: Some(value.struct_name.span.clone()),
            scope: new_scope,
        });

        let mut members = structure.members.borrow_mut();
        for member in value.members.iter() {
            if let leo_ast::StructMember::StructConst(name, type_, const_value) = member {
                if members.contains_key(name.name.as_ref()) {
                    return Err(
                        AsgError::redefined_struct_member(&value.struct_name.name, &name.name, &name.span).into(),
                    );
                }
                let type_ = new_scope.resolve_ast_type(type_, &name.span)?;
                members.insert(
                    name.name.to_string(),
                    StructMember::Const(<&Expression<'a>>::from_ast(new_scope, const_value, Some(type_.into()))?),
                );
            } else if let leo_ast::StructMember::StructVariable(name, type_) = member {
                if members.contains_key(name.name.as_ref()) {
                    return Err(
                        AsgError::redefined_struct_member(&value.struct_name.name, &name.name, &name.span).into(),
                    );
                }
                members.insert(
                    name.name.to_string(),
                    StructMember::Variable(new_scope.resolve_ast_type(type_, &name.span)?),
                );
            }
        }

        Ok(structure)
    }

    pub(super) fn init_member(scope: &'a Scope<'a>, value: &leo_ast::Struct) -> Result<&'a Struct<'a>> {
        let new_scope = scope.make_subscope();
        let structs = scope.structs.borrow();

        let structure = structs.get(value.struct_name.name.as_ref()).unwrap();

        let mut members = structure.members.borrow_mut();
        for member in value.members.iter() {
            if let leo_ast::StructMember::StructFunction(function) = member {
                if members.contains_key(function.identifier.name.as_ref()) {
                    return Err(AsgError::redefined_struct_member(
                        &value.struct_name.name,
                        &function.identifier.name,
                        &function.identifier.span,
                    )
                    .into());
                }
                let asg_function = Function::init(new_scope, function)?;
                asg_function.structure.replace(Some(structure));
                if asg_function.is_test() {
                    return Err(AsgError::struct_test_function(&function.identifier.span).into());
                }
                members.insert(
                    function.identifier.name.to_string(),
                    StructMember::Function(asg_function),
                );
            }
        }

        Ok(structure)
    }

    pub(super) fn fill_from_ast(self: &'a Struct<'a>, value: &leo_ast::Struct) -> Result<()> {
        for member in value.members.iter() {
            match member {
                leo_ast::StructMember::StructConst(..) => {}
                leo_ast::StructMember::StructVariable(..) => {}
                leo_ast::StructMember::StructFunction(function) => {
                    let asg_function = match *self
                        .members
                        .borrow()
                        .get(function.identifier.name.as_ref())
                        .expect("missing header for defined struct function")
                    {
                        StructMember::Function(f) => f,
                        _ => unimplemented!(),
                    };
                    Function::fill_from_ast(asg_function, function)?;
                }
            }
        }
        Ok(())
    }
}

impl<'a> Into<leo_ast::Struct> for &Struct<'a> {
    fn into(self) -> leo_ast::Struct {
        let members = self
            .members
            .borrow()
            .iter()
            .map(|(name, member)| match &member {
                StructMember::Const(value) => leo_ast::StructMember::StructConst(
                    Identifier::new((&**name).into()),
                    value.get_type().as_ref().unwrap().into(),
                    (*value).into(),
                ),
                StructMember::Variable(type_) => {
                    leo_ast::StructMember::StructVariable(Identifier::new((&**name).into()), type_.into())
                }
                StructMember::Function(func) => leo_ast::StructMember::StructFunction(Box::new((*func).into())),
            })
            .collect();
        leo_ast::Struct {
            struct_name: self.name.borrow().clone(),
            members,
        }
    }
}
