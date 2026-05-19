// Copyright (C) 2019-2026 Provable Inc.
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

use leo_errors::Formatted;
use leo_span::Span;
use std::fmt::Display;

const CODE_PREFIX: &str = "LUN";
const CODE_MASK: i32 = 9000;

pub(crate) fn value_out_of_i128_bounds(value: impl Display, span: Span) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 2, format!("the loop bound `{value}` does not fit into an `i128`"), span)
        .with_help("All loop bounds must fit into `i128`. Reduce the bound to a value within `i128`'s range.")
}
