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

use crate::{Identifier, Node, Type};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParamMode {
    Const,
    Private,
    Public,
}

impl fmt::Display for ParamMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParamMode::*;

        match self {
            Const => write!(f, "const"),
            Private => write!(f, "private"),
            Public => write!(f, "public"),
        }
    }
}

/// A function parameter.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionInputVariable {
    /// The name the parameter is accessible as in the function's body.
    pub identifier: Identifier,
    /// The mode of the function parameter.
    mode: ParamMode,
    /// What's the parameter's type?
    type_: Type,
    /// The parameters span from any annotations to its type.
    pub span: Span,
}

impl FunctionInputVariable {
    pub fn new(identifier: Identifier, mode: ParamMode, type_: Type, span: Span) -> Self {
        Self {
            identifier,
            mode,
            type_,
            span,
        }
    }

    pub fn mode(&self) -> ParamMode {
        self.mode
    }

    pub fn type_(&self) -> Type {
        self.type_.clone()
    }
}

impl FunctionInputVariable {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.mode)?;
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
