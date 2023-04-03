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

use std::fmt::Display;

create_messages!(
    /// ParserWarning enum that represents all the warnings for the `leo-parser` crate.
    ParserWarning,
    code_mask: 0000i32,
    code_prefix: "PAR",

    /// For when a user used const on a parameter or input instead of constant.
    @formatted
    const_parameter_or_input {
        args: (),
        msg: "`constant` is preferred over `const` for function parameters to indicate a R1CS constant.",
        help: None,
    }

    /// For when a keyword is deprecated but could be used as a valid identifier.
    @formatted
    deprecated {
        args: (keyword: impl Display, help: impl Display),
        msg: format!("The keyword `{keyword}` is deprecated."),
        help: Some(help.to_string()),
    }



);
