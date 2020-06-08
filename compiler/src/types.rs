//! A typed Leo program consists of import, circuit, and function definitions.
//! Each defined type consists of typed statements and expressions.

use crate::Import;
use leo_types::{Identifier, Function, TestFunction, Type};

use std::collections::HashMap;

/// Circuits

#[derive(Clone, PartialEq, Eq)]
pub enum CircuitMember {
    CircuitField(Identifier, Type),
    CircuitFunction(bool, Function),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Circuit {
    pub identifier: Identifier,
    pub members: Vec<CircuitMember>,
}

/// A simple program with statement expressions, program arguments and program returns.
#[derive(Debug, Clone)]
pub struct Program {
    pub name: Identifier,
    pub num_parameters: usize,
    pub imports: Vec<Import>,
    pub circuits: HashMap<Identifier, Circuit>,
    pub functions: HashMap<Identifier, Function>,
    pub tests: HashMap<Identifier, TestFunction>,
}

impl<'ast> Program {
    pub fn new() -> Self {
        Self {
            name: Identifier::new("".into()),
            num_parameters: 0,
            imports: vec![],
            circuits: HashMap::new(),
            functions: HashMap::new(),
            tests: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.name.clone()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Identifier::new(name);
        self
    }
}
