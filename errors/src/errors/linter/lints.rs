// Copyright (C) 2019-2025 Provable Inc.
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
    #[derive(Hash, PartialEq, Eq)]
    Lint,
    code_mask: 11000i32,
    code_prefix: "LINT",

    @formatted
    identity_op {
        args: (name: impl Display),
        msg: format!("This operation has no effect."),
        help: Some(format!("Consider reducing it to: `{name}`.")),
    }

    @formatted
    divison_by_zero {
        args: (),
        msg: format!("Attempt to divide by zero."),
        help: None,
    }

    @formatted
    irrefutable_pattern {
        args: (),
        msg: format!("Irrefutable comparison: this expression will always yield true."),
        help: None,
    }

    @formatted
    nonminimal_expression {
        args: (kind: impl Display),
        msg: format!("{kind}: this expression can be simplified."),
        help: None,
    }

    @formatted
    useless_parens {
        args: (replacement: impl Display),
        msg: format!("Unnecessary parentheses around the expression."),
        help: Some(format!("Consider replacing the expression with '{replacement}'.")),
    }

    @formatted
    useless_braces {
        args: (),
        msg: format!("Unnecessary braces around the statements."),
        help: Some("Consider removing the extra braces.".to_string()),
    }

    @formatted
    empty_braces {
        args: (),
        msg: format!("Empty block statement."),
        help: Some("Consider removing the block or adding statements to it.".to_string()),
    }

    @formatted
    unused_variable {
        args: (var: impl Display),
        msg: format!("unused variable `{var}`"),
        help: Some(format!("if this is intentional, consider prefixing it with an underscore: `_{var}`.")),
    }

    @formatted
    unused_assignments {
        args: (var: impl Display),
        msg: format!("value assigned to `{var}` is never read."),
        help: Some("maybe it is overwritten before being read?.".to_string()),
    }

    @formatted
    duplicate_import {
        args: (import: impl Display),
        msg: format!("the import `{import}` is defined multiple times"),
        help: None,
    }

    @formatted
    zero_prefixed_literal {
        args: (literal: impl Display),
        msg: "literal has leading zeroes",
        help: Some(format!("consider removing the leading zeroes to imporve coding practices\n\n-\t{}\n+\t{}", literal, literal.to_string().trim_start_matches("0"))),
    }
);
