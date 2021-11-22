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

use crate::{Identifier, Scope, Struct, StructMember, Type, Variable};
use leo_errors::Span;

use indexmap::IndexMap;
use std::cell::RefCell;

/// Stores program input values as ASG nodes.
#[derive(Clone, Copy)]
pub struct Input<'a> {
    pub registers: &'a Struct<'a>,
    pub state: &'a Struct<'a>,
    pub state_leaf: &'a Struct<'a>,
    pub record: &'a Struct<'a>,
    pub container_struct: &'a Struct<'a>,
    pub container: &'a Variable<'a>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InputCategory {
    MainInput,
    ConstInput,
    Register,
    PublicState,
    StateRecord,
    StateLeaf,
}

pub const CONTAINER_PSEUDO_STRUCT: &str = "$InputContainer";
pub const REGISTERS_PSEUDO_STRUCT: &str = "$InputRegister";
pub const RECORD_PSEUDO_STRUCT: &str = "$InputRecord";
pub const STATE_PSEUDO_STRUCT: &str = "$InputState";
pub const STATE_LEAF_PSEUDO_STRUCT: &str = "$InputStateLeaf";

impl<'a> Input<'a> {
    fn make_header(scope: &'a Scope<'a>, name: &str) -> &'a Struct<'a> {
        scope.context.alloc_struct(Struct {
            id: scope.context.get_id(),
            name: RefCell::new(Identifier::new(name.into())),
            members: RefCell::new(IndexMap::new()),
            scope,
            span: Some(Span::default()),
        })
    }

    pub fn new(scope: &'a Scope<'a>) -> Self {
        let input_scope = scope.make_subscope();
        let registers = Self::make_header(input_scope, REGISTERS_PSEUDO_STRUCT);
        let record = Self::make_header(input_scope, RECORD_PSEUDO_STRUCT);
        let state = Self::make_header(input_scope, STATE_PSEUDO_STRUCT);
        let state_leaf = Self::make_header(input_scope, STATE_LEAF_PSEUDO_STRUCT);

        let mut container_members = IndexMap::new();
        container_members.insert("registers".to_string(), StructMember::Variable(Type::Struct(registers)));
        container_members.insert("record".to_string(), StructMember::Variable(Type::Struct(record)));
        container_members.insert("state".to_string(), StructMember::Variable(Type::Struct(state)));
        container_members.insert(
            "state_leaf".to_string(),
            StructMember::Variable(Type::Struct(state_leaf)),
        );

        let container_struct = input_scope.context.alloc_struct(Struct {
            id: scope.context.get_id(),
            name: RefCell::new(Identifier::new(CONTAINER_PSEUDO_STRUCT.into())),
            members: RefCell::new(container_members),
            scope: input_scope,
            span: Some(Span::default()),
        });

        Input {
            registers,
            record,
            state,
            state_leaf,
            container_struct,
            container: input_scope.context.alloc_variable(RefCell::new(crate::InnerVariable {
                id: scope.context.get_id(),
                name: Identifier::new("input".into()),
                type_: Type::Struct(container_struct),
                mutable: false,
                const_: false,
                declaration: crate::VariableDeclaration::Input,
                references: vec![],
                assignments: vec![],
            })),
        }
    }
}

impl<'a> Struct<'a> {
    pub fn is_input_pseudo_struct(&self) -> bool {
        matches!(
            &*self.name.borrow().name,
            REGISTERS_PSEUDO_STRUCT | RECORD_PSEUDO_STRUCT | STATE_PSEUDO_STRUCT | STATE_LEAF_PSEUDO_STRUCT
        )
    }

    pub fn input_type(&self) -> Option<InputCategory> {
        match self.name.borrow().name.as_ref() {
            REGISTERS_PSEUDO_STRUCT => Some(InputCategory::Register),
            RECORD_PSEUDO_STRUCT => Some(InputCategory::StateRecord),
            STATE_PSEUDO_STRUCT => Some(InputCategory::PublicState),
            STATE_LEAF_PSEUDO_STRUCT => Some(InputCategory::StateLeaf),
            _ => None,
        }
    }
}
