//! Methods to enforce constraints on field elements in a resolved Leo program.

use crate::{
    constraints::{ConstrainedProgram, ConstrainedValue},
    errors::FieldElementError,
    types::InputValue,
};

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::curves::TEModelParameters;
use snarkos_models::gadgets::curves::{FieldGadget, FpGadget};
use snarkos_models::gadgets::utilities::alloc::AllocGadget;
use snarkos_models::gadgets::utilities::boolean::Boolean;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::r1cs::ConstraintSystem,
};
use std::fmt;
use snarkos_models::gadgets::utilities::select::CondSelectGadget;

/// A constant or allocated element in the field
#[derive(Clone, PartialEq, Eq)]
pub enum FieldElement<F: Field + PrimeField> {
    Constant(F),
    Allocated(FpGadget<F>),
}

impl<F: Field + PrimeField> FieldElement<F> {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FieldElement::Constant(ref constant) => write!(f, "{}", constant),
            FieldElement::Allocated(ref allocated) => write!(f, "{:?}", allocated),
        }
    }
}

impl<F: Field + PrimeField> fmt::Display for FieldElement<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<F: Field + PrimeField> fmt::Debug for FieldElement<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl<F: Field + PrimeField> CondSelectGadget<F> for FieldElement<F> {
    fn conditionally_select<CS: ConstraintSystem<F>>(
        cs: CS,
        cond: &Boolean,
        first: &Self,
        second: &Self,
    ) -> Result<Self, SynthesisError> {
        match (first, second) {
            (FieldElement::Allocated(fe_1), FieldElement::Allocated(fe_2)) => Ok(FieldElement::Allocated(
                FpGadget::<F>::conditionally_select(cs, cond, fe_1, fe_2)?
            )),
            (_, _) => Err(SynthesisError::Unsatisfiable), // types do not match
        }
    }

    fn cost() -> usize {
        FpGadget::<F>::cost()
    }
}


