//! An in memory store to keep track of defined names when constraining a Leo program.

use crate::constraints::ConstrainedValue;

use snarkos_models::curves::TEModelParameters;
use snarkos_models::gadgets::curves::FieldGadget;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};
use std::{collections::HashMap, marker::PhantomData};

pub struct ConstrainedProgram<
    P: std::clone::Clone + TEModelParameters,
    F: Field + PrimeField,
    FG: FieldGadget<P::BaseField, F>,
    CS: ConstraintSystem<F>,
> {
    pub identifiers: HashMap<String, ConstrainedValue<P, F, FG>>,
    pub _cs: PhantomData<CS>,
}

pub fn new_scope(outer: String, inner: String) -> String {
    format!("{}_{}", outer, inner)
}

impl<
        P: std::clone::Clone + TEModelParameters,
        F: Field + PrimeField,
        FG: FieldGadget<P::BaseField, F>,
        CS: ConstraintSystem<F>,
    > ConstrainedProgram<P, F, FG, CS>
{
    pub fn new() -> Self {
        Self {
            identifiers: HashMap::new(),
            _cs: PhantomData::<CS>,
        }
    }

    pub(crate) fn store(&mut self, name: String, value: ConstrainedValue<P, F, FG>) {
        self.identifiers.insert(name, value);
    }

    pub(crate) fn get(&self, name: &String) -> Option<&ConstrainedValue<P, F, FG>> {
        self.identifiers.get(name)
    }

    pub(crate) fn get_mut(&mut self, name: &String) -> Option<&mut ConstrainedValue<P, F, FG>> {
        self.identifiers.get_mut(name)
    }
}
