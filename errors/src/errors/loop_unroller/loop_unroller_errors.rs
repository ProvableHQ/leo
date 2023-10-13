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
    /// LoopUnrollerError enum that represents all the errors for the loop unrolling errors in the `leo-loop_unroller` crate.
    LoopUnrollerError,
    code_mask: 9000i32,
    code_prefix: "LUN",

    @formatted
    loop_range_decreasing {
        args: (),
        msg: format!("The loop range must be increasing."),
        help: None,
    }

    @formatted
    variable_array_access {
        args: (),
        msg: format!("The array index must be constant."),
        help: None,
    }
);
