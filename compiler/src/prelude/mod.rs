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

pub mod bits;
pub use bits::*;

pub mod blake2s;
pub use blake2s::*;

pub mod bytes;
pub use bytes::*;

use crate::{ConstrainedValue, GroupType};
use leo_asg::Function;
use leo_errors::{Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

pub trait CoreCircuit<'a, F: PrimeField, G: GroupType<F>>: Send + Sync {
    fn call_function<CS: ConstraintSystem<F>>(
        &self,
        cs: &mut CS,
        function: &'a Function<'a>,
        span: &Span,
        target: Option<ConstrainedValue<'a, F, G>>,
        arguments: Vec<ConstrainedValue<'a, F, G>>,
    ) -> Result<ConstrainedValue<'a, F, G>>;
}

pub fn resolve_core_circuit<'a, F: PrimeField, G: GroupType<F>>(name: &str) -> impl CoreCircuit<'a, F, G> {
    match name {
        "blake2s" => Blake2s,
        _ => unimplemented!("invalid core circuit: {}", name),
    }
}

pub trait CoreFunctionCall<'a, F: PrimeField, G: GroupType<F>>: Send + Sync {
    fn call_function<CS: ConstraintSystem<F>>(
        &self,
        cs: &mut CS,
        function: &'a Function<'a>,
        span: &Span,
        target: Option<ConstrainedValue<'a, F, G>>,
        arguments: Vec<ConstrainedValue<'a, F, G>>,
    ) -> Result<ConstrainedValue<'a, F, G>>;
}

pub enum CoreFunction {
    ToBits(ToBits),
    FromBits(FromBits),
    ToBytes(ToBytes),
    FromBytes(FromBytes),
}

impl<'a, F: PrimeField, G: GroupType<F>> CoreFunctionCall<'a, F, G> for CoreFunction {
    fn call_function<CS: ConstraintSystem<F>>(
        &self,
        cs: &mut CS,
        function: &'a Function<'a>,
        span: &Span,
        target: Option<ConstrainedValue<'a, F, G>>,
        arguments: Vec<ConstrainedValue<'a, F, G>>,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        match self {
            CoreFunction::ToBits(f) => f.call_function(cs, function, span, target, arguments),
            CoreFunction::FromBits(f) => f.call_function(cs, function, span, target, arguments),
            CoreFunction::ToBytes(f) => f.call_function(cs, function, span, target, arguments),
            CoreFunction::FromBytes(f) => f.call_function(cs, function, span, target, arguments),
        }
    }
}

pub fn resolve_core_function<F: PrimeField, G: GroupType<F>>(name: &str) -> Option<CoreFunction> {
    match name {
        "to_bits" => Some(CoreFunction::ToBits(ToBits)),
        "from_bits" => Some(CoreFunction::FromBits(FromBits)),
        "to_bytes" => Some(CoreFunction::ToBytes(ToBytes)),
        "from_bytes" => Some(CoreFunction::FromBytes(FromBytes)),
        _ => None,
    }
}
