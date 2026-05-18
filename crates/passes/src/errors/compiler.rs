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

const CODE_PREFIX: &str = "CMP";
const CODE_MASK: i32 = 6000;

pub(crate) fn const_not_evaluated(span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 7,
        "the value of this `const` could not be determined at compile time",
        span,
    )
    .with_help("Replace the initializer with an expression that only references literals and other `const` values.")
}

pub(crate) fn loop_bounds_not_evaluated(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 8, "this loop bound could not be determined at compile time", span)
        .with_help("Loop bounds must be `const` expressions. Replace the bound with a literal or a `const` value.")
}

pub(crate) fn array_index_not_evaluated(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 9, "this array index could not be determined at compile time", span)
        .with_help("Array indices must be `const` expressions. Replace the index with a literal or a `const` value.")
}

pub(crate) fn const_prop_unroll_many_loops(bound: usize, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 10,
        format!("the const-propagation and loop-unrolling passes ran {bound} times without reaching a fixed point"),
        span,
    )
    .with_help("This usually means the program has heavily nested loops or operations. Simplify the nesting if possible; if your program is not pathological, please file a Leo compiler bug.")
}

pub(crate) fn const_generic_not_resolved(kind: impl Display, item: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 12,
        format!("unable to resolve {kind} `{item}`: a non-const expression was provided where a const generic parameter is required"),
        span,
    )
    .with_help("Pass a literal or `const` value for the generic parameter so it can be resolved at compile time.")
}

pub(crate) fn array_length_not_evaluated(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 13, "this array length could not be determined at compile time", span)
        .with_help("Array lengths must be `const` expressions. Replace the length with a literal or a `const` value.")
}

pub(crate) fn repeat_count_not_evaluated(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 14, "this repeat count could not be determined at compile time", span)
        .with_help("Repeat counts must be `const` expressions. Replace the count with a literal or a `const` value.")
}

pub(crate) fn too_many_write_commands(actual: impl Display, max: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 15,
        format!("this block contains {actual} `set`/`remove` mapping commands, but the maximum allowed is {max}"),
        span,
    )
    .with_help(format!("Reduce the number of `set`/`remove` commands in this block to at most {max}, e.g. by combining writes or moving some out of the block."))
}
