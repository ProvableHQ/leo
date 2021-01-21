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

use crate::{Circuit, CircuitBody, CircuitMember, CircuitMemberBody, Identifier, Scope, Type, Variable, WeakType};
use indexmap::IndexMap;
use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

#[derive(Clone)]
pub struct Input {
    pub registers: Arc<CircuitBody>,
    pub state: Arc<CircuitBody>,
    pub state_leaf: Arc<CircuitBody>,
    pub record: Arc<CircuitBody>,
    pub container_circuit: Arc<CircuitBody>,
    pub container: Variable,
}

pub const CONTAINER_PSUEDO_CIRCUIT: &str = "$InputContainer";
pub const REGISTERS_PSUEDO_CIRCUIT: &str = "$InputRegister";
pub const RECORD_PSUEDO_CIRCUIT: &str = "$InputRecord";
pub const STATE_PSUEDO_CIRCUIT: &str = "$InputState";
pub const STATE_LEAF_PSUEDO_CIRCUIT: &str = "$InputStateLeaf";

impl Input {
    fn make_header(name: &str) -> Arc<Circuit> {
        Arc::new(Circuit {
            id: uuid::Uuid::new_v4(),
            name: RefCell::new(Identifier::new(name.to_string())),
            body: RefCell::new(Weak::new()),
            members: RefCell::new(IndexMap::new()),
        })
    }

    fn make_body(scope: &Scope, circuit: &Arc<Circuit>) -> Arc<CircuitBody> {
        let body = Arc::new(CircuitBody {
            scope: scope.clone(),
            span: None,
            circuit: circuit.clone(),
            members: RefCell::new(IndexMap::new()),
        });
        circuit.body.replace(Arc::downgrade(&body));
        body
    }

    pub fn new(scope: &Scope) -> Self {
        let registers = Self::make_header(REGISTERS_PSUEDO_CIRCUIT);
        let record = Self::make_header(RECORD_PSUEDO_CIRCUIT);
        let state = Self::make_header(STATE_PSUEDO_CIRCUIT);
        let state_leaf = Self::make_header(STATE_LEAF_PSUEDO_CIRCUIT);

        let mut container_members = IndexMap::new();
        container_members.insert(
            "registers".to_string(),
            CircuitMember::Variable(WeakType::Circuit(Arc::downgrade(&registers))),
        );
        container_members.insert(
            "record".to_string(),
            CircuitMember::Variable(WeakType::Circuit(Arc::downgrade(&record))),
        );
        container_members.insert(
            "state".to_string(),
            CircuitMember::Variable(WeakType::Circuit(Arc::downgrade(&state))),
        );
        container_members.insert(
            "state_leaf".to_string(),
            CircuitMember::Variable(WeakType::Circuit(Arc::downgrade(&state_leaf))),
        );

        let container_circuit = Arc::new(Circuit {
            id: uuid::Uuid::new_v4(),
            name: RefCell::new(Identifier::new(CONTAINER_PSUEDO_CIRCUIT.to_string())),
            body: RefCell::new(Weak::new()),
            members: RefCell::new(container_members),
        });

        let registers_body = Self::make_body(scope, &registers);
        let record_body = Self::make_body(scope, &record);
        let state_body = Self::make_body(scope, &state);
        let state_leaf_body = Self::make_body(scope, &state_leaf);

        let mut container_body_members = IndexMap::new();
        container_body_members.insert(
            "registers".to_string(),
            CircuitMemberBody::Variable(Type::Circuit(registers.clone())),
        );
        container_body_members.insert(
            "record".to_string(),
            CircuitMemberBody::Variable(Type::Circuit(record.clone())),
        );
        container_body_members.insert(
            "state".to_string(),
            CircuitMemberBody::Variable(Type::Circuit(state.clone())),
        );
        container_body_members.insert(
            "state_leaf".to_string(),
            CircuitMemberBody::Variable(Type::Circuit(state_leaf.clone())),
        );

        let container_circuit_body = Arc::new(CircuitBody {
            scope: scope.clone(),
            span: None,
            circuit: container_circuit.clone(),
            members: RefCell::new(container_body_members),
        });
        container_circuit.body.replace(Arc::downgrade(&container_circuit_body));

        Input {
            registers: registers_body,
            record: record_body,
            state: state_body,
            state_leaf: state_leaf_body,
            container_circuit: container_circuit_body,
            container: Arc::new(RefCell::new(crate::InnerVariable {
                id: uuid::Uuid::new_v4(),
                name: Identifier::new("input".to_string()),
                type_: Type::Circuit(container_circuit),
                mutable: false,
                declaration: crate::VariableDeclaration::Input,
                const_value: None,
                references: vec![],
                assignments: vec![],
            })),
        }
    }
}

impl Circuit {
    pub fn is_input_psuedo_circuit(&self) -> bool {
        match &*self.name.borrow().name {
            REGISTERS_PSUEDO_CIRCUIT | RECORD_PSUEDO_CIRCUIT | STATE_PSUEDO_CIRCUIT | STATE_LEAF_PSUEDO_CIRCUIT => true,
            _ => false,
        }
    }
}
