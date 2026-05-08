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

use std::fmt::Display;

use leo_errors::{Backtraced, Formatted, Label};
use leo_span::Span;

const CODE_PREFIX: &str = "AST";
const CODE_MASK: i32 = 2000;

pub(crate) fn function_not_found(func: impl Display) -> Backtraced {
    Backtraced::error(CODE_PREFIX, CODE_MASK + 16, format!("function `{func}` not found"))
}

pub(crate) fn name_defined_multiple_times(name: impl Display, span: Span, labels: Vec<Label>) -> Formatted {
    Formatted::error(CODE_PREFIX, CODE_MASK + 17, format!("The name `{name}` is defined multiple times."), span)
        .with_labels(labels)
}
