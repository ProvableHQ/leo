use crate::errors::GroupError;
use crate::GroupType;

use snarkos_curves::edwards_bls12::{EdwardsParameters, Fq};
use snarkos_curves::templates::twisted_edwards_extended::GroupAffine;
use snarkos_gadgets::curves::edwards_bls12::FqGadget;
use snarkos_gadgets::curves::templates::twisted_edwards::AffineGadget;
use snarkos_models::curves::ModelParameters;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum EdwardsGroupType {
    Constant(GroupAffine<EdwardsParameters>),
    Allocated(AffineGadget<EdwardsParameters, Fq, FqGadget>),
}

impl GroupType<<EdwardsParameters as ModelParameters>::BaseField, Fq> for EdwardsGroupType {
    fn constant(x: String, y: String) -> Result<Self, GroupError> {
        let x = <EdwardsParameters as ModelParameters>::BaseField::from_str(&x)
            .map_err(|_| GroupError::InvalidGroup(x))?;
        let y = <EdwardsParameters as ModelParameters>::BaseField::from_str(&y)
            .map_err(|_| GroupError::InvalidGroup(y))?;

        Ok(EdwardsGroupType::Constant(GroupAffine::new(x, y)))
    }

    // fn add<CS: ConstraintSystem<Fq>>(&self, cs: CS, other: &Self) -> Result<Self, GroupElementError> {
    //     match (self, other) {
    //         (EdwardsGroupType::Constant(self_value), EdwardsGroupType::Constant(other_value)) =>
    //             Ok(EdwardsGroupType::Constant(self_value.add(other_value))),
    //
    //         (EdwardsGroupType::Allocated(self_value), EdwardsGroupType::Allocated(other_value)) => {
    //             let result = <AffineGadget<EdwardsParameters, Fq, FqGadget> as GroupGadget<GroupAffine<EdwardsParameters>, Fq>>::add(self_value, cs, other_value)?;
    //             Ok(EdwardsGroupType::Allocated(result))
    //         }
    //
    //         (EdwardsGroupType::Constant(constant_value), EdwardsGroupType::Allocated(allocated_value))
    //         | (EdwardsGroupType::Allocated(allocated_value), EdwardsGroupType::Constant(constant_value)) =>
    //             Ok(EdwardsGroupType::Allocated(allocated_value.add_constant(cs, constant_value)?)),
    //     }
    // }
}
