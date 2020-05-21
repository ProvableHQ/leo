use crate::errors::GroupElementError;
use crate::{ConstrainedProgram, ConstrainedValue, InputValue};

use snarkos_models::{
    curves::{Field, Group, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<F: Field + PrimeField, G: Group, CS: ConstraintSystem<F>> ConstrainedProgram<F, G, CS> {
    pub(crate) fn group_element_from_input(
        &mut self,
        _cs: &mut CS,
        _name: String,
        _private: bool,
        input_value: Option<InputValue<F, G>>,
    ) -> Result<ConstrainedValue<F, G>, GroupElementError> {
        // Check that the parameter value is the correct type
        // let group_option = match input_value {
        //     Some(input) => {
        //         if let InputValue::Group(group) = input {
        //             Some(group)
        //         } else {
        //             return Err(GroupElementError::InvalidGroup(input.to_string()));
        //         }
        //     }
        //     None => None,
        // };
        //
        // // Check visibility of parameter
        // let group_value = if private {
        //     cs.alloc(
        //         || name,
        //         || group_option.ok_or(SynthesisError::AssignmentMissing),
        //     )?
        // } else {
        //     cs.alloc_input(
        //         || name,
        //         || group_option.ok_or(SynthesisError::AssignmentMissing),
        //     )?
        // };
        //
        // Ok(ConstrainedValue::GroupElement())

        // TODO: use group gadget to allocate groups
        if let Some(InputValue::Group(group)) = input_value {
            return Ok(ConstrainedValue::GroupElement(group));
        }

        Ok(ConstrainedValue::GroupElement(G::default()))
    }

    pub fn evaluate_group_eq(group_element_1: G, group_element_2: G) -> ConstrainedValue<F, G> {
        ConstrainedValue::Boolean(Boolean::constant(group_element_1.eq(&group_element_2)))
    }

    pub fn evaluate_group_add(group_element_1: G, group_element_2: G) -> ConstrainedValue<F, G> {
        ConstrainedValue::GroupElement(group_element_1.add(&group_element_2))
    }

    pub fn evaluate_group_sub(group_element_1: G, group_element_2: G) -> ConstrainedValue<F, G> {
        ConstrainedValue::GroupElement(group_element_1.sub(&group_element_2))
    }
    //
    // pub fn evaluate_group_mul(group_element: G, scalar_field: G::ScalarField) -> ConstrainedValue<F, G> {
    //     ConstrainedValue::GroupElement(group_element.mul(&scalar_field))
    // }
}
