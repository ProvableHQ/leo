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

use crate::create_messages;
use std::fmt::{Debug, Display};

create_messages!(
    /// InputError enum that represents all the errors for the inputs part of `leo-ast` crate.
    TypeCheckerError,
    code_mask: 2000i32,
    code_prefix: "TYC",

    /// For when the parser encountered an invalid assignment target.
    @formatted
    invalid_assignment_target {
        args: (),
        msg: "invalid assignment target",
        help: None,
    }

    /// For when the user tries to assign to a const input.
    @formatted
    cannot_assign_to_const_input {
        args: (input: impl Display),
        msg: format!(
            "Cannot assign to const input `{input}`",
        ),
        help: None,
    }

    /// For when the user tries to assign to a const input.
    @formatted
    cannot_assign_to_const_var {
        args: (var: impl Display),
        msg: format!(
            "Cannot assign to const variable `{var}`",
        ),
        help: None,
    }

    /// For when the user tries to assign to a const input.
    @formatted
    type_should_be {
        args: (type_: impl Display, expected: impl Display),
        msg: format!(
            "Found type `{type_}` but type `{expected}` was expected",
        ),
        help: None,
    }

    /// The method name is known but not supported for the given type.
    @formatted
    type_method_not_supported {
        args: (type_: impl Display, method: impl Display),
        msg: format!(
            "Type `{type_}` does not support associated method `{method}`",
        ),
        help: None,
    }

    /// For when the user tries to return a unknown variable.
    @formatted
    unknown_sym {
        args: (kind: impl Display, sym: impl Display),
        msg: format!(
            "Unknown {kind} `{sym}`",
        ),
        help: None,
    }

    /// For when the user tries to expect a non integer type .
    @formatted
    type_should_be_integer {
        args: (op: impl Debug, type_: impl Display),
        msg: format!(
            "Binary statement has numeric operation `{op:?}` but has expected type `{type_}`",
        ),
        help: None,
    }

    /// For when the user tries to negate a non negatable type.
    @formatted
    type_is_not_negatable {
        args: (type_: impl Display),
        msg: format!(
            "The type `{type_}` is not negatable",
        ),
        help: None,
    }

    /// For when the user tries calls a function with the incorrect number of args.
    @formatted
    incorrect_num_args_to_call {
        args: (expected: impl Display, received: impl Display),
        msg: format!(
            "Call expected `{expected}` args, but got `{received}`",
        ),
        help: None,
    }

    /// For when one of the following types was expected.
    @formatted
    expected_one_type_of {
        args: (expected: impl Display, received: impl Display),
        msg: format!(
            "Expected one type from `{expected}`, but got `{received}`",
        ),
        help: None,
    }

    /// For when the base of a power is not a valid type.
    @formatted
    incorrect_pow_base_type {
        args: (type_: impl Display),
        msg: format!(
            "The first operand must be an integer or field but got type `{type_}`",
        ),
        help: None,
    }

    /// For when the exponent of a power is not a valid type.
    @formatted
    incorrect_pow_exponent_type {
        args: (allowed: impl Display, type_: impl Display),
        msg: format!(
            "The second operand must be a {allowed} but got type `{type_}`",
        ),
        help: None,
    }

    /// For when an integer is not in a valid range.
    @formatted
    invalid_int_value {
        args: (value: impl Display, type_: impl Display),
        msg: format!(
            "The value {value} is not a valid `{type_}`",
        ),
        help: None,
    }

    /// For when an invalid built in type is used.
    @formatted
    invalid_built_in_type {
        args: (type_: impl Display),
        msg: format!(
            "The type {type_} is not a valid built in type.",
        ),
        help: None,
    }

    /// For when a function doesn't have a return statement.
    @formatted
    function_has_no_return {
        args: (func: impl Display),
        msg: format!(
            "The function {func} has no return statement.",
        ),
        help: None,
    }

    /// For when a loop uses a negative value.
    @formatted
    loop_has_neg_value {
        args: (value: impl Display),
        msg: format!(
            "The loop has a negative loop bound `{value}`",
        ),
        help: None,
    }

    /// For when a loop uses a non int type as a bound.
    @formatted
    cannot_use_type_as_loop_bound {
        args: (type_: impl Display),
        msg: format!(
            "The type `{type_}` is not allowed as a loop bound.",
        ),
        help: None,
    }
);
