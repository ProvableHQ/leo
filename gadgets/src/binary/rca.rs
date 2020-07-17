use crate::{binary::FullAdder, signed_integer::*};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::PrimeField,
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

/// Returns the bitwise sum of a n-bit number with carry bit
pub trait RippleCarryAdder<Rhs = Self>
where
    Self: std::marker::Sized,
{
    #[must_use]
    fn add_bits<F: PrimeField, CS: ConstraintSystem<F>>(
        &self,
        cs: CS,
        other: &Self,
    ) -> Result<Vec<Boolean>, SynthesisError>;
}

// Generic impl
impl RippleCarryAdder for Vec<Boolean> {
    fn add_bits<F: PrimeField, CS: ConstraintSystem<F>>(
        &self,
        mut cs: CS,
        other: &Self,
    ) -> Result<Vec<Boolean>, SynthesisError> {
        let mut result = vec![];
        let mut carry = Boolean::constant(false);
        for (i, (a, b)) in self.iter().zip(other.iter()).enumerate() {
            let (sum, next) = Boolean::add(cs.ns(|| format!("rpc {}", i)), a, b, &carry)?;

            carry = next;
            result.push(sum);
        }

        // append the carry bit to the end
        result.push(carry);

        Ok(result)
    }
}

macro_rules! rpc_impl {
    ($($gadget: ident)*) => ($(
        impl RippleCarryAdder for $gadget {
            fn add_bits<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Vec<Boolean>, SynthesisError> {
                self.bits.add_bits(cs, &other.bits)
            }
        }
    )*)
}

rpc_impl!(Int8 Int16 Int32 Int64 Int128);
