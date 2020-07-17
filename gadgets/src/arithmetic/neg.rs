use crate::bits::RippleCarryAdder;

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::Field,
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

/// Returns a negated representation of `self` in the constraint system.
pub trait Neg<F: Field>
where
    Self: std::marker::Sized,
{
    type ErrorType;

    #[must_use]
    fn neg<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Self, Self::ErrorType>;
}

impl<F: Field> Neg<F> for Vec<Boolean> {
    type ErrorType = SynthesisError;

    fn neg<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Self, SynthesisError> {
        // flip all bits
        let flipped: Self = self.iter().map(|bit| bit.not()).collect();

        // add one
        let mut one = vec![Boolean::constant(true)];
        one.append(&mut vec![Boolean::Constant(false); self.len() - 1]);

        let mut bits = flipped.add_bits(cs.ns(|| format!("add one")), &one)?;
        let _carry = bits.pop(); // we already accounted for overflow above

        Ok(bits)
    }
}
