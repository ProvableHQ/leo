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

use crate::{Identifier, Node, Span, Type};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionInputVariable {
    pub identifier: Identifier,
    pub const_: bool,
    pub mutable: bool,
    pub type_: Type,
    pub span: Span,
}

impl FunctionInputVariable {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // mut var: bool
        if self.const_ {
            write!(f, "const ")?;
        }
        if self.mutable {
            write!(f, "mut ")?;
        }
        write!(f, "{}: ", self.identifier)?;
        write!(f, "{}", self.type_)
    }
}

impl fmt::Display for FunctionInputVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for FunctionInputVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl Node for FunctionInputVariable {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
