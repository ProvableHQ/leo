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

use snarkvm_errors::gadgets::SynthesisError;
use snarkvm_models::curves::Field;
use snarkvm_models::gadgets::r1cs::ConstraintSystem;
use snarkvm_models::gadgets::utilities::boolean::Boolean;

/// Single bit binary adder with carry bit
/// https://en.wikipedia.org/wiki/Adder_(electronics)#Full_adder
/// sum = (a XOR b) XOR carry
/// carry = a AND b OR carry AND (a XOR b)
/// Returns (sum, carry)
pub trait FullAdder<'a, F: Field>
where
    Self: std::marker::Sized,
{
    fn add<CS: ConstraintSystem<F>>(
        cs: CS,
        a: &'a Self,
        b: &'a Self,
        carry: &'a Self,
    ) -> Result<(Self, Self), SynthesisError>;
}

impl<'a, F: Field> FullAdder<'a, F> for Boolean {
    fn add<CS: ConstraintSystem<F>>(
        mut cs: CS,
        a: &'a Self,
        b: &'a Self,
        carry: &'a Self,
    ) -> Result<(Self, Self), SynthesisError> {
        let a_x_b = Boolean::xor(cs.ns(|| "a XOR b"), a, b)?;
        let sum = Boolean::xor(cs.ns(|| "adder sum"), &a_x_b, carry)?;

        let c1 = Boolean::and(cs.ns(|| "a AND b"), a, b)?;
        let c2 = Boolean::and(cs.ns(|| "carry AND (a XOR b)"), carry, &a_x_b)?;
        let carry = Boolean::or(cs.ns(|| "c1 OR c2"), &c1, &c2)?;

        Ok((sum, carry))
    }
}
