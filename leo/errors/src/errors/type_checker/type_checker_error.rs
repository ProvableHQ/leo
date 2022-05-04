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

    /// For when types do not match.
    @formatted
    types_do_not_match {
        args: (lhs: impl Display, rhs: impl Display),
        msg: format!(
            "unexpected type, lhs type is: '{lhs}', but rhs type is: '{rhs}'",
        ),
        help: None,
    }

    /// For when types do not match.
    @formatted
    type_expected_but_not_found {
        args: (known: impl Display),
        msg: format!(
            "One side has type: '{known}', but the other has no type",
        ),
        help: None,
    }

    /// For when the user tries to assign to a unknown variable.
    @formatted
    unknown_assignee {
        args: (var: impl Display),
        msg: format!(
            "Unknown assignee `{var}`",
        ),
        help: None,
    }

    /// For when the user tries to assign to a const input.
    @formatted
    cannont_assign_to_const_input {
        args: (input: impl Display),
        msg: format!(
            "Cannot assign to const input `{input}`",
        ),
        help: None,
    }

    /// For when the user tries to assign to a const input.
    @formatted
    cannont_assign_to_const_var {
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

    /// For when the user tries to return a unknown variable.
    @formatted
    unknown_returnee {
        args: (var: impl Display),
        msg: format!(
            "Unknown returnee `{var}`",
        ),
        help: None,
    }
);
