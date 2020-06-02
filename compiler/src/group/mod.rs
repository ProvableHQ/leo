use crate::errors::GroupError;

use snarkos_models::{
    curves::Field,
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{
            alloc::AllocGadget,
            eq::{ConditionalEqGadget, EqGadget},
            select::CondSelectGadget,
            ToBitsGadget, ToBytesGadget,
        },
    },
};
use std::fmt::Debug;

pub mod edwards_bls12;

pub trait GroupType<F: Field>:
    Sized
    + Clone
    + Debug
    + EqGadget<F>
    + ConditionalEqGadget<F>
    + AllocGadget<String, F>
    + CondSelectGadget<F>
    + ToBitsGadget<F>
    + ToBytesGadget<F>
{
    fn constant(string: String) -> Result<Self, GroupError>;

    fn add<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, GroupError>;

    fn sub<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, GroupError>;
}
