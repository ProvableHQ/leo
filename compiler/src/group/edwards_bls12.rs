use crate::errors::GroupError;
use crate::GroupType;

use snarkos_curves::edwards_bls12::{EdwardsAffine, EdwardsParameters, Fq};
use snarkos_curves::templates::twisted_edwards_extended::GroupAffine;
use snarkos_errors::gadgets::SynthesisError;
use snarkos_gadgets::curves::edwards_bls12::EdwardsBlsGadget;
use snarkos_models::curves::{AffineCurve, ModelParameters};
use snarkos_models::gadgets::curves::{FpGadget, GroupGadget};
use snarkos_models::gadgets::r1cs::ConstraintSystem;
use snarkos_models::gadgets::utilities::boolean::Boolean;
use snarkos_models::gadgets::utilities::eq::{ConditionalEqGadget, EqGadget};
use std::ops::Sub;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum EdwardsGroupType {
    Constant(EdwardsAffine),
    Allocated(EdwardsBlsGadget),
}

impl GroupType<<EdwardsParameters as ModelParameters>::BaseField, Fq> for EdwardsGroupType {
    fn constant(string: String) -> Result<Self, GroupError> {
        // 0 or (0, 1)
        let result =
            match Fq::from_str(&string).ok() {
                Some(x) => EdwardsAffine::get_point_from_x(x, false)
                    .ok_or(GroupError::InvalidGroup(string))?,
                None => EdwardsAffine::from_str(&string)
                    .map_err(|_| GroupError::InvalidGroup(string))?,
            };

        Ok(EdwardsGroupType::Constant(result))
    }

    fn add<CS: ConstraintSystem<Fq>>(&self, cs: CS, other: &Self) -> Result<Self, GroupError> {
        match (self, other) {
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                Ok(EdwardsGroupType::Constant(self_value.add(other_value)))
            }

            (EdwardsGroupType::Allocated(self_value), EdwardsGroupType::Allocated(other_value)) => {
                let result = <EdwardsBlsGadget as GroupGadget<
                    GroupAffine<EdwardsParameters>,
                    Fq,
                >>::add(self_value, cs, other_value)?;
                Ok(EdwardsGroupType::Allocated(result))
            }

            (
                EdwardsGroupType::Constant(constant_value),
                EdwardsGroupType::Allocated(allocated_value),
            )
            | (
                EdwardsGroupType::Allocated(allocated_value),
                EdwardsGroupType::Constant(constant_value),
            ) => Ok(EdwardsGroupType::Allocated(
                allocated_value.add_constant(cs, constant_value)?,
            )),
        }
    }

    fn sub<CS: ConstraintSystem<Fq>>(&self, cs: CS, other: &Self) -> Result<Self, GroupError> {
        match (self, other) {
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                Ok(EdwardsGroupType::Constant(self_value.sub(other_value)))
            }

            (EdwardsGroupType::Allocated(self_value), EdwardsGroupType::Allocated(other_value)) => {
                let result = <EdwardsBlsGadget as GroupGadget<
                    GroupAffine<EdwardsParameters>,
                    Fq,
                >>::sub(self_value, cs, other_value)?;
                Ok(EdwardsGroupType::Allocated(result))
            }

            (
                EdwardsGroupType::Constant(constant_value),
                EdwardsGroupType::Allocated(allocated_value),
            )
            | (
                EdwardsGroupType::Allocated(allocated_value),
                EdwardsGroupType::Constant(constant_value),
            ) => Ok(EdwardsGroupType::Allocated(
                allocated_value.sub_constant(cs, constant_value)?,
            )),
        }
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

            (
                EdwardsGroupType::Constant(constant_value),
                EdwardsGroupType::Allocated(allocated_value),
            )
            | (
                EdwardsGroupType::Allocated(allocated_value),
                EdwardsGroupType::Constant(constant_value),
            ) => <EdwardsBlsGadget as GroupGadget<GroupAffine<EdwardsParameters>, Fq>>::get_value(
                allocated_value,
            )
            .map(|allocated_value| allocated_value == *constant_value)
            .unwrap_or(false),
        }
    }
}

impl Eq for EdwardsGroupType {}

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
                return <EdwardsBlsGadget>::conditional_enforce_equal(
                    self_value,
                    cs,
                    other_value,
                    condition,
                )
            }
            // c - a = a - c
            (
                EdwardsGroupType::Constant(constant_value),
                EdwardsGroupType::Allocated(allocated_value),
            )
            | (
                EdwardsGroupType::Allocated(allocated_value),
                EdwardsGroupType::Constant(constant_value),
            ) => {
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
