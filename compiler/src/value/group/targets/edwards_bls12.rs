// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{errors::GroupError, GroupType};
use leo_ast::{GroupCoordinate, GroupTuple, GroupValue, Span};

use snarkos_curves::{
    edwards_bls12::{EdwardsAffine, EdwardsParameters, Fq},
    templates::twisted_edwards_extended::GroupAffine,
};
use snarkos_errors::gadgets::SynthesisError;
use snarkos_gadgets::curves::edwards_bls12::EdwardsBlsGadget;
use snarkos_models::{
    curves::{AffineCurve, Fp256, One, TEModelParameters, Zero},
    gadgets::{
        curves::{FieldGadget, FpGadget, GroupGadget},
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
use std::{
    borrow::Borrow,
    ops::{Mul, Neg, Sub},
    str::FromStr,
};

#[derive(Clone, Debug)]
pub enum EdwardsGroupType {
    Constant(EdwardsAffine),
    Allocated(Box<EdwardsBlsGadget>),
}

impl GroupType<Fq> for EdwardsGroupType {
    fn constant(group: GroupValue) -> Result<Self, GroupError> {
        let value = Self::edwards_affine_from_value(group)?;

        Ok(EdwardsGroupType::Constant(value))
    }

    fn to_allocated<CS: ConstraintSystem<Fq>>(&self, mut cs: CS, span: &Span) -> Result<Self, GroupError> {
        self.allocated(cs.ns(|| format!("allocate affine point {}:{}", span.line, span.start)))
            .map(|ebg| EdwardsGroupType::Allocated(Box::new(ebg)))
            .map_err(|error| GroupError::synthesis_error(error, span.to_owned()))
    }

    fn negate<CS: ConstraintSystem<Fq>>(&self, cs: CS, span: &Span) -> Result<Self, GroupError> {
        match self {
            EdwardsGroupType::Constant(group) => Ok(EdwardsGroupType::Constant(group.neg())),
            EdwardsGroupType::Allocated(group) => {
                let result = <EdwardsBlsGadget as GroupGadget<GroupAffine<EdwardsParameters>, Fq>>::negate(group, cs)
                    .map_err(|e| GroupError::negate_operation(e, span.to_owned()))?;

                Ok(EdwardsGroupType::Allocated(Box::new(result)))
            }
        }
    }

    fn add<CS: ConstraintSystem<Fq>>(&self, cs: CS, other: &Self, span: &Span) -> Result<Self, GroupError> {
        match (self, other) {
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                Ok(EdwardsGroupType::Constant(self_value.add(other_value)))
            }

            (EdwardsGroupType::Allocated(self_value), EdwardsGroupType::Allocated(other_value)) => {
                let result = <EdwardsBlsGadget as GroupGadget<GroupAffine<EdwardsParameters>, Fq>>::add(
                    self_value,
                    cs,
                    other_value,
                )
                .map_err(|e| GroupError::binary_operation("+".to_string(), e, span.to_owned()))?;

                Ok(EdwardsGroupType::Allocated(Box::new(result)))
            }

            (EdwardsGroupType::Constant(constant_value), EdwardsGroupType::Allocated(allocated_value))
            | (EdwardsGroupType::Allocated(allocated_value), EdwardsGroupType::Constant(constant_value)) => {
                Ok(EdwardsGroupType::Allocated(Box::new(
                    allocated_value
                        .add_constant(cs, constant_value)
                        .map_err(|e| GroupError::binary_operation("+".to_string(), e, span.to_owned()))?,
                )))
            }
        }
    }

    fn sub<CS: ConstraintSystem<Fq>>(&self, cs: CS, other: &Self, span: &Span) -> Result<Self, GroupError> {
        match (self, other) {
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                Ok(EdwardsGroupType::Constant(self_value.sub(other_value)))
            }

            (EdwardsGroupType::Allocated(self_value), EdwardsGroupType::Allocated(other_value)) => {
                let result = <EdwardsBlsGadget as GroupGadget<GroupAffine<EdwardsParameters>, Fq>>::sub(
                    self_value,
                    cs,
                    other_value,
                )
                .map_err(|e| GroupError::binary_operation("-".to_string(), e, span.to_owned()))?;

                Ok(EdwardsGroupType::Allocated(Box::new(result)))
            }

            (EdwardsGroupType::Constant(constant_value), EdwardsGroupType::Allocated(allocated_value))
            | (EdwardsGroupType::Allocated(allocated_value), EdwardsGroupType::Constant(constant_value)) => {
                Ok(EdwardsGroupType::Allocated(Box::new(
                    allocated_value
                        .sub_constant(cs, constant_value)
                        .map_err(|e| GroupError::binary_operation("-".to_string(), e, span.to_owned()))?,
                )))
            }
        }
    }
}

impl EdwardsGroupType {
    pub fn edwards_affine_from_value(value: GroupValue) -> Result<EdwardsAffine, GroupError> {
        match value {
            GroupValue::Single(number, span) => Self::edwards_affine_from_single(number, span),
            GroupValue::Tuple(tuple) => Self::edwards_affine_from_tuple(tuple),
        }
    }

