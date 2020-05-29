use crate::errors::GroupError;
use snarkos_models::curves::Field;
use std::fmt::Debug;

pub mod edwards_bls12;

pub trait GroupType<NativeF: Field, F: Field>: Sized + Clone + Debug {
    fn constant(x: String, y: String) -> Result<Self, GroupError>;
}
