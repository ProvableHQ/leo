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

use crate::{Block, FunctionInput, Node, Type};

use leo_span::Span;

use core::fmt;
use serde::{Deserialize, Serialize};

/// A finalize block.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Finalize {
    /// The finalize block's input parameters.
    pub input: Vec<FunctionInput>,
    /// The finalize blocks's return type.
    pub output: Type,
    /// The body of the function.
    pub block: Block,
    /// The entire span of the finalize block.
    pub span: Span,
}

impl fmt::Display for Finalize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " finalize")?;
        let parameters = self.input.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        write!(f, "({}) -> {} {}", parameters, self.output, self.block)
    }
}

crate::simple_node_impl!(Finalize);
