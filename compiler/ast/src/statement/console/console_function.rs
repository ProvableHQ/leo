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

use crate::Expression;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A console logging function to invoke.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ConsoleFunction {
    /// A `console.assert(expr)` call to invoke, asserting that the expression evaluates to true.
    Assert(Expression),
    /// A `console.assert_eq(expr1, expr2)` call to invoke, asserting that the operands are equal.
    AssertEq(Expression, Expression),
    /// A `console.assert_neq(expr1, expr2)` call to invoke, asserting that the operands are not equal.
    AssertNeq(Expression, Expression),
}

impl fmt::Display for ConsoleFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsoleFunction::Assert(expr) => write!(f, "assert({expr})"),
            ConsoleFunction::AssertEq(expr1, expr2) => write!(f, "assert_eq({expr1}, {expr2})"),
            ConsoleFunction::AssertNeq(expr1, expr2) => write!(f, "assert_neq({expr1}, {expr2})"),
        }
    }
}
