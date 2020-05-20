use crate::{ConstrainedProgram, ConstrainedValue};

use snarkos_models::{
    curves::{Field, Group, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<F: Field + PrimeField, G: Group, CS: ConstraintSystem<F>> ConstrainedProgram<F, G, CS> {
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
