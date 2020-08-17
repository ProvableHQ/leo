//! An in memory store to keep track of defined names when constraining a Leo program.

use crate::{value::ConstrainedValue, GroupType};

use snarkos_models::curves::{Field, PrimeField};

use std::collections::HashMap;

#[derive(Clone)]
pub struct ConstrainedProgram<F: Field + PrimeField, G: GroupType<F>> {
    pub identifiers: HashMap<String, ConstrainedValue<F, G>>,
}

pub fn new_scope(outer: String, inner: String) -> String {
    format!("{}_{}", outer, inner)
}

pub fn is_in_scope(current_scope: &String, desired_scope: &String) -> bool {
    current_scope.ends_with(desired_scope)
}

impl<F: Field + PrimeField, G: GroupType<F>> ConstrainedProgram<F, G> {
    pub fn new() -> Self {
        Self {
            identifiers: HashMap::new(),
        }
    }

    pub(crate) fn store(&mut self, name: String, value: ConstrainedValue<F, G>) {
        self.identifiers.insert(name, value);
    }

    pub(crate) fn get(&self, name: &String) -> Option<&ConstrainedValue<F, G>> {
        self.identifiers.get(name)
    }

    pub(crate) fn get_mut(&mut self, name: &String) -> Option<&mut ConstrainedValue<F, G>> {
        self.identifiers.get_mut(name)
    }
}
