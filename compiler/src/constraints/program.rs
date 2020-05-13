//! An in memory store to keep track of defined names when constraining a Leo program.

use crate::{constraints::ConstrainedValue, types::Variable};

use snarkos_models::{
    curves::{Group, Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};
use std::{collections::HashMap, marker::PhantomData};

pub struct ConstrainedProgram<F: Field + PrimeField, G: Group, CS: ConstraintSystem<F>> {
    pub resolved_names: HashMap<String, ConstrainedValue<F, G>>,
    pub _cs: PhantomData<CS>,
}

pub fn new_scope(outer: String, inner: String) -> String {
    format!("{}_{}", outer, inner)
}

pub fn new_scope_from_variable<F: Field + PrimeField, G: Group>(
    outer: String,
    inner: &Variable<F, G>,
) -> String {
    new_scope(outer, inner.name.clone())
}

pub fn new_variable_from_variable<F: Field + PrimeField, G: Group>(
    outer: String,
    inner: &Variable<F, G>,
) -> Variable<F, G> {
    Variable {
        name: new_scope_from_variable(outer, inner),
        _engine: PhantomData::<F>,
        _group: PhantomData::<G>
    }
}

pub fn new_variable_from_variables<F: Field + PrimeField, G: Group>(
    outer: &Variable<F, G>,
    inner: &Variable<F, G>,
) -> Variable<F, G> {
    Variable {
        name: new_scope_from_variable(outer.name.clone(), inner),
        _engine: PhantomData::<F>,
        _group: PhantomData::<G>,
    }
}

impl<F: Field + PrimeField, G: Group, CS: ConstraintSystem<F>> ConstrainedProgram<F, G, CS> {
    pub fn new() -> Self {
        Self {
            resolved_names: HashMap::new(),
            _cs: PhantomData::<CS>,
        }
    }

    pub(crate) fn store(&mut self, name: String, value: ConstrainedValue<F, G>) {
        self.resolved_names.insert(name, value);
    }

    pub(crate) fn store_variable(&mut self, variable: Variable<F, G>, value: ConstrainedValue<F, G>) {
        self.store(variable.name, value);
    }

    pub(crate) fn contains_name(&self, name: &String) -> bool {
        self.resolved_names.contains_key(name)
    }

    pub(crate) fn contains_variable(&self, variable: &Variable<F, G>) -> bool {
        self.contains_name(&variable.name)
    }

    pub(crate) fn get(&self, name: &String) -> Option<&ConstrainedValue<F, G>> {
        self.resolved_names.get(name)
    }

    pub(crate) fn get_mut(&mut self, name: &String) -> Option<&mut ConstrainedValue<F, G>> {
        self.resolved_names.get_mut(name)
    }

    pub(crate) fn get_mut_variable(
        &mut self,
        variable: &Variable<F, G>,
    ) -> Option<&mut ConstrainedValue<F, G>> {
        self.get_mut(&variable.name)
    }
}
