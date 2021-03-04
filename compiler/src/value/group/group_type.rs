// Copyright (C) 2019-2021 Aleo Systems Inc.
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

//! A data type that represents members in the group formed by the set of affine points on a curve.

use crate::errors::GroupError;
use leo_asg::GroupValue;
use leo_asg::Span;

use snarkvm_fields::Field;
use snarkvm_models::curves::One;
use snarkvm_models::gadgets::r1cs::ConstraintSystem;
use snarkvm_models::gadgets::utilities::alloc::AllocGadget;
use snarkvm_models::gadgets::utilities::eq::ConditionalEqGadget;
use snarkvm_models::gadgets::utilities::eq::EqGadget;
use snarkvm_models::gadgets::utilities::eq::EvaluateEqGadget;
use snarkvm_models::gadgets::utilities::select::CondSelectGadget;
use snarkvm_models::gadgets::utilities::ToBitsGadget;
use snarkvm_models::gadgets::utilities::ToBytesGadget;
use std::fmt::Debug;
use std::fmt::Display;

pub trait GroupType<F: Field>:
    Sized
    + Clone
    + Debug
    + Display
    + One
    + EvaluateEqGadget<F>
    + EqGadget<F>
    + ConditionalEqGadget<F>
    + AllocGadget<GroupValue, F>
    + CondSelectGadget<F>
    + ToBitsGadget<F>
    + ToBytesGadget<F>
{
    fn constant(value: &GroupValue, span: &Span) -> Result<Self, GroupError>;

    fn to_allocated<CS: ConstraintSystem<F>>(&self, cs: CS, span: &Span) -> Result<Self, GroupError>;

    fn negate<CS: ConstraintSystem<F>>(&self, cs: CS, span: &Span) -> Result<Self, GroupError>;

    fn add<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: &Span) -> Result<Self, GroupError>;

    fn sub<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self, span: &Span) -> Result<Self, GroupError>;
}
