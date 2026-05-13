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

const CODE_PREFIX: &str = "NV";
const CODE_MASK: i32 = 11000;

pub(crate) fn illegal_name(
    item_name: impl Display,
    item_type: impl Display,
    keyword: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK,
        format!("`{item_name}` is an invalid {item_type} name. A {item_type} cannot be called \"{keyword}\"."),
        span,
    )
}

pub(crate) fn illegal_name_content(
    item_name: impl Display,
    item_type: impl Display,
    keyword: impl Display,
    span: Span,
) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 1,
        format!("`{item_name}` is an invalid {item_type} name. A {item_type} cannot have \"{keyword}\" in its name."),
        span,
    )
}
