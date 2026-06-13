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
        format!("`{item_name}` is not a valid {item_type} name: `{keyword}` is a reserved keyword"),
        span,
    )
    .with_help(format!("Rename the {item_type} to something other than `{keyword}`."))
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
        format!("`{item_name}` is not a valid {item_type} name: it contains the reserved keyword `{keyword}`"),
        span,
    )
    .with_help(format!("Rename the {item_type} so it does not contain `{keyword}` as a substring."))
}

pub(crate) fn name_starts_with_underscore(item_name: impl Display, item_type: impl Display, span: Span) -> Formatted {
    Formatted::error(
        CODE_PREFIX,
        CODE_MASK + 2,
        format!("{item_type} `{item_name}` cannot have a name that starts with `_`"),
        span,
    )
    .with_help(format!(
        "{item_type} names are written to the Aleo bytecode and must start with a letter. Rename to avoid the leading underscore."
    ))
}
