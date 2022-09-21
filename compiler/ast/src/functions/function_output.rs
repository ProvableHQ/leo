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

use crate::{Identifier, Mode, Node, Type};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Output {
    Internal(FunctionOutput),
    External(FunctionOutputExternal),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Output::*;
        match self {
            Internal(output) => output.fmt(f),
            External(output) => output.fmt(f),
        }
    }
}

impl Node for Output {
    fn span(&self) -> Span {
        use Output::*;
        match self {
            Internal(output) => output.span(),
            External(output) => output.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        use Output::*;
        match self {
            Internal(output) => output.set_span(span),
            External(output) => output.set_span(span),
        }
    }
}

/// A function output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionOutput {
    /// The mode of the function output.
    pub mode: Mode,
    /// The type of the function output.
    pub type_: Type,
    /// The parameters span from any annotations to its type.
    pub span: Span,
}

impl fmt::Display for FunctionOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.mode, self.type_)
    }
}

crate::simple_node_impl!(FunctionOutput);

/// A function output from an external program with type record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionOutputExternal {
    /// The name of the external program.
    pub external: Identifier,
    /// The name of the external record type.
    pub record: Identifier,
    /// The parameters span from any annotations to its type.
    pub span: Span,
}

impl FunctionOutputExternal {
    pub fn get_type(&self) -> Type {
        Type::Identifier(self.record.clone())
    }
}

impl fmt::Display for FunctionOutputExternal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.leo/{}.record", self.external, self.record)
    }
}

crate::simple_node_impl!(FunctionOutputExternal);
