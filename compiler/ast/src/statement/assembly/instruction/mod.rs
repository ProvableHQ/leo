// Copyright (C) 2019-2023 Aleo Systems Inc.
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

pub mod binary;
pub use binary::*;

pub mod call;
pub use call::*;

pub mod cast;
pub use cast::*;

pub mod decrement;
pub use decrement::*;

pub mod increment;
pub use increment::*;

pub mod operand;
pub use operand::*;

pub mod register_type;
pub use register_type::*;

pub mod unary;
pub use unary::*;

pub mod ternary;
pub use ternary::*;

use crate::Node;

use core::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use leo_span::Span;

/// An AVM instruction, e.g. `add foo bar into baz;`.
// The body of `Instruction` must contain all instructions defined in `snarkVM/synthesizer/src/program/instruction/mod.rs`.
// The body of `Instruction` must also contain `increment` and `decrement` commands, as long as they are still defined in snarkVM.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Instruction {
    /// Compute the absolute value of `first`, checking for overflow, and storing the outcome in `destination`.
    Abs(Abs),
    /// Compute the absolute value of `first`, wrapping around at the boundary of the type, and storing the outcome in `destination`.
    AbsWrapped(AbsWrapped),
    /// Adds `first` with `second`, storing the outcome in `destination`.
    Add(Add),
    /// Adds `first` with `second`, wrapping around at the boundary of the type, and storing the outcome in `destination`.
    AddWrapped(AddWrapped),
    /// Performs a bitwise `and` operation on `first` and `second`, storing the outcome in `destination`.
    And(And),
    /// Asserts `first` and `second` are equal.
    AssertEq(AssertEq),
    /// Asserts `first` and `second` are **not** equal.
    AssertNeq(AssertNeq),
    /// Calls a closure on the operands.
    Call(Call),
    /// Casts the operands into the declared type.
    Cast(Cast),
    /// Performs a BHP commitment on inputs of 256-bit chunks.
    CommitBHP256(CommitBHP256),
    /// Performs a BHP commitment on inputs of 512-bit chunks.
    CommitBHP512(CommitBHP512),
    /// Performs a BHP commitment on inputs of 768-bit chunks.
    CommitBHP768(CommitBHP768),
    /// Performs a BHP commitment on inputs of 1024-bit chunks.
    CommitBHP1024(CommitBHP1024),
    /// Performs a Pedersen commitment on up to a 64-bit input.
    CommitPED64(CommitPED64),
    /// Performs a Pedersen commitment on up to a 128-bit input.
    CommitPED128(CommitPED128),
    /// Decrements the value in `mapping` at `key` by `value`.
    Decrement(Decrement),
    /// Divides `first` by `second`, storing the outcome in `destination`.
    Div(Div),
    /// Divides `first` by `second`, wrapping around at the boundary of the type, and storing the outcome in `destination`.
    DivWrapped(DivWrapped),
    /// Doubles `first`, storing the outcome in `destination`.
    Double(Double),
    /// Computes whether `first` is greater than `second` as a boolean, storing the outcome in `destination`.
    GreaterThan(GreaterThan),
    /// Computes whether `first` is greater than or equal to `second` as a boolean, storing the outcome in `destination`.
    GreaterThanOrEqual(GreaterThanOrEqual),
    /// Performs a BHP hash on inputs of 256-bit chunks.
    HashBHP256(HashBHP256),
    /// Performs a BHP hash on inputs of 512-bit chunks.
    HashBHP512(HashBHP512),
    /// Performs a BHP hash on inputs of 768-bit chunks.
    HashBHP768(HashBHP768),
    /// Performs a BHP hash on inputs of 1024-bit chunks.
    HashBHP1024(HashBHP1024),
    /// Performs a Pedersen hash on up to a 64-bit input.
    HashPED64(HashPED64),
    /// Performs a Pedersen hash on up to a 128-bit input.
    HashPED128(HashPED128),
    /// Performs a Poseidon hash with an input rate of 2.
    HashPSD2(HashPSD2),
    /// Performs a Poseidon hash with an input rate of 4.
    HashPSD4(HashPSD4),
    /// Performs a Poseidon hash with an input rate of 8.
    HashPSD8(HashPSD8),
    /// Increments the value in `mapping` at `key` by `value`.
    Increment(Increment),
    /// Computes the multiplicative inverse of `first`, storing the outcome in `destination`.
    Inv(Inv),
    /// Computes whether `first` equals `second` as a boolean, storing the outcome in `destination`.
    IsEq(IsEq),
    /// Computes whether `first` does **not** equals `second` as a boolean, storing the outcome in `destination`.
    IsNeq(IsNeq),
    /// Computes whether `first` is less than `second` as a boolean, storing the outcome in `destination`.
    LessThan(LessThan),
    /// Computes whether `first` is less than or equal to `second` as a boolean, storing the outcome in `destination`.
    LessThanOrEqual(LessThanOrEqual),
    /// Computes `first` mod `second`, storing the outcome in `destination`.
    Modulo(Modulo),
    /// Multiplies `first` with `second`, storing the outcome in `destination`.
    Mul(Mul),
    /// Multiplies `first` with `second`, wrapping around at the boundary of the type, and storing the outcome in `destination`.
    MulWrapped(MulWrapped),
    /// Returns `false` if `first` and `second` are true, storing the outcome in `destination`.
    Nand(Nand),
    /// Negates `first`, storing the outcome in `destination`.
    Neg(Neg),
    /// Returns `true` if neither `first` nor `second` is `true`, storing the outcome in `destination`.
    Nor(Nor),
    /// Flips each bit in the representation of `first`, storing the outcome in `destination`.
    Not(Not),
    /// Performs a bitwise `or` on `first` and `second`, storing the outcome in `destination`.
    Or(Or),
    /// Raises `first` to the power of `second`, storing the outcome in `destination`.
    Pow(Pow),
    /// Raises `first` to the power of `second`, wrapping around at the boundary of the type, storing the outcome in `destination`.
    PowWrapped(PowWrapped),
    /// Divides `first` by `second`, storing the remainder in `destination`.
    Rem(Rem),
    /// Divides `first` by `second`, wrapping around at the boundary of the type, storing the remainder in `destination`.
    RemWrapped(RemWrapped),
    /// Shifts `first` left by `second` bits, storing the outcome in `destination`.
    Shl(Shl),
    /// Shifts `first` left by `second` bits, continuing past the boundary of the type, storing the outcome in `destination`.
    ShlWrapped(ShlWrapped),
    /// Shifts `first` right by `second` bits, storing the outcome in `destination`.
    Shr(Shr),
    /// Shifts `first` right by `second` bits, continuing past the boundary of the type, storing the outcome in `destination`.
    ShrWrapped(ShrWrapped),
    /// Squares 'first', storing the outcome in `destination`.
    Square(Square),
    /// Compute the square root of 'first', storing the outcome in `destination`.
    SquareRoot(SquareRoot),
    /// Computes `first - second`, storing the outcome in `destination`.
    Sub(Sub),
    /// Computes `first - second`, wrapping around at the boundary of the type, and storing the outcome in `destination`.
    SubWrapped(SubWrapped),
    /// Selects `first`, if `condition` is true, otherwise selects `second`, storing the result in `destination`.
    Ternary(Ternary),
    /// Performs a bitwise `xor` on `first` and `second`, storing the outcome in `destination`.
    Xor(Xor),
}

