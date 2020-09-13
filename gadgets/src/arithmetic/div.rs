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

use snarkos_errors::gadgets::SynthesisError;
use snarkos_models::{
    curves::{Field, PrimeField},
    gadgets::{
        r1cs::ConstraintSystem,
        utilities::uint::{UInt, UInt128, UInt16, UInt32, UInt64, UInt8},
    },
};

/// Returns division of `self` / `other` in the constraint system.
pub trait Div<F: Field, Rhs = Self>
where
    Self: std::marker::Sized,
{
    type ErrorType;

    #[must_use]
    fn div<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, Self::ErrorType>;
}

// Implement unsigned integers
macro_rules! div_uint_impl {
    ($($gadget: ident),*) => ($(
        impl<F: Field + PrimeField> Div<F> for $gadget {
            type ErrorType = SynthesisError;

            fn div<CS: ConstraintSystem<F>>(
                &self,
                cs: CS,
                other: &Self
            ) -> Result<Self, Self::ErrorType> {
                if let (Some(self_value), Some(other_value)) = (&self.value, &other.value) {
                    if self_value.checked_div(*other_value).is_none() {
                        return Err(SynthesisError::Unsatisfiable)
                    }
                }
                <$gadget as UInt>::div(self, cs, other)
            }
        }
    )*)
}

div_uint_impl!(UInt8, UInt16, UInt32, UInt64, UInt128);