impl<
        P: std::clone::Clone + TEModelParameters,
        F: Field + PrimeField,
        FG: FieldGadget<P::BaseField, F>,
        CS: ConstraintSystem<F>,
    > ConstrainedProgram<P, F, FG, CS>
{
    pub(crate) fn field_element_from_input(
        &mut self,
        cs: &mut CS,
        name: String,
        private: bool,
        input_value: Option<InputValue<P::BaseField, F>>,
    ) -> Result<ConstrainedValue<P, F, FG>, FieldElementError> {
        // Check that the parameter value is the correct type
        let field_option = match input_value {
            Some(input) => {
                if let InputValue::Field(fe) = input {
                    Some(fe)
                } else {
                    return Err(FieldElementError::InvalidField(input.to_string()));
                }
            }
            None => None,
        };

        // Check visibility of parameter
        let field_value = if private {
            FpGadget::<F>::alloc(&mut cs.ns(|| name), || {
                field_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        } else {
            FpGadget::<F>::alloc_input(&mut cs.ns(|| name), || {
                field_option.ok_or(SynthesisError::AssignmentMissing)
            })?
        };

        Ok(ConstrainedValue::FieldElement(FieldElement::Allocated(
            field_value,
        )))
    }

    pub(crate) fn get_field_element_constant(constant: F) -> ConstrainedValue<P, F, FG> {
        ConstrainedValue::FieldElement(FieldElement::Constant(constant))
    }

    pub(crate) fn evaluate_field_eq(
        fe_1: FieldElement<F>,
        fe_2: FieldElement<F>,
    ) -> ConstrainedValue<P, F, FG> {
        let result = match (fe_1, fe_2) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                fe_1_constant.eq(&fe_2_constant)
            }
            (FieldElement::Allocated(fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                fe_1_gadget.eq(&fe_2_gadget)
            }
            (FieldElement::Allocated(fe_gadget), FieldElement::Constant(fe_constant))
            | (FieldElement::Constant(fe_constant), FieldElement::Allocated(fe_gadget)) => {
                match fe_gadget.value {
                    Some(value) => value.eq(&fe_constant),
                    None => false,
                }
            }
        };

        ConstrainedValue::Boolean(Boolean::Constant(result))
    }

    pub(crate) fn evaluate_field_ge(
        fe_1: FieldElement<F>,
        fe_2: FieldElement<F>,
    ) -> ConstrainedValue<P, F, FG> {
        let result = match (fe_1, fe_2) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                fe_1_constant.ge(&fe_2_constant)
            }
            (FieldElement::Allocated(fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                match (fe_1_gadget.value, fe_2_gadget.value) {
                    (Some(value_1), Some(value_2)) => value_1.ge(&value_2),
                    (_, _) => false,
                }
            }
            (FieldElement::Allocated(fe_gadget), FieldElement::Constant(fe_constant))
            | (FieldElement::Constant(fe_constant), FieldElement::Allocated(fe_gadget)) => {
                match fe_gadget.value {
                    Some(value) => value.ge(&fe_constant),
                    None => false,
                }
            }
        };

        ConstrainedValue::Boolean(Boolean::Constant(result))
    }

    pub(crate) fn evaluate_field_gt(
        fe_1: FieldElement<F>,
        fe_2: FieldElement<F>,
    ) -> ConstrainedValue<P, F, FG> {
        let result = match (fe_1, fe_2) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                fe_1_constant.gt(&fe_2_constant)
            }
            (FieldElement::Allocated(fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                match (fe_1_gadget.value, fe_2_gadget.value) {
                    (Some(value_1), Some(value_2)) => value_1.gt(&value_2),
                    (_, _) => false,
                }
            }
            (FieldElement::Allocated(fe_gadget), FieldElement::Constant(fe_constant))
            | (FieldElement::Constant(fe_constant), FieldElement::Allocated(fe_gadget)) => {
                match fe_gadget.value {
                    Some(value) => value.gt(&fe_constant),
                    None => false,
                }
            }
        };

        ConstrainedValue::Boolean(Boolean::Constant(result))
    }

    pub(crate) fn evaluate_field_le(
        fe_1: FieldElement<F>,
        fe_2: FieldElement<F>,
    ) -> ConstrainedValue<P, F, FG> {
        let result = match (fe_1, fe_2) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                fe_1_constant.le(&fe_2_constant)
            }
            (FieldElement::Allocated(fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                match (fe_1_gadget.value, fe_2_gadget.value) {
                    (Some(value_1), Some(value_2)) => value_1.le(&value_2),
                    (_, _) => false,
                }
            }
            (FieldElement::Allocated(fe_gadget), FieldElement::Constant(fe_constant))
            | (FieldElement::Constant(fe_constant), FieldElement::Allocated(fe_gadget)) => {
                match fe_gadget.value {
                    Some(value) => value.le(&fe_constant),
                    None => false,
                }
            }
        };

        ConstrainedValue::Boolean(Boolean::Constant(result))
    }

    pub(crate) fn evaluate_field_lt(
        fe_1: FieldElement<F>,
        fe_2: FieldElement<F>,
    ) -> ConstrainedValue<P, F, FG> {
        let result = match (fe_1, fe_2) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                fe_1_constant.lt(&fe_2_constant)
            }
            (FieldElement::Allocated(fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                match (fe_1_gadget.value, fe_2_gadget.value) {
                    (Some(value_1), Some(value_2)) => value_1.lt(&value_2),
                    (_, _) => false,
                }
            }
            (FieldElement::Allocated(fe_gadget), FieldElement::Constant(fe_constant))
            | (FieldElement::Constant(fe_constant), FieldElement::Allocated(fe_gadget)) => {
                match fe_gadget.value {
                    Some(value) => value.lt(&fe_constant),
                    None => false,
                }
            }
        };

        ConstrainedValue::Boolean(Boolean::Constant(result))
    }

    pub(crate) fn enforce_field_add(
        cs: &mut CS,
        fe_1: FieldElement<F>,
        fe_2: FieldElement<F>,
    ) -> Result<ConstrainedValue<P, F, FG>, FieldElementError> {
        Ok(ConstrainedValue::FieldElement(match (fe_1, fe_2) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                FieldElement::Constant(fe_1_constant.add(&fe_2_constant))
            }
            (FieldElement::Allocated(fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                FieldElement::Allocated(fe_1_gadget.add(cs.ns(|| "field add"), &fe_2_gadget)?)
            }
            (FieldElement::Allocated(fe_gadget), FieldElement::Constant(fe_constant))
            | (FieldElement::Constant(fe_constant), FieldElement::Allocated(fe_gadget)) => {
                FieldElement::Allocated(
                    fe_gadget.add_constant(cs.ns(|| "field add"), &fe_constant)?,
                )
            }
        }))
    }

    pub(crate) fn enforce_field_sub(
        cs: &mut CS,
        fe_1: FieldElement<F>,
        fe_2: FieldElement<F>,
    ) -> Result<ConstrainedValue<P, F, FG>, FieldElementError> {
        Ok(ConstrainedValue::FieldElement(match (fe_1, fe_2) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                FieldElement::Constant(fe_1_constant.sub(&fe_2_constant))
            }
            (FieldElement::Allocated(fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                FieldElement::Allocated(
                    fe_1_gadget.sub(cs.ns(|| "field subtraction"), &fe_2_gadget)?,
                )
            }
            (FieldElement::Allocated(fe_gadget), FieldElement::Constant(fe_constant)) => {
                FieldElement::Allocated(
                    fe_gadget.sub_constant(cs.ns(|| "field subtraction"), &fe_constant)?,
                )
            }
            (_, _) => unimplemented!("field subtraction between constant and allocated not impl"),
        }))
    }

    pub(crate) fn enforce_field_mul(
        cs: &mut CS,
        fe_1: FieldElement<F>,
        fe_2: FieldElement<F>,
    ) -> Result<ConstrainedValue<P, F, FG>, FieldElementError> {
        Ok(ConstrainedValue::FieldElement(match (fe_1, fe_2) {
            (FieldElement::Constant(fe_1_constant), FieldElement::Constant(fe_2_constant)) => {
                FieldElement::Constant(fe_1_constant.mul(&fe_2_constant))
            }
            (FieldElement::Allocated(fe_1_gadget), FieldElement::Allocated(fe_2_gadget)) => {
                FieldElement::Allocated(
                    fe_1_gadget.mul(cs.ns(|| "field multiplication"), &fe_2_gadget)?,
                )
            }
            (FieldElement::Allocated(fe_gadget), FieldElement::Constant(fe_constant))
            | (FieldElement::Constant(fe_constant), FieldElement::Allocated(fe_gadget)) => {
                FieldElement::Allocated(
                    fe_gadget.mul_by_constant(cs.ns(|| "field multiplication"), &fe_constant)?,
                )
            }
        }))
    }
}