    pub fn edwards_affine_from_single(number: String, span: Span) -> Result<EdwardsAffine, GroupError> {
        if number.eq("0") {
            Ok(EdwardsAffine::zero())
        } else {
            let one = edwards_affine_one();
            let number_value = Fp256::from_str(&number).map_err(|_| GroupError::n_group(number, span))?;
            let result: EdwardsAffine = one.mul(&number_value);

            Ok(result)
        }
    }

    pub fn edwards_affine_from_tuple(group: GroupTuple) -> Result<EdwardsAffine, GroupError> {
        let span = group.span;
        let x = group.x;
        let y = group.y;

        match (x, y) {
            // (x, y)
            (GroupCoordinate::Number(x_string, x_span), GroupCoordinate::Number(y_string, y_span)) => {
                Self::edwards_affine_from_pair(x_string, y_string, x_span, y_span, span)
            }
            // (x, +)
            (GroupCoordinate::Number(x_string, x_span), GroupCoordinate::SignHigh) => {
                Self::edwards_affine_from_x_str(x_string, x_span, Some(true), span)
            }
            // (x, -)
            (GroupCoordinate::Number(x_string, x_span), GroupCoordinate::SignLow) => {
                Self::edwards_affine_from_x_str(x_string, x_span, Some(false), span)
            }
            // (x, _)
            (GroupCoordinate::Number(x_string, x_span), GroupCoordinate::Inferred) => {
                Self::edwards_affine_from_x_str(x_string, x_span, None, span)
            }
            // (+, y)
            (GroupCoordinate::SignHigh, GroupCoordinate::Number(y_string, y_span)) => {
                Self::edwards_affine_from_y_str(y_string, y_span, Some(true), span)
            }
            // (-, y)
            (GroupCoordinate::SignLow, GroupCoordinate::Number(y_string, y_span)) => {
                Self::edwards_affine_from_y_str(y_string, y_span, Some(false), span)
            }
            // (_, y)
            (GroupCoordinate::Inferred, GroupCoordinate::Number(y_string, y_span)) => {
                Self::edwards_affine_from_y_str(y_string, y_span, None, span)
            }
            // Invalid
            (x, y) => Err(GroupError::invalid_group(format!("({}, {})", x, y), span)),
        }
    }

    pub fn edwards_affine_from_x_str(
        x_string: String,
        x_span: Span,
        greatest: Option<bool>,
        element_span: Span,
    ) -> Result<EdwardsAffine, GroupError> {
        let x = Fq::from_str(&x_string).map_err(|_| GroupError::x_invalid(x_string, x_span))?;
        match greatest {
            // Sign provided
            Some(greatest) => {
                EdwardsAffine::from_x_coordinate(x, greatest).ok_or_else(|| GroupError::x_recover(element_span))
            }
            // Sign inferred
            None => {
                // Attempt to recover with a sign_low bit.
                if let Some(element) = EdwardsAffine::from_x_coordinate(x, false) {
                    return Ok(element);
                }

                // Attempt to recover with a sign_high bit.
                if let Some(element) = EdwardsAffine::from_x_coordinate(x, true) {
                    return Ok(element);
                }

                // Otherwise return error.
                Err(GroupError::x_recover(element_span))
            }
        }
    }

    pub fn edwards_affine_from_y_str(
        y_string: String,
        y_span: Span,
        greatest: Option<bool>,
        element_span: Span,
    ) -> Result<EdwardsAffine, GroupError> {
        let y = Fq::from_str(&y_string).map_err(|_| GroupError::y_invalid(y_string, y_span))?;

        match greatest {
            // Sign provided
            Some(greatest) => {
                EdwardsAffine::from_y_coordinate(y, greatest).ok_or_else(|| GroupError::y_recover(element_span))
            }
            // Sign inferred
            None => {
                // Attempt to recover with a sign_low bit.
                if let Some(element) = EdwardsAffine::from_y_coordinate(y, false) {
                    return Ok(element);
                }

                // Attempt to recover with a sign_high bit.
                if let Some(element) = EdwardsAffine::from_y_coordinate(y, true) {
                    return Ok(element);
                }

                // Otherwise return error.
                Err(GroupError::y_recover(element_span))
            }
        }
    }

    pub fn edwards_affine_from_pair(
        x_string: String,
        y_string: String,
        x_span: Span,
        y_span: Span,
        element_span: Span,
    ) -> Result<EdwardsAffine, GroupError> {
        let x = Fq::from_str(&x_string).map_err(|_| GroupError::x_invalid(x_string, x_span))?;
        let y = Fq::from_str(&y_string).map_err(|_| GroupError::y_invalid(y_string, y_span))?;

        let element = EdwardsAffine::new(x, y);

        if element.is_on_curve() {
            Ok(element)
        } else {
            Err(GroupError::not_on_curve(element.to_string(), element_span))
        }
    }

