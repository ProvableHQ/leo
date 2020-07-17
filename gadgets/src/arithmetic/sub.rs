use snarkos_models::{curves::Field, gadgets::r1cs::ConstraintSystem};

/// Returns subtraction of `self` - `other` in the constraint system.
pub trait Sub<F: Field, Rhs = Self>
where
    Self: std::marker::Sized,
{
    type ErrorType;

    #[must_use]
    fn sub<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, Self::ErrorType>;
}
