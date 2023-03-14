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

use crate::{Identifier, Node, Operand};

use leo_span::Span;

use core::fmt;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Call {
    pub function: Identifier,
    pub arguments: Vec<Operand>,
    pub destinations: Vec<Identifier>,
    pub span: Span,
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "call {} {} into {};",
            self.function,
            self.arguments.iter().map(|arg| arg.to_string()).join(" "),
            self.destinations.iter().map(|dst| dst.to_string()).join(" ")
        )
    }
}

crate::simple_node_impl!(Call);
