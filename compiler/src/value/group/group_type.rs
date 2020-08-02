//! A data type that represents members in the group formed by the set of affine points on a curve.

use crate::errors::GroupError;
use leo_types::Span;

use snarkos_models::{
    curves::{Field, One},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::{
            alloc::AllocGadget,
            eq::{ConditionalEqGadget, EqGadget, EvaluateEqGadget},
            select::CondSelectGadget,
            ToBitsGadget,
            ToBytesGadget,
        },
    },
};
use std::fmt::{Debug, Display};

pub trait GroupType<F: Field>:
    Sized
    + Clone
    + Debug
    + Display
    + One
    + EvaluateEqGadget<F>
    + EqGadget<F>
    + ConditionalEqGadget<F>
    + AllocGadget<String, F>
    + CondSelectGadget<F>
    + ToBitsGadget<F>
    + ToBytesGadget<F>
{
    fn constant(string: String, span: Span) -> Result<Self, GroupError>;

    fn to_allocated<CS: ConstraintSystem<F>>(&self, cs: CS, span: Span) -> Result<Self, GroupError>;

    fn add<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: Span) -> Result<Self, GroupError>;

    fn sub<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: Span) -> Result<Self, GroupError>;
}
