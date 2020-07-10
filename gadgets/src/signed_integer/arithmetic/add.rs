use crate::{errors::IntegerError, Int128, Int16, Int32, Int64, Int8};
use snarkos_models::{curves::PrimeField, gadgets::r1cs::ConstraintSystem};

/// Implements modular addition for a signed integer gadget
pub trait Add<Rhs = Self>
where
    Self: std::marker::Sized,
{
    #[must_use]
    fn add<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<(), IntegerError>;
}

macro_rules! add_int_impl {
    ($($t:ty)*) => ($(
        impl Add for $t {
            fn add<F: PrimeField, CS: ConstraintSystem<F>>(&self, _cs: CS, _other: &Self) -> Result<(), IntegerError> {
                Ok(())
            }
        }
    )*)
}

add_int_impl!(Int8 Int16 Int32 Int64 Int128);
