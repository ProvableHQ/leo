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
    // `operation` is a noun phrase describing what the check is (e.g. "an `assert`",
    // "a storage variable read"), so it slots directly into the sentence.
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK,
        format!("{operation} runs after an interaction (`Final.run()`)"),
        span,
    )
    .with_note("Under the Checks-Effects-Interactions pattern, all checks (reads, asserts) must precede any `.run()` calls in the same execution path.")
    .with_help("Move this check above the `.run()` call to avoid reentrancy issues.")
}

pub(crate) fn effect_after_interaction(operation: impl Display, span: Span) -> Formatted {
    // `operation` is a noun phrase describing what the effect is (e.g. "a storage
    // variable write", "a call to `Mapping::set`").
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 1,
        format!("{operation} runs after an interaction (`Final.run()`)"),
        span,
    )
    .with_note("Under the Checks-Effects-Interactions pattern, all effects (state writes) must precede any `.run()` calls in the same execution path.")
    .with_help("Move this effect above the `.run()` call to avoid reentrancy issues.")
}

pub(crate) fn callee_has_effects_after_interaction(callee: impl Display, span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 2,
        format!("the call to `{callee}` runs after an interaction (`Final.run()`), and `{callee}` itself performs checks or effects"),
        span,
    )
    .with_note("Under the Checks-Effects-Interactions pattern, any callee that performs checks or effects must run before any `.run()` calls in the same execution path.")
    .with_help(format!("Move the call to `{callee}` above the `.run()` call to avoid reentrancy issues."))
}

pub(crate) fn cei_violation_in_loop(span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 3,
        "loop body contains both interactions (`Final.run()`) and state operations (checks or effects), violating the Checks-Effects-Interactions pattern",
        span,
    )
    .with_help("Restructure the loop so a single iteration does not mix interactions with checks or effects. Split it into two passes if needed.")
}

pub(crate) fn tainted_value_in_finalize(variable: impl Display, external_call: impl Display, span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 4,
        format!("value `{variable}` used in the finalize block was derived from external call `{external_call}`, whose finalize may alter the state this value depends on"),
        span,
    )
    .with_help(format!("The external program's finalize runs concurrently and may invalidate assumptions about `{variable}`. Re-read the value on-chain inside the finalize block."))
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
        format!("tainted value `{variable}` (derived from external call `{external_call}`) is passed as an argument to `{callee}`, whose finalize will use the potentially stale value"),
        span,
    )
    .with_help(format!("The external program's finalize runs concurrently and may invalidate `{variable}`. Pass the value on-chain inside the finalize block instead."))
}
