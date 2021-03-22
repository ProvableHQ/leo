// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{Identifier, Node, Span};

use serde::{Deserialize, Serialize};
use std::fmt;

/// The `input` keyword can view program register, record, and state values.
/// Values cannot be modified. The `input` keyword cannot be made mutable.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct InputKeyword {
    pub identifier: Identifier,
}

impl fmt::Display for InputKeyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "input")
    }
}

impl Node for InputKeyword {
    fn span(&self) -> &Span {
        &self.identifier.span
    }

    fn set_span(&mut self, span: Span) {
        self.identifier.span = span;
    }
}
