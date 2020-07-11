use crate::{errors::IntegerError, Int16, Int32, Int64, Int8};
use snarkos_models::{curves::PrimeField, gadgets::r1cs::ConstraintSystem};

/// Division for a signed integer gadget
pub trait Div<Rhs = Self>
where
    Self: std::marker::Sized,
{
    #[must_use]
    fn div<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<(), IntegerError>;
}

macro_rules! div_int_impl {
    ($($t:ty)*) => ($(
        impl Div for $t {
            fn div<F: PrimeField, CS: ConstraintSystem<F>>(&self, _cs: CS, _other: &Self) -> Result<(), IntegerError> {
                Ok(())
            }
        }
    )*)
}

div_int_impl!(Int8 Int16 Int32 Int64);
