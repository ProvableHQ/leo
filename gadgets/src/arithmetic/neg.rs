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

use crate::bits::RippleCarryAdder;

use snarkvm_errors::gadgets::SynthesisError;
use snarkvm_models::{
    curves::Field,
    gadgets::{r1cs::ConstraintSystem, utilities::boolean::Boolean},
};

use std::iter;

/// Returns a negated representation of `self` in the constraint system.
pub trait Neg<F: Field>
where
    Self: std::marker::Sized,
{
    type ErrorType;

    fn neg<CS: ConstraintSystem<F>>(&self, cs: CS) -> Result<Self, Self::ErrorType>;
}

impl<F: Field> Neg<F> for Vec<Boolean> {
    type ErrorType = SynthesisError;

    fn neg<CS: ConstraintSystem<F>>(&self, mut cs: CS) -> Result<Self, SynthesisError> {
        // flip all bits
        let flipped: Self = self.iter().map(|bit| bit.not()).collect();

        // add one
        let mut one = Vec::with_capacity(self.len());
        one.push(Boolean::constant(true));
        one.extend(iter::repeat(Boolean::Constant(false)).take(self.len() - 1));

        let mut bits = flipped.add_bits(cs.ns(|| "add one"), &one)?;
        let _carry = bits.pop(); // we already accounted for overflow above

        Ok(bits)
    }
}
