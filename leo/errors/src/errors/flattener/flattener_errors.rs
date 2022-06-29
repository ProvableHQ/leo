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

/// Generates the type name of a value.
pub fn type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

create_messages!(
    /// CliError enum that represents all the errors for the `leo-lang` crate.
    FlattenError,
    code_mask: 3000i32,
    code_prefix: "FLA",

    /// For when a constant operation would cause an overflow.
    @formatted
    binary_overflow {
        args: (left: impl Display, op: impl Display, right: impl Display, right_type: impl Display),
        msg: format!("The const operation `{left}{} {op} {right}{right_type}` causes an overflow.", type_name(&left)),
        help: None,
    }

    /// For when a constant operation would cause an overflow.
    @formatted
    unary_overflow {
        args: (left: impl Display, op: impl Display),
        msg: format!("The const operation `{left}{} {op}` causes an overflow.", type_name(&left)),
        help: None,
    }

    /// For when a loop uses a negative value.
    @formatted
    loop_has_neg_value {
        args: (value: impl Display),
        msg: format!(
            "The loop has a negative loop bound `{value}`.",
        ),
        help: None,
    }

    /// For when a loop bound goes negative or above usize::MAX
    @formatted
    incorrect_loop_bound {
        args: (pos: impl Display, bound: impl Display),
        msg: format!(
            "The loop has an incorrect `{pos}` bound of `{bound}`.",
        ),
        help: None,
    }

    /// For when a loop bound is non const.
    @formatted
    non_const_loop_bounds {
        args: (pos: impl Display),
        msg: format!(
            "The loop has an `{pos}` bound that is non_const.",
        ),
        help: None,
    }

    /// For when there is no main function.
    @backtraced
    no_main_function {
        args: (),
        msg: "The program has no main function.",
        help: None,
    }

    /// For when the main function has a mismatching type for a constant input.
    @formatted
    main_function_mismatching_const_input_type {
        args: (type_: impl Display, input_type_: impl Display),
        msg: format!(
            "The input was expected to be `{type_}` but the input file has `{input_type_}`"
        ),
        help: None,
    }

    /// For when the main function has constant variable but the input file does not.
    @formatted
    input_file_does_not_have_constant {
        args: (constant: impl Display),
        msg: format!(
            "The main function expected a constant `{constant}` but the input file does not have one."
        ),
        help: None,
    }

    /// For when the main function has constant variables but the input file has none.
    @formatted
    input_file_has_no_constants {
        args: (),
        msg: "The main function expected constant inputs but the input file does not have any.",
        help: None,
    }
);