    pub fn alloc_helper<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<GroupValue>>(
        value_gen: Fn,
    ) -> Result<EdwardsAffine, SynthesisError> {
        let group_value = match value_gen() {
            Ok(value) => {
                let group_value = value.borrow().clone();
                Ok(group_value)
            }
            _ => Err(SynthesisError::AssignmentMissing),
        }?;

        Self::edwards_affine_from_value(group_value).map_err(|_| SynthesisError::AssignmentMissing)
    }

    pub fn allocated<CS: ConstraintSystem<Fq>>(&self, mut cs: CS) -> Result<EdwardsBlsGadget, SynthesisError> {
        match self {
            EdwardsGroupType::Constant(constant) => {
                <EdwardsBlsGadget as AllocGadget<GroupAffine<EdwardsParameters>, Fq>>::alloc(
                    &mut cs.ns(|| format!("{:?}", constant)),
                    || Ok(constant),
                )
            }
            EdwardsGroupType::Allocated(allocated) => {
                let x_value = allocated.x.get_value();
                let y_value = allocated.y.get_value();

                let x_allocated = FpGadget::alloc(cs.ns(|| "x"), || x_value.ok_or(SynthesisError::AssignmentMissing))?;
                let y_allocated = FpGadget::alloc(cs.ns(|| "y"), || y_value.ok_or(SynthesisError::AssignmentMissing))?;

                Ok(EdwardsBlsGadget::new(x_allocated, y_allocated))
            }
        }
    }
}

impl AllocGadget<GroupValue, Fq> for EdwardsGroupType {
    fn alloc<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<GroupValue>, CS: ConstraintSystem<Fq>>(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let value = <EdwardsBlsGadget as AllocGadget<GroupAffine<EdwardsParameters>, Fq>>::alloc(cs, || {
            Self::alloc_helper(value_gen)
        })?;

        Ok(EdwardsGroupType::Allocated(Box::new(value)))
    }

    fn alloc_input<Fn: FnOnce() -> Result<T, SynthesisError>, T: Borrow<GroupValue>, CS: ConstraintSystem<Fq>>(
        cs: CS,
        value_gen: Fn,
    ) -> Result<Self, SynthesisError> {
        let value = <EdwardsBlsGadget as AllocGadget<GroupAffine<EdwardsParameters>, Fq>>::alloc_input(cs, || {
            Self::alloc_helper(value_gen)
        })?;

        Ok(EdwardsGroupType::Allocated(Box::new(value)))
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

// fn compare_allocated_edwards_bls_gadgets<CS: ConstraintSystem<Fq>>(
//     mut cs: CS,
//     first: &EdwardsBlsGadget,
//     second: &EdwardsBlsGadget,
// ) -> Result<Boolean, SynthesisError> {
//     // compare x coordinates
//     let x_first = &first.x;
//     let x_second = &second.x;
//
//     let compare_x = x_first.evaluate_equal(&mut cs.ns(|| format!("compare x")), x_second)?;
//
//     // compare y coordinates
//     let y_first = &first.y;
//     let y_second = &second.y;
//
//     let compare_y = y_first.evaluate_equal(&mut cs.ns(|| format!("compare y")), y_second)?;
//
//     Boolean::and(
//         &mut cs.ns(|| format!("compare x and y results")),
//         &compare_x,
//         &compare_y,
//     )
// }

impl EvaluateEqGadget<Fq> for EdwardsGroupType {
    fn evaluate_equal<CS: ConstraintSystem<Fq>>(&self, mut _cs: CS, other: &Self) -> Result<Boolean, SynthesisError> {
        match (self, other) {
            (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) => {
                Ok(Boolean::constant(self_value.eq(other_value)))
            }
            _ => unimplemented!(),
            // (EdwardsGroupType::Allocated(first), EdwardsGroupType::Allocated(second)) => {
            //     compare_allocated_edwards_bls_gadgets(cs, first, second)
            // }
            // (EdwardsGroupType::Constant(constant_value), EdwardsGroupType::Allocated(allocated_value))
            // | (EdwardsGroupType::Allocated(allocated_value), EdwardsGroupType::Constant(constant_value)) => {
            //     let allocated_constant_value =
            //         <EdwardsBlsGadget as AllocGadget<GroupAffine<EdwardsParameters>, Fq>>::alloc(
            //             &mut cs.ns(|| format!("alloc constant for eq")),
            //             || Ok(constant_value),
            //         )?;
            //     compare_allocated_edwards_bls_gadgets(cs, allocated_value, &allocated_constant_value)
            // }
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
            let first_gadget = first.allocated(cs.ns(|| "first"))?;
            let second_gadget = second.allocated(cs.ns(|| "second"))?;
            let result = EdwardsBlsGadget::conditionally_select(cs, cond, &first_gadget, &second_gadget)?;

            Ok(EdwardsGroupType::Allocated(Box::new(result)))
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

fn edwards_affine_one() -> GroupAffine<EdwardsParameters> {
    let (x, y) = EdwardsParameters::AFFINE_GENERATOR_COEFFS;

    EdwardsAffine::new(x, y)
}

impl One for EdwardsGroupType {
    fn one() -> Self {
        let one = edwards_affine_one();

        Self::Constant(one)
    }

    fn is_one(&self) -> bool {
        self.eq(&Self::one())
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
