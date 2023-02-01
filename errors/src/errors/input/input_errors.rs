// Copyright (C) 2019-2023 Aleo Systems Inc.
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
    InputError,
    code_mask: 1000i32,
    code_prefix: "INP",

    /// For when declared variable type mismatches actual type.
    @formatted
    unexpected_type {
        args: (expected: impl Display, received: impl Display),
        msg: format!(
            "unexpected type, expected: '{expected}', received: '{received}'",
        ),
        help: None,
    }

    /// For when the expression is not allowed in an input file.
    @formatted
    illegal_expression {
        args: (expr: impl Display),
        msg: format!("expression '{expr}' is not allowed in inputs"),
        help: None,
    }

    /// For when section name is not an allowed one.
    @formatted
    unexpected_section {
        args: (expected: &[impl Display], received: impl Display),
        msg: format!(
            "unexpected section: expected {} -- got '{received}'",
            expected
                .iter()
                .map(|x| format!("'{x}'"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        help: None,
    }
);
