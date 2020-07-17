use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::Field,
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

/// Single bit binary adder with carry bit
/// sum = (a XOR b) XOR carry
/// carry = a AND b OR carry AND (a XOR b)
/// Returns (sum, carry)
pub trait FullAdder<'a, F: Field>
where
    Self: std::marker::Sized,
{
    fn add<CS: ConstraintSystem<F>>(
        cs: CS,
        a: &'a Self,
        b: &'a Self,
        carry: &'a Self,
    ) -> Result<(Self, Self), SynthesisError>;
}

impl<'a, F: Field> FullAdder<'a, F> for Boolean {
    fn add<CS: ConstraintSystem<F>>(
        mut cs: CS,
        a: &'a Self,
        b: &'a Self,
        carry: &'a Self,
    ) -> Result<(Self, Self), SynthesisError> {
        let a_x_b = Boolean::xor(cs.ns(|| format!("a XOR b")), a, b)?;
        let sum = Boolean::xor(cs.ns(|| format!("adder sum")), &a_x_b, carry)?;

        let c1 = Boolean::and(cs.ns(|| format!("a AND b")), a, b)?;
        let c2 = Boolean::and(cs.ns(|| format!("carry AND (a XOR b)")), carry, &a_x_b)?;
        let carry = Boolean::or(cs.ns(|| format!("c1 OR c2")), &c1, &c2)?;

        Ok((sum, carry))
    }
}
