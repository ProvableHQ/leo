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
        "The value of this const could not be determined at compile time.",
        span,
    )
}

pub(crate) fn loop_bounds_not_evaluated(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 8, "This loop bound could not be determined at compile time.", span)
}

pub(crate) fn array_index_not_evaluated(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 9, "This array index could not be determined at compile time.", span)
}

pub(crate) fn const_prop_unroll_many_loops(bound: usize, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 10,
        format!("The const propagation and loop unrolling passes ran {bound} times without reaching a fixed point."),
        span,
    )
    .with_help("This should only happen with a pathological Leo program containing numerous nested loops or nested operations. Otherwise, this may be a bug in the Leo compiler.")
}

pub(crate) fn const_generic_not_resolved(kind: impl Display, item: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 12,
        format!(
            "Unable to resolve {kind} `{item}`. A non-const expression was provided where a const generic parameter is required."
        ),
        span,
    )
}

pub(crate) fn array_length_not_evaluated(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 13, "This array length could not be determined at compile time.", span)
}

pub(crate) fn repeat_count_not_evaluated(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 14, "This repeat count could not be determined at compile time.", span)
}

pub(crate) fn too_many_write_commands(actual: impl Display, max: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 15,
        format!("This block contains {actual} `set`/`remove` mapping commands, but the maximum allowed is {max}."),
        span,
    )
}
