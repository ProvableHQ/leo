use snarkos_models::{curves::Field, gadgets::r1cs::ConstraintSystem};

/// Exponentiation for a signed integer gadget
pub trait Pow<F: Field, Rhs = Self>
where
    Self: std::marker::Sized,
{
    type ErrorType;

    #[must_use]
    fn pow<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, Self::ErrorType>;
}
