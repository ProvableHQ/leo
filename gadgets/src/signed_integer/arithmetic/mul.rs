use crate::{errors::IntegerError, Int128, Int16, Int32, Int64, Int8};
use snarkos_models::{curves::PrimeField, gadgets::r1cs::ConstraintSystem};

/// Modular multiplication for a signed integer gadget
pub trait Mul<Rhs = Self>
where
    Self: std::marker::Sized,
{
    #[must_use]
    fn mul<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<(), IntegerError>;
}

macro_rules! mul_int_impl {
    ($($t:ty)*) => ($(
        impl Mul for $t {
            fn mul<F: PrimeField, CS: ConstraintSystem<F>>(&self, _cs: CS, _other: &Self) -> Result<(), IntegerError> {
                Ok(())
            }
        }
    )*)
}

mul_int_impl!(Int8 Int16 Int32 Int64 Int128);
