// Copyright (C) 2019-2026 Provable Inc.
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

use std::fmt::Display;

use leo_errors::Formatted;
use leo_span::Span;

use super::Value;
use crate::{BinaryOperation, UnaryOperation};

const CODE_PREFIX: &str = "CEV";
const CODE_MASK: i32 = 8000;

pub(crate) fn binary_op_failure(
    lhs: &Value,
    op: BinaryOperation,
    rhs: &Value,
    reason: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK,
        format!("Binary operation `{lhs} {op} {rhs}` failed at compile time: {reason}."),
        span,
    )
}

pub(crate) fn unary_op_failure(value: &Value, op: UnaryOperation, reason: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 1,
        format!("Unary operation `{value}.{op}()` failed at compile time: {reason}."),
        span,
    )
}

pub(crate) fn intrinsic_failure(reason: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 2,
        format!("Error during compile time evaluation of this intrinsic: {reason}."),
        span,
    )
}
