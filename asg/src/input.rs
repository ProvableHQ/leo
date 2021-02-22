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

use indexmap::IndexMap;
use std::cell::RefCell;

/// Stores program input values as ASG nodes.
#[derive(Clone, Copy)]
pub struct Input<'a> {
    pub registers: &'a Circuit<'a>,
    pub state: &'a Circuit<'a>,
    pub state_leaf: &'a Circuit<'a>,
    pub record: &'a Circuit<'a>,
    pub container_circuit: &'a Circuit<'a>,
    pub container: &'a Variable<'a>,
}

pub const CONTAINER_PSEUDO_CIRCUIT: &str = "$InputContainer";
pub const REGISTERS_PSEUDO_CIRCUIT: &str = "$InputRegister";
pub const RECORD_PSEUDO_CIRCUIT: &str = "$InputRecord";
pub const STATE_PSEUDO_CIRCUIT: &str = "$InputState";
pub const STATE_LEAF_PSEUDO_CIRCUIT: &str = "$InputStateLeaf";

impl<'a> Input<'a> {
    fn make_header(scope: &'a Scope<'a>, name: &str) -> &'a Circuit<'a> {
        scope.alloc_circuit(Circuit {
            id: uuid::Uuid::new_v4(),
            name: RefCell::new(Identifier::new(name.to_string())),
            members: RefCell::new(IndexMap::new()),
            core_mapping: RefCell::new(None),
            scope,
            span: Default::default(),
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

        let container_circuit = input_scope.alloc_circuit(Circuit {
            id: uuid::Uuid::new_v4(),
            name: RefCell::new(Identifier::new(CONTAINER_PSEUDO_CIRCUIT.to_string())),
            members: RefCell::new(container_members),
            core_mapping: RefCell::new(None),
            scope: input_scope,
            span: Default::default(),
        });

        Input {
            registers,
            record,
            state,
            state_leaf,
            container_circuit,
            container: input_scope.alloc_variable(RefCell::new(crate::InnerVariable {
                id: uuid::Uuid::new_v4(),
                name: Identifier::new("input".to_string()),
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
}
