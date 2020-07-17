use snarkos_models::{curves::Field, gadgets::r1cs::ConstraintSystem};

/// Returns division of `self` / `other` in the constraint system.
pub trait Div<F: Field, Rhs = Self>
where
    Self: std::marker::Sized,
{
    type ErrorType;

    #[must_use]
    fn div<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, Self::ErrorType>;
}
