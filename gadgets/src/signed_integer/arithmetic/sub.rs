use crate::{errors::SignedIntegerError, Add, Int128, Int16, Int32, Int64, Int8, Negate};
use snarkos_models::{curves::PrimeField, gadgets::r1cs::ConstraintSystem};

/// Subtraction for a signed integer gadget
pub trait Sub<Rhs = Self>
where
    Self: std::marker::Sized,
{
    #[must_use]
    fn sub<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, SignedIntegerError>;
}

macro_rules! sub_int_impl {
    ($($gadget: ident)*) => ($(
        impl Sub for $gadget {
            fn sub<F: PrimeField, CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, SignedIntegerError> {
                // Negate other
                let other_neg = other.neg(cs.ns(|| format!("negate")))?;

                // self + negated other
                self.add(cs.ns(|| format!("add_complement")), &other_neg)
            }
        }
    )*)
}

sub_int_impl!(Int8 Int16 Int32 Int64 Int128);
