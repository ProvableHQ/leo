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
    ConstValue, Expression, ExpressionNode, FromAst, Identifier, Node, PartialType, Scope, Struct, StructMember, Type,
};

use leo_errors::{AsgError, Result, Span};
use std::cell::Cell;

#[derive(Clone)]
pub struct StructAccess<'a> {
    pub parent: Cell<Option<&'a Expression<'a>>>,
    pub span: Option<Span>,
    pub structure: Cell<&'a Struct<'a>>,
    pub target: Cell<Option<&'a Expression<'a>>>,
    pub member: Identifier,
}

impl<'a> Node for StructAccess<'a> {
    fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

impl<'a> ExpressionNode<'a> for StructAccess<'a> {
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
        let members = self.structure.get().members.borrow();
        let member = members.get(self.member.name.as_ref())?;
        match member {
            StructMember::Const(value) => value.get_type(),
            StructMember::Variable(type_) => Some(type_.clone()),
            StructMember::Function(_) => None,
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
            ConstValue::Struct(_, members) => {
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

impl<'a> FromAst<'a, leo_ast::accesses::MemberAccess> for StructAccess<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::accesses::MemberAccess,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<StructAccess<'a>> {
        let target = <&'a Expression<'a>>::from_ast(scope, &*value.inner, None)?;
        let structure = match target.get_type() {
            Some(Type::Struct(structure)) => structure,
            x => {
                return Err(AsgError::unexpected_type(
                    "struct",
                    x.map(|x| x.to_string()).unwrap_or_else(|| "unknown".to_string()),
                    &value.span,
                )
                .into());
            }
        };

        // scoping refcell reference
        let found_member = {
            if let Some(member) = structure.members.borrow().get(value.name.name.as_ref()) {
                if let Some(expected_type) = &expected_type {
                    if let StructMember::Variable(type_) = &member {
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
        } else if structure.is_input_pseudo_struct() {
            // add new member to implicit input
            if let Some(expected_type) = expected_type.map(PartialType::full).flatten() {
                structure.members.borrow_mut().insert(
                    value.name.name.to_string(),
                    StructMember::Variable(expected_type.clone()),
                );
            } else {
                return Err(AsgError::input_ref_needs_type(
                    &structure.name.borrow().name,
                    &value.name.name,
                    &value.span,
                )
                .into());
            }
        } else {
            return Err(AsgError::unresolved_struct_member(
                &structure.name.borrow().name,
                &value.name.name,
                &value.span,
            )
            .into());
        }

        Ok(StructAccess {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            target: Cell::new(Some(target)),
            structure: Cell::new(structure),
            member: value.name.clone(),
        })
    }
}

impl<'a> FromAst<'a, leo_ast::accesses::StaticAccess> for StructAccess<'a> {
    fn from_ast(
        scope: &'a Scope<'a>,
        value: &leo_ast::accesses::StaticAccess,
        expected_type: Option<PartialType<'a>>,
    ) -> Result<StructAccess<'a>> {
        let structure = match &*value.inner {
            leo_ast::Expression::Identifier(name) => scope
                .resolve_struct(&name.name)
                .ok_or_else(|| AsgError::unresolved_struct(&name.name, &name.span))?,
            _ => {
                return Err(AsgError::unexpected_type("struct", "unknown", &value.span).into());
            }
        };

        let member_type = structure
            .members
            .borrow()
            .get(value.name.name.as_ref())
            .map(|m| m.get_type())
            .flatten();
        match (expected_type, member_type) {
            (Some(expected_type), Some(type_)) if !expected_type.matches(&type_) => {
                return Err(AsgError::unexpected_type(expected_type, type_, &value.span).into());
            }
            _ => {}
        }

        Ok(StructAccess {
            parent: Cell::new(None),
            span: Some(value.span.clone()),
            target: Cell::new(None),
            structure: Cell::new(structure),
            member: value.name.clone(),
        })
    }
}

impl<'a> Into<leo_ast::Expression> for &StructAccess<'a> {
    fn into(self) -> leo_ast::Expression {
        if let Some(target) = self.target.get() {
            leo_ast::Expression::Access(leo_ast::AccessExpression::Member(leo_ast::accesses::MemberAccess {
                inner: Box::new(target.into()),
                name: self.member.clone(),
                span: self.span.clone().unwrap_or_default(),
                type_: None,
            }))
        } else {
            leo_ast::Expression::Access(leo_ast::AccessExpression::Static(leo_ast::accesses::StaticAccess {
                inner: Box::new(leo_ast::Expression::Identifier(
                    self.structure.get().name.borrow().clone(),
                )),
                name: self.member.clone(),
                type_: None,
                span: self.span.clone().unwrap_or_default(),
            }))
        }
    }
}
