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

use leo_errors::Formatted;
use leo_span::Span;
use std::fmt::Display;

const CODE_PREFIX: &str = "CEI";
const CODE_MASK: i32 = 13000;

// Warnings

pub(crate) fn check_after_interaction(operation: impl Display, span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK,
        format!("Check operation `{operation}` occurs after an interaction (Final.run())."),
        span,
    )
    .with_help("Move all checks (reads, asserts) before any calls to `.run()` to prevent reentrancy vulnerabilities.")
}

pub(crate) fn effect_after_interaction(operation: impl Display, span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 1,
        format!("Effect operation `{operation}` occurs after an interaction (Final.run())."),
        span,
    )
    .with_help("Move all effects (state writes) before any calls to `.run()` to prevent reentrancy vulnerabilities.")
}

pub(crate) fn callee_has_effects_after_interaction(callee: impl Display, span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 2,
        format!("Call to `{callee}` (which contains checks or effects) occurs after an interaction (Final.run())."),
        span,
    )
    .with_help("Move this call before any calls to `.run()` to prevent reentrancy vulnerabilities.")
}

pub(crate) fn cei_violation_in_loop(span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 3,
        "Loop body contains both interactions (Final.run()) and state operations (checks or effects), which violates the CEI pattern.",
        span,
    )
    .with_help("Restructure the loop so that interactions do not occur alongside checks or effects within the same iteration.")
}

pub(crate) fn tainted_value_in_finalize(variable: impl Display, external_call: impl Display, span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 4,
        format!("Value `{variable}` used in finalize block is derived from external call `{external_call}`, whose finalize may alter the state this value depends on."),
        span,
    )
    .with_help("The external program's finalize runs concurrently and may invalidate assumptions based on this value. Consider re-reading the value on-chain inside the finalize block.")
}

pub(crate) fn tainted_argument_to_external_call(
    variable: impl Display,
    callee: impl Display,
    external_call: impl Display,
    span: Span,
) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 5,
        format!("Tainted value `{variable}` (derived from external call `{external_call}`) is passed as an argument to `{callee}`, whose finalize will use the potentially stale value."),
        span,
    )
    .with_help("The external program's finalize runs concurrently and may invalidate assumptions based on this value. Consider passing the value on-chain inside the finalize block instead.")
}
