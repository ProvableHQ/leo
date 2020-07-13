use crate::errors::IntegerError;

use snarkos_models::{
    curves::PrimeField,
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

///
pub trait Cmp<Rhs = Self>
where
    Self: std::marker::Sized,
{
}
