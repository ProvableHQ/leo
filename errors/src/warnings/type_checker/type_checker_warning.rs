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

use std::fmt::Display;

create_messages!(
    /// TypeCheckerWarning enum that represents all the warnings for the `leo-parser` crate.
    TypeCheckerWarning,
    code_mask: 0000i32,
    code_prefix: "PAR",

    @formatted
    unknown_annotation {
        args: (annotation: impl Display),
        msg: format!("Unknown annotation: `{annotation}`."),
        help: Some("Use a valid annotation. The Leo compiler supports: `@inline` and `@program`".to_string()),
    }

    @formatted
    function_is_never_called {
        args: (func: impl Display),
        msg: format!("The function `{func}` is never called."),
        help: None,

    }

);
