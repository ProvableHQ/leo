use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{
            boolean::Boolean,
            uint::{UInt128, UInt16, UInt32, UInt64, UInt8},
        },
    },
};

pub trait EvaluateLtGadget<F: Field> {
    fn less_than<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError>;
}

// implementing `EvaluateLtGadget` will implement `ComparatorGadget`
pub trait ComparatorGadget<F: Field>
where
    Self: EvaluateLtGadget<F>,
{
    fn greater_than<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        other.less_than(cs, self)
    }

    fn less_than_or_equal<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        let is_gt = self.greater_than(cs, other)?;
        Ok(is_gt.not())
    }

    fn greater_than_or_equal<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        other.less_than_or_equal(cs, self)
    }
}

macro_rules! uint_cmp_impl {
    ($($gadget: ident),*) => ($(
        impl<F: Field + PrimeField> EvaluateLtGadget<F> for $gadget {
            fn less_than<CS: ConstraintSystem<F>>(&self, mut cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
                for (i, (self_bit, other_bit)) in self
                    .bits
                    .iter()
                    .rev()
                    .zip(other.bits.iter().rev())
                    .enumerate()
                {
                    // is_greater = a & !b
                    // only true when a > b
                    let is_greater = Boolean::and(cs.ns(|| format!("a and not b [{}]", i)), self_bit, &other_bit.not())?;

                    // is_less = !a & b
                    // only true when a < b
                    let is_less = Boolean::and(cs.ns(|| format!("not a and b [{}]", i)), &self_bit.not(), other_bit)?;

                    if is_greater.get_value().unwrap() {
                        return Ok(is_greater.not());
                    } else if is_less.get_value().unwrap() {
                        return Ok(is_less);
                    } else if i == self.bits.len() - 1 {
                        return Ok(is_less);
                    }
                }

                Err(SynthesisError::Unsatisfiable)
            }
        }

        impl<F: Field + PrimeField> ComparatorGadget<F> for $gadget {}
    )*)
}

uint_cmp_impl!(UInt8, UInt16, UInt32, UInt64, UInt128);
