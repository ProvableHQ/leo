use crate::{errors::GroupError, GroupType};

use snarkos_curves::{
    edwards_bls12::{EdwardsAffine, EdwardsParameters, Fq},
    templates::twisted_edwards_extended::GroupAffine,
};
use snarkos_errors::gadgets::SynthesisError;
use snarkos_gadgets::curves::edwards_bls12::EdwardsBlsGadget;
use snarkos_models::{
    curves::AffineCurve,
    gadgets::{
        curves::{FpGadget, GroupGadget},
        r1cs::ConstraintSystem,
        utilities::{
            alloc::AllocGadget,
            boolean::Boolean,
            eq::{ConditionalEqGadget, EqGadget, EvaluateEqGadget},
            select::CondSelectGadget,
            uint::UInt8,
            ToBitsGadget,
            ToBytesGadget,
        },
    },
};
use std::{borrow::Borrow, ops::Sub, str::FromStr};

#[derive(Clone, Debug)]
pub enum EdwardsGroupType {
    Constant(EdwardsAffine),
    Allocated(EdwardsBlsGadget),
}

impl GroupType<Fq> for EdwardsGroupType {
    fn constant(string: String) -> Result<Self, GroupError> {
        let value = Self::edwards_affine_from_str(string)?;

        Ok(EdwardsGroupType::Constant(value))
    }

    fn add<CS: ConstraintSystem<Fq>>(&self, cs: CS, other: &Self) -> Result<Self, GroupError> {
        match (self, other) {
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                Ok(EdwardsGroupType::Constant(self_value.add(other_value)))
            }

            (EdwardsGroupType::Allocated(self_value), EdwardsGroupType::Allocated(other_value)) => {
                let result = <EdwardsBlsGadget as GroupGadget<GroupAffine<EdwardsParameters>, Fq>>::add(
                    self_value,
                    cs,
                    other_value,
                )?;
                Ok(EdwardsGroupType::Allocated(result))
            }

            (EdwardsGroupType::Constant(constant_value), EdwardsGroupType::Allocated(allocated_value))
            | (EdwardsGroupType::Allocated(allocated_value), EdwardsGroupType::Constant(constant_value)) => Ok(
                EdwardsGroupType::Allocated(allocated_value.add_constant(cs, constant_value)?),
            ),
        }
    }

    fn sub<CS: ConstraintSystem<Fq>>(&self, cs: CS, other: &Self) -> Result<Self, GroupError> {
        match (self, other) {
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                Ok(EdwardsGroupType::Constant(self_value.sub(other_value)))
            }

            (EdwardsGroupType::Allocated(self_value), EdwardsGroupType::Allocated(other_value)) => {
                let result = <EdwardsBlsGadget as GroupGadget<GroupAffine<EdwardsParameters>, Fq>>::sub(
                    self_value,
                    cs,
                    other_value,
                )?;
                Ok(EdwardsGroupType::Allocated(result))
            }

            (EdwardsGroupType::Constant(constant_value), EdwardsGroupType::Allocated(allocated_value))
            | (EdwardsGroupType::Allocated(allocated_value), EdwardsGroupType::Constant(constant_value)) => Ok(
                EdwardsGroupType::Allocated(allocated_value.sub_constant(cs, constant_value)?),
            ),
        }
    }
}

impl EdwardsGroupType {
    pub fn edwards_affine_from_str(string: String) -> Result<EdwardsAffine, GroupError> {
        // 0 or (0, 1)
        match Fq::from_str(&string).ok() {
            Some(x) => EdwardsAffine::get_point_from_x(x, false).ok_or(GroupError::Invalid(string)),
            None => EdwardsAffine::from_str(&string).map_err(|_| GroupError::Invalid(string)),
        }
    }

    pub fn alloc_x_helper<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>>(
        value_gen: Fn,
    ) -> Result<EdwardsAffine, SynthesisError> {
        let affine_string = match value_gen() {
            Ok(value) => {
                let string_value = value.borrow().clone();
                Ok(string_value)
            }
            _ => Err(SynthesisError::AssignmentMissing),
        }?;

        Self::edwards_affine_from_str(affine_string).map_err(|_| SynthesisError::AssignmentMissing)
    }

    pub fn allocated<CS: ConstraintSystem<Fq>>(&self, mut cs: CS) -> Result<EdwardsBlsGadget, SynthesisError> {
        match self {
            EdwardsGroupType::Constant(constant) => {
                <EdwardsBlsGadget as AllocGadget<GroupAffine<EdwardsParameters>, Fq>>::alloc(
                    &mut cs.ns(|| format!("{:?}", constant)),
                    || Ok(constant),
                )
            }
            EdwardsGroupType::Allocated(allocated) => Ok(allocated.clone()),
        }
    }
}

impl AllocGadget<String, Fq> for EdwardsGroupType {
    fn alloc<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>, CS: ConstraintSystem<Fq>>(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let value = <EdwardsBlsGadget as AllocGadget<GroupAffine<EdwardsParameters>, Fq>>::alloc(cs, || {
            Self::alloc_x_helper(value_gen)
        })?;

        Ok(EdwardsGroupType::Allocated(value))
    }

    fn alloc_input<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<String>, CS: ConstraintSystem<Fq>>(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let value = <EdwardsBlsGadget as AllocGadget<GroupAffine<EdwardsParameters>, Fq>>::alloc_input(cs, || {
            Self::alloc_x_helper(value_gen)
        })?;

        Ok(EdwardsGroupType::Allocated(value))
    }
}

