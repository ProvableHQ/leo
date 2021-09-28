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

use crate::{Circuit, CircuitMember, Identifier, Scope, Type, Variable};
use leo_errors::Span;

use indexmap::IndexMap;
use serde::Serialize;
use std::cell::RefCell;

/// Stores program input values as ASG nodes.
#[derive(Clone, Copy, Serialize)]
pub struct Input<'a> {
    pub registers: &'a Circuit<'a>,
    pub state: &'a Circuit<'a>,
    pub state_leaf: &'a Circuit<'a>,
    pub record: &'a Circuit<'a>,
    pub container_circuit: &'a Circuit<'a>,
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

pub const CONTAINER_PSEUDO_CIRCUIT: &str = "$InputContainer";
pub const REGISTERS_PSEUDO_CIRCUIT: &str = "$InputRegister";
pub const RECORD_PSEUDO_CIRCUIT: &str = "$InputRecord";
pub const STATE_PSEUDO_CIRCUIT: &str = "$InputState";
pub const STATE_LEAF_PSEUDO_CIRCUIT: &str = "$InputStateLeaf";

impl<'a> Input<'a> {
    fn make_header(scope: &'a Scope<'a>, name: &str) -> &'a Circuit<'a> {
        scope.context.alloc_circuit(Circuit {
            id: scope.context.get_id(),
            name: RefCell::new(Identifier::new(name.into())),
            members: RefCell::new(IndexMap::new()),
            scope,
            span: Some(Span::default()),
        })
    }

    pub fn new(scope: &'a Scope<'a>) -> Self {
        let input_scope = scope.make_subscope();
        let registers = Self::make_header(input_scope, REGISTERS_PSEUDO_CIRCUIT);
        let record = Self::make_header(input_scope, RECORD_PSEUDO_CIRCUIT);
        let state = Self::make_header(input_scope, STATE_PSEUDO_CIRCUIT);
        let state_leaf = Self::make_header(input_scope, STATE_LEAF_PSEUDO_CIRCUIT);

        let mut container_members = IndexMap::new();
        container_members.insert(
            "registers".to_string(),
            CircuitMember::Variable(Type::Circuit(registers)),
        );
        container_members.insert("record".to_string(), CircuitMember::Variable(Type::Circuit(record)));
        container_members.insert("state".to_string(), CircuitMember::Variable(Type::Circuit(state)));
        container_members.insert(
            "state_leaf".to_string(),
            CircuitMember::Variable(Type::Circuit(state_leaf)),
        );

        let container_circuit = input_scope.context.alloc_circuit(Circuit {
            id: scope.context.get_id(),
            name: RefCell::new(Identifier::new(CONTAINER_PSEUDO_CIRCUIT.into())),
            members: RefCell::new(container_members),
            scope: input_scope,
            span: Some(Span::default()),
        });

        Input {
            registers,
            record,
            state,
            state_leaf,
            container_circuit,
            container: input_scope.context.alloc_variable(RefCell::new(crate::InnerVariable {
                id: scope.context.get_id(),
                name: Identifier::new("input".into()),
                type_: Type::Circuit(container_circuit),
                mutable: false,
                const_: false,
                declaration: crate::VariableDeclaration::Input,
                references: vec![],
                assignments: vec![],
            })),
        }
    }
}

impl<'a> Circuit<'a> {
    pub fn is_input_pseudo_circuit(&self) -> bool {
        matches!(
            &*self.name.borrow().name,
            REGISTERS_PSEUDO_CIRCUIT | RECORD_PSEUDO_CIRCUIT | STATE_PSEUDO_CIRCUIT | STATE_LEAF_PSEUDO_CIRCUIT
        )
    }

    pub fn input_type(&self) -> Option<InputCategory> {
        match self.name.borrow().name.as_ref() {
            REGISTERS_PSEUDO_CIRCUIT => Some(InputCategory::Register),
            RECORD_PSEUDO_CIRCUIT => Some(InputCategory::StateRecord),
            STATE_PSEUDO_CIRCUIT => Some(InputCategory::PublicState),
            STATE_LEAF_PSEUDO_CIRCUIT => Some(InputCategory::StateLeaf),
            _ => None,
        }
    }
}
