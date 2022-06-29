// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use serde::{Deserialize, Serialize};
use std::fmt;

/// An operation executed by an instruction.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Opcode {
    // TODO: Resolve invalid opcodes. Current design requires AST traversals of `Opcode`s to handle the spurious `Invalid` variant.
    /// A dummy opcode for invalid opcodes found by the parser.
    Invalid,
    /// Adds `first` with `second`, storing the outcome in `destination`.
    Add,
    /// Performs a bitwise `and` operation on `first` and `second`, storing the outcome in `destination`.
    And,
    /// Divides `first` by `second`, storing the outcome in `destination`.
    Div,
    /// Computes whether `first` is greater than `second` as a boolean, storing the outcome in `destination`.
    GreaterThan,
    /// Computes whether `first` is greater than or equal to `second` as a boolean, storing the outcome in `destination`.
    GreaterThanOrEqual,
    /// Computes whether `first` equals `second` as a boolean, storing the outcome in `destination`.
    IsEqual,
    /// Computes whether `first` does **not** equals `second` as a boolean, storing the outcome in `destination`.
    IsNotEqual,
    /// Computes whether `first` is less than `second` as a boolean, storing the outcome in `destination`.
    LessThan,
    /// Computes whether `first` is less than or equal to `second` as a boolean, storing the outcome in `destination`.
    LessThanOrEqual,
    /// Multiplies `first` with `second`, storing the outcome in `destination`.
    Mul,
    /// Flips each bit in the representation of `first`, storing the outcome in `destination`.
    Not,
    /// Performs a bitwise `or` on `first` and `second`, storing the outcome in `destination`.
    Or,
    /// Computes `first - second`, storing the outcome in `destination`.
    Sub,
    /// Selects `first`, if `condition` is true, otherwise selects `second`, storing the result in `destination`.
    Ternary,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Invalid => write!(f, "invalid"),
            Opcode::Add => write!(f, "add"),
            Opcode::And => write!(f, "and"),
            Opcode::Div => write!(f, "div"),
            Opcode::GreaterThan => write!(f, "gt"),
            Opcode::GreaterThanOrEqual => write!(f, "gte"),
            Opcode::IsEqual => write!(f, "eq"),
            Opcode::IsNotEqual => write!(f, "neq"),
            Opcode::LessThan => write!(f, "lt"),
            Opcode::LessThanOrEqual => write!(f, "lte"),
            Opcode::Mul => write!(f, "mul"),
            Opcode::Not => write!(f, "not"),
            Opcode::Or => write!(f, "or"),
            Opcode::Sub => write!(f, "sub"),
            Opcode::Ternary => write!(f, "ter"),
        }
    }
}
