use std::sync::{ Arc, Weak };
use crate::{ Circuit, Variable, Identifier, Type, CircuitMember, WeakType };
use std::cell::RefCell;
use indexmap::IndexMap;

#[derive(Clone)]
pub struct Input {
    pub registers: Arc<Circuit>,
    pub state: Arc<Circuit>,
    pub state_leaf: Arc<Circuit>,
    pub record: Arc<Circuit>,
    pub container_circuit: Arc<Circuit>,
    pub container: Variable,
}

pub const CONTAINER_PSUEDO_CIRCUIT: &str = "$InputContainer";
pub const REGISTERS_PSUEDO_CIRCUIT: &str = "$InputRegister";
pub const RECORD_PSUEDO_CIRCUIT: &str = "$InputRecord";
pub const STATE_PSUEDO_CIRCUIT: &str = "$InputState";
pub const STATE_LEAF_PSUEDO_CIRCUIT: &str = "$InputStateLeaf";

impl Input {
    pub fn new() -> Self {
        let registers = Arc::new(Circuit {
            id: uuid::Uuid::new_v4(),
            name: RefCell::new(Identifier::new(REGISTERS_PSUEDO_CIRCUIT.to_string())),
            body: RefCell::new(Weak::new()),
            members: RefCell::new(IndexMap::new()),
        });
        let record = Arc::new(Circuit {
            id: uuid::Uuid::new_v4(),
            name: RefCell::new(Identifier::new(RECORD_PSUEDO_CIRCUIT.to_string())),
            body: RefCell::new(Weak::new()),
            members: RefCell::new(IndexMap::new()),
        });
        let state = Arc::new(Circuit {
            id: uuid::Uuid::new_v4(),
            name: RefCell::new(Identifier::new(STATE_PSUEDO_CIRCUIT.to_string())),
            body: RefCell::new(Weak::new()),
            members: RefCell::new(IndexMap::new()),
        });
        let state_leaf = Arc::new(Circuit {
            id: uuid::Uuid::new_v4(),
            name: RefCell::new(Identifier::new(STATE_LEAF_PSUEDO_CIRCUIT.to_string())),
            body: RefCell::new(Weak::new()),
            members: RefCell::new(IndexMap::new()),
        });

        let mut container_members = IndexMap::new();
        container_members.insert("registers".to_string(), CircuitMember::Variable(WeakType::Circuit(Arc::downgrade(&registers))));
        container_members.insert("record".to_string(), CircuitMember::Variable(WeakType::Circuit(Arc::downgrade(&record))));
        container_members.insert("state".to_string(), CircuitMember::Variable(WeakType::Circuit(Arc::downgrade(&state))));
        container_members.insert("state_leaf".to_string(), CircuitMember::Variable(WeakType::Circuit(Arc::downgrade(&state_leaf))));

        let container_circuit = Arc::new(Circuit {
            id: uuid::Uuid::new_v4(),
            name: RefCell::new(Identifier::new(CONTAINER_PSUEDO_CIRCUIT.to_string())),
            body: RefCell::new(Weak::new()),
            members: RefCell::new(container_members),
        });

        Input {
            registers,
            record,
            state,
            state_leaf,
            container_circuit: container_circuit.clone(),
            container: Arc::new(RefCell::new(crate::InnerVariable {
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
            REGISTERS_PSUEDO_CIRCUIT |
            RECORD_PSUEDO_CIRCUIT |
            STATE_PSUEDO_CIRCUIT |
            STATE_LEAF_PSUEDO_CIRCUIT => true,
            _ => false,
        }
    }
}