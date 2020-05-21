//! An in memory store to keep track of defined names when constraining a Leo program.

use crate::constraints::ConstrainedValue;

use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};
use std::{collections::HashMap, marker::PhantomData};

pub struct ConstrainedProgram<NativeF: Field, F: Field + PrimeField, CS: ConstraintSystem<F>> {
    pub identifiers: HashMap<String, ConstrainedValue<NativeF, F>>,
    pub _cs: PhantomData<CS>,
}

pub fn new_scope(outer: String, inner: String) -> String {
    format!("{}_{}", outer, inner)
}

impl<NativeF: Field, F: Field + PrimeField, CS: ConstraintSystem<F>>
    ConstrainedProgram<NativeF, F, CS>
{
    pub fn new() -> Self {
        Self {
            identifiers: HashMap::new(),
            _cs: PhantomData::<CS>,
        }
    }

    pub(crate) fn store(&mut self, name: String, value: ConstrainedValue<NativeF, F>) {
        self.identifiers.insert(name, value);
    }

    pub(crate) fn get(&self, name: &String) -> Option<&ConstrainedValue<NativeF, F>> {
        self.identifiers.get(name)
    }

    pub(crate) fn get_mut(&mut self, name: &String) -> Option<&mut ConstrainedValue<NativeF, F>> {
        self.identifiers.get_mut(name)
    }
}
