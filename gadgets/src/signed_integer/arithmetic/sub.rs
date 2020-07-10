use crate::{errors::IntegerError, Int128, Int16, Int32, Int64, Int8};
use snarkos_models::{curves::PrimeField, gadgets::r1cs::ConstraintSystem};

/// Modular subtraction for a signed integer gadget
pub trait Sub<Rhs = Self>
where
    Self: std::marker::Sized,
{
    #[must_use]
    fn sub<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<(), IntegerError>;
}

macro_rules! sub_int_impl {
    ($($t:ty)*) => ($(
        impl Sub for $t {
            fn sub<F: PrimeField, CS: ConstraintSystem<F>>(&self, _cs: CS, _other: &Self) -> Result<(), IntegerError> {
                Ok(())
            }
        }
    )*)
}

sub_int_impl!(Int8 Int16 Int32 Int64 Int128);