impl Node for Instruction {
    fn span(&self) -> Span {
        match self {
            Instruction::Abs(n) => n.span(),
            Instruction::AbsWrapped(n) => n.span(),
            Instruction::Add(n) => n.span(),
            Instruction::AddWrapped(n) => n.span(),
            Instruction::And(n) => n.span(),
            Instruction::AssertEq(n) => n.span(),
            Instruction::AssertNeq(n) => n.span(),
            Instruction::Call(n) => n.span(),
            Instruction::Cast(n) => n.span(),
            Instruction::CommitBHP256(n) => n.span(),
            Instruction::CommitBHP512(n) => n.span(),
            Instruction::CommitBHP768(n) => n.span(),
            Instruction::CommitBHP1024(n) => n.span(),
            Instruction::CommitPED64(n) => n.span(),
            Instruction::CommitPED128(n) => n.span(),
            Instruction::Decrement(n) => n.span(),
            Instruction::Div(n) => n.span(),
            Instruction::DivWrapped(n) => n.span(),
            Instruction::Double(n) => n.span(),
            Instruction::GreaterThan(n) => n.span(),
            Instruction::GreaterThanOrEqual(n) => n.span(),
            Instruction::HashBHP256(n) => n.span(),
            Instruction::HashBHP512(n) => n.span(),
            Instruction::HashBHP768(n) => n.span(),
            Instruction::HashBHP1024(n) => n.span(),
            Instruction::HashPED64(n) => n.span(),
            Instruction::HashPED128(n) => n.span(),
            Instruction::HashPSD2(n) => n.span(),
            Instruction::HashPSD4(n) => n.span(),
            Instruction::HashPSD8(n) => n.span(),
            Instruction::Increment(n) => n.span(),
            Instruction::Inv(n) => n.span(),
            Instruction::IsEq(n) => n.span(),
            Instruction::IsNeq(n) => n.span(),
            Instruction::LessThan(n) => n.span(),
            Instruction::LessThanOrEqual(n) => n.span(),
            Instruction::Modulo(n) => n.span(),
            Instruction::Mul(n) => n.span(),
            Instruction::MulWrapped(n) => n.span(),
            Instruction::Nand(n) => n.span(),
            Instruction::Neg(n) => n.span(),
            Instruction::Nor(n) => n.span(),
            Instruction::Not(n) => n.span(),
            Instruction::Or(n) => n.span(),
            Instruction::Pow(n) => n.span(),
            Instruction::PowWrapped(n) => n.span(),
            Instruction::Rem(n) => n.span(),
            Instruction::RemWrapped(n) => n.span(),
            Instruction::Shl(n) => n.span(),
            Instruction::ShlWrapped(n) => n.span(),
            Instruction::Shr(n) => n.span(),
            Instruction::ShrWrapped(n) => n.span(),
            Instruction::Square(n) => n.span(),
            Instruction::SquareRoot(n) => n.span(),
            Instruction::Sub(n) => n.span(),
            Instruction::SubWrapped(n) => n.span(),
            Instruction::Ternary(n) => n.span(),
            Instruction::Xor(n) => n.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Instruction::Abs(n) => n.set_span(span),
            Instruction::AbsWrapped(n) => n.set_span(span),
            Instruction::Add(n) => n.set_span(span),
            Instruction::AddWrapped(n) => n.set_span(span),
            Instruction::And(n) => n.set_span(span),
            Instruction::AssertEq(n) => n.set_span(span),
            Instruction::AssertNeq(n) => n.set_span(span),
            Instruction::Call(n) => n.set_span(span),
            Instruction::Cast(n) => n.set_span(span),
            Instruction::CommitBHP256(n) => n.set_span(span),
            Instruction::CommitBHP512(n) => n.set_span(span),
            Instruction::CommitBHP768(n) => n.set_span(span),
            Instruction::CommitBHP1024(n) => n.set_span(span),
            Instruction::CommitPED64(n) => n.set_span(span),
            Instruction::CommitPED128(n) => n.set_span(span),
            Instruction::Decrement(n) => n.set_span(span),
            Instruction::Div(n) => n.set_span(span),
            Instruction::DivWrapped(n) => n.set_span(span),
            Instruction::Double(n) => n.set_span(span),
            Instruction::GreaterThan(n) => n.set_span(span),
            Instruction::GreaterThanOrEqual(n) => n.set_span(span),
            Instruction::HashBHP256(n) => n.set_span(span),
            Instruction::HashBHP512(n) => n.set_span(span),
            Instruction::HashBHP768(n) => n.set_span(span),
            Instruction::HashBHP1024(n) => n.set_span(span),
            Instruction::HashPED64(n) => n.set_span(span),
            Instruction::HashPED128(n) => n.set_span(span),
            Instruction::HashPSD2(n) => n.set_span(span),
            Instruction::HashPSD4(n) => n.set_span(span),
            Instruction::HashPSD8(n) => n.set_span(span),
            Instruction::Increment(n) => n.set_span(span),
            Instruction::Inv(n) => n.set_span(span),
            Instruction::IsEq(n) => n.set_span(span),
            Instruction::IsNeq(n) => n.set_span(span),
            Instruction::LessThan(n) => n.set_span(span),
            Instruction::LessThanOrEqual(n) => n.set_span(span),
            Instruction::Modulo(n) => n.set_span(span),
            Instruction::Mul(n) => n.set_span(span),
            Instruction::MulWrapped(n) => n.set_span(span),
            Instruction::Nand(n) => n.set_span(span),
            Instruction::Neg(n) => n.set_span(span),
            Instruction::Nor(n) => n.set_span(span),
            Instruction::Not(n) => n.set_span(span),
            Instruction::Or(n) => n.set_span(span),
            Instruction::Pow(n) => n.set_span(span),
            Instruction::PowWrapped(n) => n.set_span(span),
            Instruction::Rem(n) => n.set_span(span),
            Instruction::RemWrapped(n) => n.set_span(span),
            Instruction::Shl(n) => n.set_span(span),
            Instruction::ShlWrapped(n) => n.set_span(span),
            Instruction::Shr(n) => n.set_span(span),
            Instruction::ShrWrapped(n) => n.set_span(span),
            Instruction::Square(n) => n.set_span(span),
            Instruction::SquareRoot(n) => n.set_span(span),
            Instruction::Sub(n) => n.set_span(span),
            Instruction::SubWrapped(n) => n.set_span(span),
            Instruction::Ternary(n) => n.set_span(span),
            Instruction::Xor(n) => n.set_span(span),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Abs(n) => n.fmt(f),
            Instruction::AbsWrapped(n) => n.fmt(f),
            Instruction::Add(n) => n.fmt(f),
            Instruction::AddWrapped(n) => n.fmt(f),
            Instruction::And(n) => n.fmt(f),
            Instruction::AssertEq(n) => n.fmt(f),
            Instruction::AssertNeq(n) => n.fmt(f),
            Instruction::Call(n) => n.fmt(f),
            Instruction::Cast(n) => n.fmt(f),
            Instruction::CommitBHP256(n) => n.fmt(f),
            Instruction::CommitBHP512(n) => n.fmt(f),
            Instruction::CommitBHP768(n) => n.fmt(f),
            Instruction::CommitBHP1024(n) => n.fmt(f),
            Instruction::CommitPED64(n) => n.fmt(f),
            Instruction::CommitPED128(n) => n.fmt(f),
            Instruction::Decrement(n) => n.fmt(f),
            Instruction::Div(n) => n.fmt(f),
            Instruction::DivWrapped(n) => n.fmt(f),
            Instruction::Double(n) => n.fmt(f),
            Instruction::GreaterThan(n) => n.fmt(f),
            Instruction::GreaterThanOrEqual(n) => n.fmt(f),
            Instruction::HashBHP256(n) => n.fmt(f),
            Instruction::HashBHP512(n) => n.fmt(f),
            Instruction::HashBHP768(n) => n.fmt(f),
            Instruction::HashBHP1024(n) => n.fmt(f),
            Instruction::HashPED64(n) => n.fmt(f),
            Instruction::HashPED128(n) => n.fmt(f),
            Instruction::HashPSD2(n) => n.fmt(f),
            Instruction::HashPSD4(n) => n.fmt(f),
            Instruction::HashPSD8(n) => n.fmt(f),
            Instruction::Increment(n) => n.fmt(f),
            Instruction::Inv(n) => n.fmt(f),
            Instruction::IsEq(n) => n.fmt(f),
            Instruction::IsNeq(n) => n.fmt(f),
            Instruction::LessThan(n) => n.fmt(f),
            Instruction::LessThanOrEqual(n) => n.fmt(f),
            Instruction::Modulo(n) => n.fmt(f),
            Instruction::Mul(n) => n.fmt(f),
            Instruction::MulWrapped(n) => n.fmt(f),
            Instruction::Nand(n) => n.fmt(f),
            Instruction::Neg(n) => n.fmt(f),
            Instruction::Nor(n) => n.fmt(f),
            Instruction::Not(n) => n.fmt(f),
            Instruction::Or(n) => n.fmt(f),
            Instruction::Pow(n) => n.fmt(f),
            Instruction::PowWrapped(n) => n.fmt(f),
            Instruction::Rem(n) => n.fmt(f),
            Instruction::RemWrapped(n) => n.fmt(f),
            Instruction::Shl(n) => n.fmt(f),
            Instruction::ShlWrapped(n) => n.fmt(f),
            Instruction::Shr(n) => n.fmt(f),
            Instruction::ShrWrapped(n) => n.fmt(f),
            Instruction::Square(n) => n.fmt(f),
            Instruction::SquareRoot(n) => n.fmt(f),
            Instruction::Sub(n) => n.fmt(f),
            Instruction::SubWrapped(n) => n.fmt(f),
            Instruction::Ternary(n) => n.fmt(f),
            Instruction::Xor(n) => n.fmt(f),
        }
    }
}
