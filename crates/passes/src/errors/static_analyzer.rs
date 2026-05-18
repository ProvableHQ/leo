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

const CODE_PREFIX: &str = "SAZ";
const CODE_MASK: i32 = 4000;

// Errors

pub(crate) fn no_path_runs_all_finals_exactly_once(num_total_paths: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK,
        format!("no path through this function runs every `Final` exactly once (checked {num_total_paths} path(s))"),
        span,
    )
    .with_help("For a `Final` value `f`, call `f.run()` to run it. Remove duplicate `.run()` calls and add missing ones so every path consumes each `Final` exactly once.")
}

pub(crate) fn final_runs_missing(unawaited: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 1, format!("the following `Final`s were never run: {unawaited}"), span)
        .with_help("For a `Final` value `f`, call `f.run()` to run it.")
}

pub(crate) fn invalid_run_call(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 2, "not a valid `.run()` call", span)
        .with_help("For a `Final` value `f`, call `f.run()` with no arguments to run it.")
}

pub(crate) fn expected_final(type_: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 3, format!("expected a `Final`, but found `{type_}`"), span)
        .with_help("Only `Final` values can be run with `.run()`.")
}

pub(crate) fn entry_point_final_call_with_final_argument(function_name: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 4,
        format!("the call to `{function_name}` will result in failed executions on-chain"),
        span,
    )
    .with_note("There is a subtle error that occurs if an entry point fn returning `Final` follows a non-`Final` entry point fn call, and the call returns a `Final` that itself takes a `Final` as an input. See https://github.com/AleoNet/snarkVM/issues/2570 for more context.")
    .with_help("Reorder the calls so the dependency is satisfied.")
}

pub(crate) fn misplaced_final(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 5, "a `Final` cannot be used in this way", span)
        .with_help("`Final`s must be created, bound to a variable, and consumed exactly once. They cannot be moved, reassigned, or stored.")
}

pub(crate) fn compile_time_cast(value: impl Display, type_: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 8, format!("compile-time cast `{value} as {type_}` would fail"), span)
        .with_help(format!("The constant `{value}` does not fit into `{type_}`. Choose a value within the target type's range, or pick a wider target type."))
}

pub(crate) fn array_bounds(index: impl Display, len: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 10,
        format!("array index {index} is out of bounds (array length is {len})"),
        span,
    )
    .with_help(format!("Array indices are zero-based, so the valid range is `0` to `{len} - 1`."))
}

pub(crate) fn final_block_capturing_too_many_vars(size: impl Display, max: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 11,
        format!(
            "a `final` block cannot capture more than {max} variables, but this block captures {size}"
        ),
        span,
    )
    .with_help(format!("Reduce the number of values captured into the `final` block to at most {max}, e.g. by computing intermediates inside the block instead of capturing them."))
}

pub(crate) fn custom_error(msg: impl Display, help: Option<impl Display>, span: Span) -> Formatted {
    let result = Formatted::error(CODE_PREFIX, CODE_MASK + 12, format!("{msg}"), span);
    if let Some(h) = help { result.with_help(h) } else { result }
}

// Warnings

pub(crate) fn some_paths_do_not_run_all_finals(
    num_total_paths: impl Display,
    num_unawaited_paths: impl Display,
    span: Span,
) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK,
        format!("not all paths through the function run every `Final` ({num_unawaited_paths}/{num_total_paths} paths leave at least one `Final` un-run)"),
        span,
    )
    .with_help("Add `.run()` calls so every path consumes each `Final` exactly once, or pass `--disable-conditional-branch-type-checking` to silence the warning.")
}

pub(crate) fn some_paths_contain_duplicate_final_runs(
    num_total_paths: impl Display,
    num_duplicate_await_paths: impl Display,
    span: Span,
) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 1,
        format!("some paths through the function contain duplicate `Final` runs ({num_duplicate_await_paths}/{num_total_paths} paths run at least one `Final` more than once)"),
        span,
    )
    .with_help("Remove the redundant `.run()` calls, or pass `--disable-conditional-branch-type-checking` to silence the warning.")
}

pub(crate) fn final_not_awaited_in_order(future_name: impl Display, span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 3,
        format!("the `Final` `{future_name}` is not run in the order it was passed to the function"),
        span,
    )
    .with_help("Running `Final`s out of order is allowed but can change observable program semantics. See https://github.com/AleoNet/snarkVM/issues/2570 for context.")
}
