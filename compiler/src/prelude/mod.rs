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

pub mod blake2s;
pub use blake2s::*;

pub mod common;
pub use common::*;

pub mod scalar_types;
pub use scalar_types::*;

use crate::{ConstrainedValue, GroupType};
use leo_asg::Function;
use leo_errors::{Result, Span};

use snarkvm_fields::PrimeField;
use snarkvm_r1cs::ConstraintSystem;

pub trait CoreCircuitFuncCall<'a, F: PrimeField, G: GroupType<F>>: Send + Sync {
    fn call_function<CS: ConstraintSystem<F>>(
        &self,
        cs: &mut CS,
        function: &'a Function<'a>,
        span: &Span,
        target: Option<ConstrainedValue<'a, F, G>>,
        arguments: Vec<ConstrainedValue<'a, F, G>>,
    ) -> Result<ConstrainedValue<'a, F, G>>;
}

pub enum CoreCircuit {
    Blake2s(Blake2s),
    Address(LeoAddress),
    Bool(LeoBool),
    Char(LeoChar),
    Field(LeoField),
    Group(LeoGroup),
    I8(LeoI8),
    I16(LeoI16),
    I32(LeoI32),
    I64(LeoI64),
    I128(LeoI128),
    U8(LeoU8),
    U16(LeoU16),
    U32(LeoU32),
    U64(LeoU64),
    U128(LeoU128),
}

impl<'a, F: PrimeField, G: GroupType<F>> CoreCircuitFuncCall<'a, F, G> for CoreCircuit {
    fn call_function<CS: ConstraintSystem<F>>(
        &self,
        cs: &mut CS,
        function: &'a Function<'a>,
        span: &Span,
        target: Option<ConstrainedValue<'a, F, G>>,
        arguments: Vec<ConstrainedValue<'a, F, G>>,
    ) -> Result<ConstrainedValue<'a, F, G>> {
        match self {
            CoreCircuit::Address(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::Blake2s(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::Bool(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::Char(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::Field(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::Group(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::I8(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::I16(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::I32(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::I64(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::I128(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::U8(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::U16(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::U32(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::U64(f) => f.call_function(cs, function, span, target, arguments),
            CoreCircuit::U128(f) => f.call_function(cs, function, span, target, arguments),
        }
    }
}

pub fn resolve_core_circuit<F: PrimeField, G: GroupType<F>>(name: &str) -> CoreCircuit {
    match name {
        "address" => CoreCircuit::Address(LeoAddress),
        "blake2s" => CoreCircuit::Blake2s(Blake2s),
        "bool" => CoreCircuit::Bool(LeoBool),
        "char" => CoreCircuit::Char(LeoChar),
        "field" => CoreCircuit::Field(LeoField),
        "group" => CoreCircuit::Group(LeoGroup),
        "i8" => CoreCircuit::I8(LeoI8),
        "i16" => CoreCircuit::I16(LeoI16),
        "i32" => CoreCircuit::I32(LeoI32),
        "i64" => CoreCircuit::I64(LeoI64),
        "i128" => CoreCircuit::I128(LeoI128),
        "u8" => CoreCircuit::U8(LeoU8),
        "u16" => CoreCircuit::U16(LeoU16),
        "u32" => CoreCircuit::U32(LeoU32),
        "u64" => CoreCircuit::U64(LeoU64),
        "u128" => CoreCircuit::U128(LeoU128),
        _ => unimplemented!("invalid core circuit: {}", name),
    }
}