impl PartialEq for EdwardsGroupType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                self_value == other_value
            }

            (EdwardsGroupType::Allocated(self_value), EdwardsGroupType::Allocated(other_value)) => {
                self_value.eq(other_value)
            }

            (EdwardsGroupType::Constant(constant_value), EdwardsGroupType::Allocated(allocated_value))
            | (EdwardsGroupType::Allocated(allocated_value), EdwardsGroupType::Constant(constant_value)) => {
                <EdwardsBlsGadget as GroupGadget<GroupAffine<EdwardsParameters>, Fq>>::get_value(allocated_value)
                    .map(|allocated_value| allocated_value == *constant_value)
                    .unwrap_or(false)
            }
        }
    }
}

impl Eq for EdwardsGroupType {}

impl EvaluateEqGadget<Fq> for EdwardsGroupType {
    fn evaluate_equal<CS: ConstraintSystem<Fq>>(&self, mut cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        match (self, other) {
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                Ok(Boolean::Constant(self_value == other_value))
            }

            (EdwardsGroupType::Allocated(self_value), EdwardsGroupType::Allocated(other_value)) => {
                let bool_option =
                    <EdwardsBlsGadget as GroupGadget<GroupAffine<EdwardsParameters>, Fq>>::get_value(self_value)
                        .and_then(|a| {
                            <EdwardsBlsGadget as GroupGadget<GroupAffine<EdwardsParameters>, Fq>>::get_value(
                                other_value,
                            )
                            .map(|b| a.eq(&b))
                        });
                Boolean::alloc(&mut cs.ns(|| "evaluate_equal"), || {
                    bool_option.ok_or(SynthesisError::AssignmentMissing)
                })
            }

            (EdwardsGroupType::Constant(constant_value), EdwardsGroupType::Allocated(allocated_value))
            | (EdwardsGroupType::Allocated(allocated_value), EdwardsGroupType::Constant(constant_value)) => {
                let bool_option =
                    <EdwardsBlsGadget as GroupGadget<GroupAffine<EdwardsParameters>, Fq>>::get_value(allocated_value)
                        .map(|a| a.eq(constant_value));
                Boolean::alloc(&mut cs.ns(|| "evaluate_equal"), || {
                    bool_option.ok_or(SynthesisError::AssignmentMissing)
                })
            }
        }
    }
}

impl EqGadget<Fq> for EdwardsGroupType {}

impl ConditionalEqGadget<Fq> for EdwardsGroupType {
    #[inline]
    fn conditional_enforce_equal<CS: ConstraintSystem<Fq>>(
        &self,
        mut cs: CS,
        other: &Self,
        condition: &Boolean,
    ) -> Result<(), SynthesisError> {
        match (self, other) {
            // c - c
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                if self_value == other_value {
                    return Ok(());
                }
                Err(SynthesisError::AssignmentMissing)
            }
            // a - a
            (EdwardsGroupType::Allocated(self_value), EdwardsGroupType::Allocated(other_value)) => {
                <EdwardsBlsGadget>::conditional_enforce_equal(self_value, cs, other_value, condition)
            }
            // c - a = a - c
            (EdwardsGroupType::Constant(constant_value), EdwardsGroupType::Allocated(allocated_value))
            | (EdwardsGroupType::Allocated(allocated_value), EdwardsGroupType::Constant(constant_value)) => {
                let x = FpGadget::from(&mut cs, &constant_value.x);
                let y = FpGadget::from(&mut cs, &constant_value.y);
                let constant_gadget = EdwardsBlsGadget::new(x, y);

                constant_gadget.conditional_enforce_equal(cs, allocated_value, condition)
            }
        }
    }

    fn cost() -> usize {
        2 * <EdwardsBlsGadget as ConditionalEqGadget<Fq>>::cost() //upper bound
    }
}

impl CondSelectGadget<Fq> for EdwardsGroupType {
    fn conditionally_select<CS: ConstraintSystem<Fq>>(
        mut cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        if let Boolean::Constant(cond) = *cond {
            if cond { Ok(first.clone()) } else { Ok(second.clone()) }
        } else {
            let first_gadget = first.allocated(&mut cs)?;
            let second_gadget = second.allocated(&mut cs)?;
            let result = EdwardsBlsGadget::conditionally_select(cs, cond, &first_gadget, &second_gadget)?;

            Ok(EdwardsGroupType::Allocated(result))
        }
    }

    fn cost() -> usize {
        2 * <EdwardsBlsGadget as CondSelectGadget<Fq>>::cost()
    }
}

impl ToBitsGadget<Fq> for EdwardsGroupType {
    fn to_bits<CS: ConstraintSystem<Fq>>(&self, mut cs: CS) -> Result<Vec<Boolean>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bits(cs)
    }

    fn to_bits_strict<CS: ConstraintSystem<Fq>>(&self, mut cs: CS) -> Result<Vec<Boolean>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bits_strict(cs)
    }
}

impl ToBytesGadget<Fq> for EdwardsGroupType {
    fn to_bytes<CS: ConstraintSystem<Fq>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bytes(cs)
    }

    fn to_bytes_strict<CS: ConstraintSystem<Fq>>(&self, mut cs: CS) -> Result<Vec<UInt8>, SynthesisError> {
        let self_gadget = self.allocated(&mut cs)?;
        self_gadget.to_bytes_strict(cs)
    }
}

impl std::fmt::Display for EdwardsGroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EdwardsGroupType::Constant(constant) => write!(f, "{:?}", constant),
            EdwardsGroupType::Allocated(allocated) => write!(f, "{:?}", allocated),
        }
    }
}
