use crate::errors::GroupElementError;
use crate::InputValue;

use snarkos_curves::templates::twisted_edwards_extended::GroupAffine;
use snarkos_errors::gadgets::SynthesisError;
use snarkos_gadgets::curves::templates::twisted_edwards::AffineGadget;
use snarkos_models::{
    curves::{Field, PrimeField, TEModelParameters},
    gadgets::{
        curves::{FieldGadget, GroupGadget},
        r1cs::ConstraintSystem,
        utilities::{alloc::AllocGadget, boolean::Boolean, select::CondSelectGadget},
    },
};
use std::fmt;
use std::ops::{Add, Sub};

/// An affine group constant or allocated affine point
#[derive(Clone, PartialEq, Eq)]
pub enum GroupElement<
    P: std::clone::Clone + TEModelParameters,
    F: Field + PrimeField,
    FG: FieldGadget<P::BaseField, F>,
> {
    Constant(GroupAffine<P>),
    Allocated(AffineGadget<P, F, FG>),
}

impl<
        P: std::clone::Clone + TEModelParameters,
        F: Field + PrimeField,
        FG: FieldGadget<P::BaseField, F>,
    > GroupElement<P, F, FG>
{
    pub(crate) fn new_constant(x: P::BaseField, y: P::BaseField) -> Self {
        GroupElement::Constant(GroupAffine::new(x, y))
    }

    pub(crate) fn new_allocated<CS: ConstraintSystem<F>>(
        cs: &mut CS,
        name: String,
        private: bool,
        input_value: Option<InputValue<P::BaseField, F>>,
    ) -> Result<Self, GroupElementError> {
        //Check that the parameter value is the correct type
        let group_option = match input_value {
            Some(input) => {
                if let InputValue::Group(x, y) = input {
                    Some(GroupAffine::new(x, y))
                } else {
                    return Err(GroupElementError::InvalidGroup(input.to_string()));
                }
            }
            None => None,
        };

        // Check visibility of parameter
        let group_value = if private {
            AffineGadget::<P, F, FG>::alloc(&mut cs.ns(|| name), || {
                group_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        } else {
            AffineGadget::<P, F, FG>::alloc_input(&mut cs.ns(|| name), || {
                group_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        };

        Ok(GroupElement::Allocated(group_value))
    }

    pub fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (GroupElement::Constant(ge_1), GroupElement::Constant(ge_2)) => ge_1 == ge_2,
            (GroupElement::Allocated(ge_1), GroupElement::Allocated(ge_2)) => ge_1.eq(ge_2),
            (GroupElement::Allocated(ge_gadget), GroupElement::Constant(ge_constant))
            | (GroupElement::Constant(ge_constant), GroupElement::Allocated(ge_gadget)) => {
                match GroupGadget::<GroupAffine<P>, F>::get_value(ge_gadget) {
                    Some(value) => value == *ge_constant,
                    None => false,
                }
            }
        }
    }

    pub fn enforce_add<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: &Self,
    ) -> Result<Self, GroupElementError> {
        Ok(match (self, other) {
            (GroupElement::Constant(ge_1), GroupElement::Constant(ge_2)) => {
                GroupElement::Constant(ge_1.add(ge_2))
            }
            (GroupElement::Allocated(ref ge_1), GroupElement::Allocated(ge_2)) => {
                GroupElement::Allocated(GroupGadget::<GroupAffine<P>, F>::add(
                    ge_1,
                    cs.ns(|| "group add"),
                    ge_2,
                )?)
            }
            (
                GroupElement::Allocated(ref ge_allocated),
                GroupElement::Constant(ref ge_constant),
            )
            | (
                GroupElement::Constant(ref ge_constant),
                GroupElement::Allocated(ref ge_allocated),
            ) => GroupElement::Allocated(
                ge_allocated.add_constant(cs.ns(|| "group add"), ge_constant)?,
            ),
        })
    }

    pub fn enforce_sub<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
        other: &Self,
    ) -> Result<Self, GroupElementError> {
        Ok(match (self, other) {
            (GroupElement::Constant(ge_1), GroupElement::Constant(ge_2)) => {
                GroupElement::Constant(ge_1.sub(ge_2))
            }
            (GroupElement::Allocated(ref ge_1), GroupElement::Allocated(ge_2)) => {
                GroupElement::Allocated(GroupGadget::<GroupAffine<P>, F>::sub(
                    ge_1,
                    cs.ns(|| "group sub"),
                    ge_2,
                )?)
            }
            (
                GroupElement::Allocated(ref ge_allocated),
                GroupElement::Constant(ref ge_constant),
            ) => GroupElement::Allocated(
                ge_allocated.sub_constant(cs.ns(|| "group sub"), ge_constant)?,
            ),
            (_, _) => unimplemented!(
                "cannot subtract allocated group element from constant group element"
            ),
        })
    }

    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GroupElement::Constant(ref constant) => write!(f, "{:?}", constant),
            GroupElement::Allocated(ref allocated) => write!(f, "{:?}", allocated),
        }
    }
}

impl<
        P: std::clone::Clone + TEModelParameters,
        F: Field + PrimeField,
        FG: FieldGadget<P::BaseField, F>,
    > fmt::Display for GroupElement<P, F, FG>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<
        P: std::clone::Clone + TEModelParameters,
        F: Field + PrimeField,
        FG: FieldGadget<P::BaseField, F>,
    > fmt::Debug for GroupElement<P, F, FG>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<
        P: std::clone::Clone + TEModelParameters,
        F: Field + PrimeField,
        FG: FieldGadget<P::BaseField, F>,
    > CondSelectGadget<F> for GroupElement<P, F, FG>
{
    fn conditionally_select<CS: ConstraintSystem<F>>(
        cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        match (first, second) {
            (GroupElement::Allocated(ge_1), GroupElement::Allocated(ge_2)) => Ok(
                GroupElement::Allocated(AffineGadget::conditionally_select(cs, cond, ge_1, ge_2)?),
            ),
            (_, _) => Err(SynthesisError::Unsatisfiable), // types do not match
        }
    }

    fn cost() -> usize {
        AffineGadget::<P, F, FG>::cost()
    }
}
