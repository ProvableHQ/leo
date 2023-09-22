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
use std::fmt::Debug;

create_messages!(
    /// AstError enum that represents all the errors for the `leo-ast` crate.
    LoopUnrollerError,
    code_mask: 8000i32,
    code_prefix: "LUN",

    @formatted
    loop_range_decreasing {
        args: (),
        msg: format!("The loop range must be increasing."),
        help: None,
    }

    @formatted
    loop_bound_must_be_a_literal {
        args: (),
        msg: format!("Loop bound must be a literal after constant propagation."),
        help: None,
    }

    @formatted
    loop_bounds_must_have_same_type_as_loop_variable {
        args: (),
        msg: format!("Loop bounds must be the same type"),
        help: None,
    }

);
