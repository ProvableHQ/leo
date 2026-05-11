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
        format!("Finals must be run exactly once. Out of `{num_total_paths}`, there does not exist a single path in which all Finals are run exactly once."),
        span,
    )
    .with_help("Ex: for `f: Final` call `f.run()` to run a Final. Remove duplicate Final run redundancies, and add Final runs for un-run Finals.")
}

pub(crate) fn final_runs_missing(unawaited: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 1, format!("The following Finals were never run: {unawaited}"), span)
        .with_help("Ex: for `f: Final` call `f.run()` to run a Final.")
}

pub(crate) fn invalid_run_call(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 2, "Not a valid run call.", span)
        .with_help("Ex: for `f: Final` call `f.run()` to run a Final.")
}

pub(crate) fn expected_final(type_: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 3, format!("Expected a Final, but found `{type_}`"), span)
        .with_help("Only Finals can be run.")
}

pub(crate) fn entry_point_final_call_with_final_argument(function_name: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 4,
        format!("The call to {function_name} will result in failed executions on-chain."),
        span,
    )
    .with_help("There is a subtle error that occurs if an entry point fn returning Final follows a non-Final entry point fn call, and the call returns a `Final` that itself takes a `Final` as an input. See `https://github.com/AleoNet/snarkVM/issues/2570` for more context.")
}

pub(crate) fn misplaced_final(span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 5, "A Final may not be used in this way", span)
        .with_help("Finals should be created, assigned to a variable, and consumed without being moved or reassigned.")
}

pub(crate) fn compile_time_cast(value: impl Display, type_: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 8, format!("Compile time cast failure: `{value} as {type_}`."), span)
}

pub(crate) fn array_bounds(index: impl Display, len: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 10,
        format!("Array index {index} out of bounds (array length is {len})."),
        span,
    )
}

pub(crate) fn final_block_capturing_too_many_vars(size: impl Display, max: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 11,
        format!(
            "A `final` block cannot capture more than {max} variables, found one attempting to capture {size} variables."
        ),
        span,
    )
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
        format!("Not all paths through the function run all Finals. {num_unawaited_paths}/{num_total_paths} paths contain at least one Final that is never run."),
        span,
    )
    .with_help("Ex: `f.run()` to run a Final. Remove this warning by including the `--disable-conditional-branch-type-checking` flag.")
}

pub(crate) fn some_paths_contain_duplicate_final_runs(
    num_total_paths: impl Display,
    num_duplicate_await_paths: impl Display,
    span: Span,
) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 1,
        format!("Some paths through the function contain duplicate Final runs. {num_duplicate_await_paths}/{num_total_paths} paths contain at least one Final that is run more than once."),
        span,
    )
    .with_help("Look at the times `.run()` is called, and try to reduce redundancies. Remove this warning by including the `--disable-conditional-branch-type-checking` flag.")
}

pub(crate) fn final_not_awaited_in_order(future_name: impl Display, span: Span) -> Formatted {
    Formatted::warning(
        CODE_PREFIX,
        CODE_MASK + 3,
        format!("The Final `{future_name}` is not run in the order in which they were passed in to the function."),
        span,
    )
    .with_help("While it is not required for futures to be run in order, there is some specific behavior that arises, which may affect the semantics of your program. See `https://github.com/AleoNet/snarkVM/issues/2570` for more context.")
}
