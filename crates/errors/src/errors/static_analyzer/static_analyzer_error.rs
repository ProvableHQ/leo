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

use std::fmt::{Debug, Display};

// TODO: Consolidate errors.

create_messages!(
    /// StaticAnalyzer enum that represents all the errors for static analysis.
    StaticAnalyzerError,
    code_mask: 4000i32,
    code_prefix: "SAZ",

    @formatted
    no_path_runs_all_finals_exactly_once {
        args: (num_total_paths: impl Display),
        msg: format!("Finals must be run exactly once. Out of `{num_total_paths}`, there does not exist a single path in which all Finals are run exactly once."),
        help: Some("Ex: for `f: Final` call `f.run()` to run a Final. Remove duplicate Final run redundancies, and add Final runs for un-run Finals.".to_string()),
    }

    @formatted
    final_runs_missing {
        args: (unawaited: impl Display),
        msg: format!("The following Finals were never run: {unawaited}"),
        help: Some("Ex: for `f: Final` call `f.run()` to run a Final.".to_string()),
    }

    @formatted
    invalid_run_call {
        args: (),
        msg: "Not a valid run call.".to_string(),
        help: Some("Ex: for `f: Final` call `f.run()` to run a Final.".to_string()),
    }

    @formatted
    expected_final {
        args: (type_: impl Display),
        msg: format!("Expected a Final, but found `{type_}`"),
        help: Some("Only Finals can be run.".to_string()),
    }

    @formatted
    entry_point_final_call_with_final_argument {
        args: (function_name: impl Display),
        msg: format!("The call to {function_name} will result in failed executions on-chain."),
        help: Some("There is a subtle error that occurs if an entry point fn returning Final follows a non-Final entry point fn call, and the call returns a `Final` that itself takes a `Final` as an input. See `https://github.com/AleoNet/snarkVM/issues/2570` for more context.".to_string()),
    }

    @formatted
    misplaced_final {
        args: (),
        msg: "A Final may not be used in this way".to_string(),
        help: Some("Finals should be created, assigned to a variable, and consumed without being moved or reassigned.".to_string()),
    }

    @formatted
    compile_time_unary_op {
        args: (value: impl Display, op: impl Display, err: impl Display),
        msg: format!("Unary operation `{value}.{op}()` failed at compile time: {err}."),
        help: None,
    }

    @formatted
    compile_time_binary_op {
        args: (value_lhs: impl Display, value_rhs: impl Display, op: impl Display, err: impl Display),
        msg: format!("Binary operation `{value_lhs} {op} {value_rhs}` failed at compile time: {err}."),
        help: None,
    }

    @formatted
    compile_time_cast {
        args: (value: impl Display, type_: impl Display),
        msg: format!("Compile time cast failure: `{value} as {type_}`."),
        help: None,
    }

    @formatted
    compile_intrinsic {
        args: (err: impl Display),
        msg: format!("Error during compile time evaluation of this intrinsic: {err}."),
        help: None,
    }

    @formatted
    array_bounds {
        args: (index: impl Display, len: impl Display),
        msg: format!("Array index {index} out of bounds (array length is {len})."),
        help: None,
    }

    @formatted
    final_block_capturing_too_many_vars {
        args: (size: impl Display, max: impl Display),
        msg: format!("A `final` block cannot capture more than {max} variables, found one attempting to capture {size} variables."),
        help: None,
    }

    @formatted
    custom_error {
        args: (msg: impl Display, help: Option<impl Display>),
        msg: format!("{msg}"),
        help: help.map(|h| h.to_string()),
    }
);
