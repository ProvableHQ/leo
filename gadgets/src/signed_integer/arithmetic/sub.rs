use crate::{
    arithmetic::{Add, Neg, Sub},
    errors::SignedIntegerError,
    Int128,
    Int16,
    Int32,
    Int64,
    Int8,
};
use snarkos_models::{curves::PrimeField, gadgets::r1cs::ConstraintSystem};

macro_rules! sub_int_impl {
    ($($gadget: ident)*) => ($(
        impl<F: PrimeField> Sub<F> for $gadget {
            type ErrorType = SignedIntegerError;

            fn sub<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Self, Self::ErrorType> {
                // Negate other
                let other_neg = other.neg(cs.ns(|| format!("negate")))?;

                // self + negated other
                self.add(cs.ns(|| format!("add_complement")), &other_neg)
            }
        }
    )*)
}

sub_int_impl!(Int8 Int16 Int32 Int64 Int128);
