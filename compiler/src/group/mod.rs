use crate::errors::GroupError;
use snarkos_models::curves::Field;
use snarkos_models::gadgets::r1cs::ConstraintSystem;
use snarkos_models::gadgets::utilities::eq::{ConditionalEqGadget, EqGadget};
use std::fmt::Debug;

pub mod edwards_bls12;

pub trait GroupType<NativeF: Field, F: Field>:
    Sized + Clone + Debug + EqGadget<F> + ConditionalEqGadget<F>
{
    fn constant(string: String) -> Result<Self, GroupError>;

    fn add<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, GroupError>;

    fn sub<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, GroupError>;
}
