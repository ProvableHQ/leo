use crate::{binary::RippleCarryAdder, errors::IntegerError, signed_integer::*};

use snarkos_models::{curves::PrimeField, gadgets::r1cs::ConstraintSystem};

/// Inverts the given number and adds 1 to the lsb of the result
pub trait TwosComplement
where
    Self: std::marker::Sized,
{
    #[must_use]
    fn twos_comp<F: PrimeField, CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Self, IntegerError>;
}

macro_rules! twos_comp_int_impl {
    ($($gadget: ident)*) => ($(
        impl TwosComplement for $gadget {
            fn twos_comp<F: PrimeField, CS: ConstraintSystem<F>>(
                &self,
                mut cs: CS
            ) -> Result<Self, IntegerError> {
                let value = match self.value {
                    Some(val) => {
                        match val.checked_neg() {
                            Some(val_neg) => Some(val_neg),
                            None => return Err(IntegerError::NegativeZero) // -0 should fail
                        }
                    }
                    None => None,
                };

                // flip all bits
                let flipped_bits = self.bits
                    .iter()
                    .map(|bit| bit.not())
                    .collect();

                let flipped = Self {
                    bits: flipped_bits,
                    value,
                };

                // add one
                let one = Self::one();

                let mut bits = flipped.add_bits(cs.ns(|| format!("add one")), &one)?;
                let _carry = bits.pop(); // we already accounted for overflow above

                Ok(Self {
                    bits,
                    value,
                })
            }
        }
    )*)
}

twos_comp_int_impl!(Int8 Int16 Int32 Int64 Int128);
