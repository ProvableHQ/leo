use crate::errors::GroupElementError;
use crate::{ConstrainedProgram, ConstrainedValue};

use snarkos_curves::templates::twisted_edwards_extended::GroupAffine;
use snarkos_gadgets::curves::templates::twisted_edwards::AffineGadget;
use snarkos_models::curves::TEModelParameters;
use snarkos_models::gadgets::curves::{FieldGadget, GroupGadget};
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

impl<
        P: std::clone::Clone + TEModelParameters,
        F: Field + PrimeField + std::borrow::Borrow<P::BaseField>,
        FG: FieldGadget<P::BaseField, F>,
        CS: ConstraintSystem<F>,
    > ConstrainedProgram<P, F, FG, CS>
{
    pub(crate) fn get_group_element_pair(
        cs: &mut CS,
        x: P::BaseField,
        y: P::BaseField,
    ) -> Result<ConstrainedValue<P, F, FG>, GroupElementError> {
        let x = FG::alloc(cs.ns(|| "x"), || Ok(x))?;
        let y = FG::alloc(cs.ns(|| "y"), || Ok(y))?;

        Ok(ConstrainedValue::GroupElement(AffineGadget::new(x, y)))
    }

    // pub(crate) fn group_element_from_input(
    //     &mut self,
    //     _cs: &mut CS,
    //     _name: String,
    //     _private: bool,
    //     input_value: Option<InputValue<NativeF, F>>,
    // ) -> Result<ConstrainedValue<P, F, FG>, GroupElementError> {
    //     // Check that the parameter value is the correct type
    //     // let group_option = match input_value {
    //     //     Some(input) => {
    //     //         if let InputValue::Group(group) = input {
    //     //             Some(group)
    //     //         } else {
    //     //             return Err(GroupElementError::InvalidGroup(input.to_string()));
    //     //         }
    //     //     }
    //     //     None => None,
    //     // };
    //     //
    //     // // Check visibility of parameter
    //     // let group_value = if private {
    //     //     cs.alloc(
    //     //         || name,
    //     //         || group_option.ok_or(SynthesisError::AssignmentMissing),
    //     //     )?
    //     // } else {
    //     //     cs.alloc_input(
    //     //         || name,
    //     //         || group_option.ok_or(SynthesisError::AssignmentMissing),
    //     //     )?
    //     // };
    //     //
    //     // Ok(ConstrainedValue::GroupElement())
    //
    //     // TODO: use group gadget to allocate groups
    //     if let Some(InputValue::Group(x, y)) = input_value {
    //         return Ok(ConstrainedValue::GroupElement(group));
    //     }
    //
    //     Ok(ConstrainedValue::GroupElement(G::default()))
    // }

    pub fn evaluate_group_eq(
        group_element_1: AffineGadget<P, F, FG>,
        group_element_2: AffineGadget<P, F, FG>,
    ) -> ConstrainedValue<P, F, FG> {
        ConstrainedValue::Boolean(Boolean::constant(group_element_1.eq(&group_element_2)))
    }

    pub fn enforce_group_add(
        cs: &mut CS,
        group_element_1: AffineGadget<P, F, FG>,
        group_element_2: AffineGadget<P, F, FG>,
    ) -> Result<ConstrainedValue<P, F, FG>, GroupElementError> {
        let result = GroupGadget::<GroupAffine<P>, F>::add(
            &group_element_1,
            cs.ns(|| "group add"),
            &group_element_2,
        )?;
        Ok(ConstrainedValue::GroupElement(result))
    }

    pub fn enforce_group_sub(
        cs: &mut CS,
        group_element_1: AffineGadget<P, F, FG>,
        group_element_2: AffineGadget<P, F, FG>,
    ) -> Result<ConstrainedValue<P, F, FG>, GroupElementError> {
        let result = GroupGadget::<GroupAffine<P>, F>::sub(
            &group_element_1,
            cs.ns(|| "group sub"),
            &group_element_2,
        )?;
        Ok(ConstrainedValue::GroupElement(result))
    }
}
