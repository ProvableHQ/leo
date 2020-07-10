use crate::{errors::IntegerError, Int128, Int16, Int32, Int64, Int8};
use snarkos_models::{curves::PrimeField, gadgets::r1cs::ConstraintSystem};

/// Modular exponentiation for a signed integer gadget
pub trait Pow<Rhs = Self>
where
    Self: std::marker::Sized,
{
    #[must_use]
    fn pow<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<(), IntegerError>;
}

macro_rules! pow_int_impl {
    ($($t:ty)*) => ($(
        impl Pow for $t {
            fn pow<F: PrimeField, CS: ConstraintSystem<F>>(&self, _cs: CS, _other: &Self) -> Result<(), IntegerError> {
                Ok(())
            }
        }
    )*)
}

pow_int_impl!(Int8 Int16 Int32 Int64 Int128);
