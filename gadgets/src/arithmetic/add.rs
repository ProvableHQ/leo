use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::uint::{UInt, UInt128, UInt16, UInt32, UInt64, UInt8},
    },
};

/// Returns addition of `self` + `other` in the constraint system.
pub trait Add<F: Field, Rhs = Self>
where
    Self: std::marker::Sized,
{
    type ErrorType;

    #[must_use]
    fn add<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, Self::ErrorType>;
}

// Implement unsigned integers
macro_rules! add_uint_impl {
    ($($gadget: ident),*) => ($(
        impl<F: Field + PrimeField> Add<F> for $gadget {
            type ErrorType = SynthesisError;

            fn add<CS: ConstraintSystem<F>>(
                &self,
                cs: CS,
                other: &Self
            ) -> Result<Self, Self::ErrorType> {
                <$gadget as UInt>::addmany(cs, &[self.clone(), other.clone()])
            }
        }
    )*)
}

add_uint_impl!(UInt8, UInt16, UInt32, UInt64, UInt128);
