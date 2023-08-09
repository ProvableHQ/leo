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

use crate::{External, Identifier, Mode, Node, NodeID, Type};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Input {
    Internal(FunctionInput),
    External(External),
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Input::*;
        match self {
            Internal(input) => input.fmt(f),
            External(input) => input.fmt(f),
        }
    }
}

impl Input {
    pub fn type_(&self) -> Type {
        use Input::*;
        match self {
            Internal(input) => input.type_.clone(),
            External(input) => input.type_(),
        }
    }

    pub fn identifier(&self) -> Identifier {
        use Input::*;
        match self {
            Internal(input) => input.identifier,
            External(input) => input.identifier,
        }
    }

    pub fn mode(&self) -> Mode {
        use Input::*;
        match self {
            Internal(input) => input.mode,
            External(_) => Mode::None,
        }
    }
}

impl Node for Input {
    fn span(&self) -> Span {
        use Input::*;
        match self {
            Internal(input) => input.span(),
            External(input) => input.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        use Input::*;
        match self {
            Internal(input) => input.set_span(span),
            External(input) => input.set_span(span),
        }
    }

    fn id(&self) -> usize {
        use Input::*;
        match self {
            Internal(input) => input.id(),
            External(input) => input.id(),
        }
    }

    fn set_id(&mut self, id: usize) {
        use Input::*;
        match self {
            Internal(input) => input.set_id(id),
            External(input) => input.set_id(id),
        }
    }
}

/// A function parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionInput {
    /// The name the parameter is accessible as in the function's body.
    pub identifier: Identifier,
    /// The mode of the function parameter.
    pub mode: Mode,
    /// What's the parameter's type?
    pub type_: Type,
    /// The parameters span from any annotations to its type.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl FunctionInput {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}: {}", self.mode, self.identifier, self.type_)
    }
}

impl fmt::Display for FunctionInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

crate::simple_node_impl!(FunctionInput);
