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

use core::fmt;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

/// An AVM opcode, e.g. `add`, `add.w`, `cast`.
// The body of `Opcode` must contain all instructions defined in `snarkVM/synthesizer/src/program/instruction/mod.rs`.
// The body of `Opcode` must also contain `increment` and `decrement` commands, as long as they are still defined in snarkVM.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Opcode {
    /// Compute the absolute value of `first`, checking for overflow, and storing the outcome in `destination`.
    Abs,
    /// Compute the absolute value of `first`, wrapping around at the boundary of the type, and storing the outcome in `destination`.
    AbsWrapped,
    /// Adds `first` with `second`, storing the outcome in `destination`.
    Add,
    /// Adds `first` with `second`, wrapping around at the boundary of the type, and storing the outcome in `destination`.
    AddWrapped,
    /// Performs a bitwise `and` operation on `first` and `second`, storing the outcome in `destination`.
    And,
    /// Asserts `first` and `second` are equal.
    AssertEq,
    /// Asserts `first` and `second` are **not** equal.
    AssertNeq,
    /// Calls a closure on the operands.
    Call,
    /// Casts the operands into the declared type.
    Cast,
    /// Performs a BHP commitment on inputs of 256-bit chunks.
    CommitBHP256,
    /// Performs a BHP commitment on inputs of 512-bit chunks.
    CommitBHP512,
    /// Performs a BHP commitment on inputs of 768-bit chunks.
    CommitBHP768,
    /// Performs a BHP commitment on inputs of 1024-bit chunks.
    CommitBHP1024,
    /// Performs a Pedersen commitment on up to a 64-bit input.
    CommitPED64,
    /// Performs a Pedersen commitment on up to a 128-bit input.
    CommitPED128,
    /// Decrements the value in `mapping` at `key` by `value`.
    Decrement,
    /// Divides `first` by `second`, storing the outcome in `destination`.
    Div,
    /// Divides `first` by `second`, wrapping around at the boundary of the type, and storing the outcome in `destination`.
    DivWrapped,
    /// Doubles `first`, storing the outcome in `destination`.
    Double,
    /// Computes whether `first` is greater than `second` as a boolean, storing the outcome in `destination`.
    GreaterThan,
    /// Computes whether `first` is greater than or equal to `second` as a boolean, storing the outcome in `destination`.
    GreaterThanOrEqual,
    /// Performs a BHP hash on inputs of 256-bit chunks.
    HashBHP256,
    /// Performs a BHP hash on inputs of 512-bit chunks.
    HashBHP512,
    /// Performs a BHP hash on inputs of 768-bit chunks.
    HashBHP768,
    /// Performs a BHP hash on inputs of 1024-bit chunks.
    HashBHP1024,
    /// Performs a Pedersen hash on up to a 64-bit input.
    HashPED64,
    /// Performs a Pedersen hash on up to a 128-bit input.
    HashPED128,
    /// Performs a Poseidon hash with an input rate of 2.
    HashPSD2,
    /// Performs a Poseidon hash with an input rate of 4.
    HashPSD4,
    /// Performs a Poseidon hash with an input rate of 8.
    HashPSD8,
    /// Increments the value in `mapping` at `key` by `value`.
    Increment,
    /// Computes the multiplicative inverse of `first`, storing the outcome in `destination`.
    Inv,
    /// Computes whether `first` equals `second` as a boolean, storing the outcome in `destination`.
    IsEq,
    /// Computes whether `first` does **not** equals `second` as a boolean, storing the outcome in `destination`.
    IsNeq,
    /// Computes whether `first` is less than `second` as a boolean, storing the outcome in `destination`.
    LessThan,
    /// Computes whether `first` is less than or equal to `second` as a boolean, storing the outcome in `destination`.
    LessThanOrEqual,
    /// Computes `first` mod `second`, storing the outcome in `destination`.
    Modulo,
    /// Multiplies `first` with `second`, storing the outcome in `destination`.
    Mul,
    /// Multiplies `first` with `second`, wrapping around at the boundary of the type, and storing the outcome in `destination`.
    MulWrapped,
    /// Returns `false` if `first` and `second` are true, storing the outcome in `destination`.
    Nand,
    /// Negates `first`, storing the outcome in `destination`.
    Neg,
    /// Returns `true` if neither `first` nor `second` is `true`, storing the outcome in `destination`.
    Nor,
    /// Flips each bit in the representation of `first`, storing the outcome in `destination`.
    Not,
    /// Performs a bitwise `or` on `first` and `second`, storing the outcome in `destination`.
    Or,
    /// Raises `first` to the power of `second`, storing the outcome in `destination`.
    Pow,
    /// Raises `first` to the power of `second`, wrapping around at the boundary of the type, storing the outcome in `destination`.
    PowWrapped,
    /// Divides `first` by `second`, storing the remainder in `destination`.
    Rem,
    /// Divides `first` by `second`, wrapping around at the boundary of the type, storing the remainder in `destination`.
    RemWrapped,
    /// Shifts `first` left by `second` bits, storing the outcome in `destination`.
    Shl,
    /// Shifts `first` left by `second` bits, continuing past the boundary of the type, storing the outcome in `destination`.
    ShlWrapped,
    /// Shifts `first` right by `second` bits, storing the outcome in `destination`.
    Shr,
    /// Shifts `first` right by `second` bits, continuing past the boundary of the type, storing the outcome in `destination`.
    ShrWrapped,
    /// Squares 'first', storing the outcome in `destination`.
    Square,
    /// Compute the square root of 'first', storing the outcome in `destination`.
    SquareRoot,
    /// Computes `first - second`, storing the outcome in `destination`.
    Sub,
    /// Computes `first - second`, wrapping around at the boundary of the type, and storing the outcome in `destination`.
    SubWrapped,
    /// Selects `first`, if `condition` is true, otherwise selects `second`, storing the result in `destination`.
    Ternary,
    /// Performs a bitwise `xor` on `first` and `second`, storing the outcome in `destination`.
    Xor,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Abs => write!(f, "abs"),
            Opcode::AbsWrapped => write!(f, "abs.w"),
            Opcode::Add => write!(f, "add"),
            Opcode::AddWrapped => write!(f, "add.w"),
            Opcode::And => write!(f, "and"),
            Opcode::AssertEq => write!(f, "assert.eq"),
            Opcode::AssertNeq => write!(f, "assert.neq"),
            Opcode::Call => write!(f, "call"),
            Opcode::Cast => write!(f, "cast"),
            Opcode::CommitBHP256 => write!(f, "commit.bhp256"),
            Opcode::CommitBHP512 => write!(f, "commit.bhp512"),
            Opcode::CommitBHP768 => write!(f, "commit.bhp768"),
            Opcode::CommitBHP1024 => write!(f, "commit.bhp1024"),
            Opcode::CommitPED64 => write!(f, "commit.ped64"),
            Opcode::CommitPED128 => write!(f, "commit.ped128"),
            Opcode::Decrement => write!(f, "decrement"),
            Opcode::Div => write!(f, "div"),
            Opcode::DivWrapped => write!(f, "div.w"),
            Opcode::Double => write!(f, "double"),
            Opcode::GreaterThan => write!(f, "gt"),
            Opcode::GreaterThanOrEqual => write!(f, "gte"),
            Opcode::HashBHP256 => write!(f, "hash.bhp256"),
            Opcode::HashBHP512 => write!(f, "hash.bhp512"),
            Opcode::HashBHP768 => write!(f, "hash.bhp768"),
            Opcode::HashBHP1024 => write!(f, "hash.bhp1024"),
            Opcode::HashPED64 => write!(f, "hash.ped64"),
            Opcode::HashPED128 => write!(f, "hash.ped128"),
            Opcode::HashPSD2 => write!(f, "hash.psd2"),
            Opcode::HashPSD4 => write!(f, "hash.psd4"),
            Opcode::HashPSD8 => write!(f, "hash.psd8"),
            Opcode::Increment => write!(f, "increment"),
            Opcode::Inv => write!(f, "inv"),
            Opcode::IsEq => write!(f, "eq"),
            Opcode::IsNeq => write!(f, "neq"),
            Opcode::LessThan => write!(f, "lt"),
            Opcode::LessThanOrEqual => write!(f, "lte"),
            Opcode::Modulo => write!(f, "mod"),
            Opcode::Mul => write!(f, "mul"),
            Opcode::MulWrapped => write!(f, "mul.w"),
            Opcode::Nand => write!(f, "nand"),
            Opcode::Neg => write!(f, "neg"),
            Opcode::Nor => write!(f, "nor"),
            Opcode::Not => write!(f, "not"),
            Opcode::Or => write!(f, "or"),
            Opcode::Pow => write!(f, "pow"),
            Opcode::PowWrapped => write!(f, "pow.w"),
            Opcode::Rem => write!(f, "rem"),
            Opcode::RemWrapped => write!(f, "rem.w"),
            Opcode::Shl => write!(f, "shl"),
            Opcode::ShlWrapped => write!(f, "shl.w"),
            Opcode::Shr => write!(f, "shr"),
            Opcode::ShrWrapped => write!(f, "shr.w"),
            Opcode::Square => write!(f, "square"),
            Opcode::SquareRoot => write!(f, "sqrt"),
            Opcode::Sub => write!(f, "sub"),
            Opcode::SubWrapped => write!(f, "sub.w"),
            Opcode::Ternary => write!(f, "ternary"),
            Opcode::Xor => write!(f, "xor"),
        }
    }
}
