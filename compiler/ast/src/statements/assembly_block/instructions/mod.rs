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

// TODO: See if we can use snarkVM instructions directly, once they are stabilized.

pub mod helpers;
pub use helpers::*;

pub mod nop;
pub use nop::*;

use crate::{impl_binary_instruction, impl_ternary_instruction, impl_unary_instruction, Node};

use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

impl_binary_instruction!(Add, "add");
impl_binary_instruction!(And, "and");
impl_binary_instruction!(Div, "div");
impl_binary_instruction!(GreaterThan, "gt");
impl_binary_instruction!(GreaterThanOrEqual, "gte");
impl_binary_instruction!(IsEqual, "eq");
impl_binary_instruction!(IsNotEqual, "neq");
impl_binary_instruction!(LessThan, "lt");
impl_binary_instruction!(LessThanOrEqual, "lte");
impl_binary_instruction!(Mul, "mul");
impl_unary_instruction!(Not, "not");
impl_binary_instruction!(Or, "or");
impl_binary_instruction!(Sub, "sub");
impl_ternary_instruction!(Ternary, "ter");

/// An Aleo instruction found in an assembly block. For example, `add a b into c;`.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Instruction {
    /// Adds `first` with `second`, storing the outcome in `destination`.
    Add(Add),
    /// Performs a bitwise `and` operation on `first` and `second`, storing the outcome in `destination`.
    And(And),
    /// Divides `first` by `second`, storing the outcome in `destination`.
    Div(Div),
    /// Computes whether `first` is greater than `second` as a boolean, storing the outcome in `destination`.
    GreaterThan(GreaterThan),
    /// Computes whether `first` is greater than or equal to `second` as a boolean, storing the outcome in `destination`.
    GreaterThanOrEqual(GreaterThanOrEqual),
    /// Computes whether `first` is equal to `second` as a boolean, storing the outcome in `destination`.
    IsEqual(IsEqual),
    /// Computes whether `first` is not equal to `second` as a boolean, storing the outcome in `destination`.
    IsNotEqual(IsNotEqual),
    /// Computes whether `first` is less than `second` as a boolean, storing the outcome in `destination`.
    LessThan(LessThan),
    /// Computes whether `first` is less than or equal to `second` as a boolean, storing the outcome in `destination`.
    LessThanOrEqual(LessThanOrEqual),
    /// Multiplies `first` by `second`, storing the outcome in `destination`.
    Mul(Mul),
    /// A NOP.
    Nop(Nop),
    /// Performs a bitwise `not` operation on `first`, storing the outcome in `destination`.
    Not(Not),
    /// Performs a bitwise `or` operation on `first` and `second`, storing the outcome in `destination`.
    Or(Or),
    /// Subtracts `second` from `first`, storing the outcome in `destination`.
    Sub(Sub),
    /// Selects `first`, if `condition` is true, otherwise selects `second`, storing the result in `destination`.
    Ternary(Ternary),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Add(x) => x.fmt(f),
            Instruction::And(x) => x.fmt(f),
            Instruction::Div(x) => x.fmt(f),
            Instruction::IsEqual(x) => x.fmt(f),
            Instruction::IsNotEqual(x) => x.fmt(f),
            Instruction::GreaterThan(x) => x.fmt(f),
            Instruction::GreaterThanOrEqual(x) => x.fmt(f),
            Instruction::LessThan(x) => x.fmt(f),
            Instruction::LessThanOrEqual(x) => x.fmt(f),
            Instruction::Mul(x) => x.fmt(f),
            Instruction::Nop(x) => x.fmt(f),
            Instruction::Not(x) => x.fmt(f),
            Instruction::Or(x) => x.fmt(f),
            Instruction::Sub(x) => x.fmt(f),
            Instruction::Ternary(x) => x.fmt(f),
        }
    }
}

impl Node for Instruction {
    fn span(&self) -> Span {
        match self {
            Instruction::Add(n) => n.span(),
            Instruction::And(n) => n.span(),
            Instruction::Div(n) => n.span(),
            Instruction::GreaterThan(n) => n.span(),
            Instruction::GreaterThanOrEqual(n) => n.span(),
            Instruction::IsEqual(n) => n.span(),
            Instruction::IsNotEqual(n) => n.span(),
            Instruction::LessThan(n) => n.span(),
            Instruction::LessThanOrEqual(n) => n.span(),
            Instruction::Mul(n) => n.span(),
            Instruction::Nop(n) => n.span(),
            Instruction::Not(n) => n.span(),
            Instruction::Or(n) => n.span(),
            Instruction::Sub(n) => n.span(),
            Instruction::Ternary(n) => n.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Instruction::Add(n) => n.set_span(span),
            Instruction::And(n) => n.set_span(span),
            Instruction::Div(n) => n.set_span(span),
            Instruction::GreaterThan(n) => n.set_span(span),
            Instruction::GreaterThanOrEqual(n) => n.set_span(span),
            Instruction::IsEqual(n) => n.set_span(span),
            Instruction::IsNotEqual(n) => n.set_span(span),
            Instruction::LessThan(n) => n.set_span(span),
            Instruction::LessThanOrEqual(n) => n.set_span(span),
            Instruction::Mul(n) => n.set_span(span),
            Instruction::Nop(n) => n.set_span(span),
            Instruction::Not(n) => n.set_span(span),
            Instruction::Or(n) => n.set_span(span),
            Instruction::Sub(n) => n.set_span(span),
            Instruction::Ternary(n) => n.set_span(span),
        }
    }
}
