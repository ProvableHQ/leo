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

use crate::{Identifier, LiteralExpression, Node};

use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// An argument to an instruction.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Operand {
    // TODO: Resolve invalid operands. Current design requires AST traversals of `Operand`s to handle the spurious `Invalid` variant.
    /// A dummy operand for invalid operands found by the parser.
    Invalid,
    Identifier(Identifier),
    Literal(LiteralExpression),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::Invalid => write!(f, "invalid"),
            Operand::Identifier(identifier) => write!(f, "{}", identifier),
            Operand::Literal(literal) => write!(f, "{}", literal),
        }
    }
}

impl Node for Operand {
    fn span(&self) -> Span {
        match self {
            Operand::Invalid => unreachable!("Invalid operands should not exist in the AST"),
            Operand::Identifier(identifier) => identifier.span(),
            Operand::Literal(literal) => literal.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Operand::Invalid => unreachable!("Invalid operands should not exist in the AST"),
            Operand::Identifier(identifier) => identifier.set_span(span),
            Operand::Literal(literal) => literal.set_span(span),
        }
    }
}
