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
    /// CliError enum that represents all the errors for the `leo-lang` crate.
    FlattenError,
    code_mask: 3000i32,
    code_prefix: "FLA",

    /// asdf
    @formatted
    operation_overflow {
        args: (type_: impl Display, left: impl Display, op: impl Display, right: impl Display),
        msg: format!("The const operation `{left}{type_} {op} {right}{type_}` causes an overflow."),
        help: None,
    }
);
