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

use crate::{Identifier, Literal, MemberAccess, Node, ProgramId};

use leo_span::Span;

use core::fmt;
use serde::{Deserialize, Serialize};

/// An operand to an AVM instruction.
// The body of `Operand` must contain all variants defined in `snarkVM/synthesizer/src/program/instruction/operand/mod.rs`.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Operand {
    /// The operand is a literal.
    Literal(Literal),
    /// The operand is a register.
    Register(Identifier),
    /// The operand is the program ID.
    ProgramID(ProgramId),
    /// The operand is the caller address.
    /// Note that parsing guarantees that this variant is always of the form `self.caller`.
    Caller(MemberAccess),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Literal(n) => n.fmt(f),
            Self::Register(n) => n.fmt(f),
            Self::ProgramID(n) => n.fmt(f),
            Self::Caller(n) => n.fmt(f),
        }
    }
}

impl Node for Operand {
    fn span(&self) -> Span {
        match self {
            Self::Literal(n) => n.span(),
            Self::Register(n) => n.span(),
            Self::ProgramID(n) => n.span(),
            Self::Caller(n) => n.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::Literal(n) => n.set_span(span),
            Self::Register(n) => n.set_span(span),
            Self::ProgramID(n) => n.set_span(span),
            Self::Caller(n) => n.set_span(span),
        }
    }
}




