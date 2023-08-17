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

use crate::{External, Mode, Node, NodeID, Type};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Output {
    Internal(FunctionOutput),
    External(External),
}

impl Output {
    pub fn type_(&self) -> Type {
        match self {
            Output::Internal(output) => output.type_.clone(),
            Output::External(output) => output.type_(),
        }
    }

    pub fn mode(&self) -> Mode {
        match self {
            Output::Internal(output) => output.mode,
            Output::External(_) => Mode::None,
        }
    }
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

    fn id(&self) -> NodeID {
        use Output::*;
        match self {
            Internal(output) => output.id(),
            External(output) => output.id(),
        }
    }

    fn set_id(&mut self, id: NodeID) {
        use Output::*;
        match self {
            Internal(output) => output.set_id(id),
            External(output) => output.set_id(id),
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
    /// The ID of the node.
    pub id: NodeID,
}

impl fmt::Display for FunctionOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.mode, self.type_)
    }
}

crate::simple_node_impl!(FunctionOutput);
