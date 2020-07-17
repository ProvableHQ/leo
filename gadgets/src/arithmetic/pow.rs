use snarkos_models::{curves::Field, gadgets::r1cs::ConstraintSystem};

/// Returns exponentiation of `self` ** `other` in the constraint system.
pub trait Pow<F: Field, Rhs = Self>
where
    Self: std::marker::Sized,
{
    type ErrorType;

    #[must_use]
    fn pow<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, Self::ErrorType>;
}
