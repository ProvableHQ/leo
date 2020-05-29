use crate::errors::GroupError;
use crate::GroupType;

use snarkos_curves::edwards_bls12::{EdwardsAffine, EdwardsParameters, Fq};
use snarkos_gadgets::curves::edwards_bls12::EdwardsBlsGadget;
use snarkos_models::curves::ModelParameters;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum EdwardsGroupType {
    Constant(EdwardsAffine),
    Allocated(EdwardsBlsGadget),
}

impl GroupType<<EdwardsParameters as ModelParameters>::BaseField, Fq> for EdwardsGroupType {
    fn constant(string: String) -> Result<Self, GroupError> {
        let result =
            EdwardsAffine::from_str(&string).map_err(|_| GroupError::InvalidGroup(string))?;

        Ok(EdwardsGroupType::Constant(result))
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
