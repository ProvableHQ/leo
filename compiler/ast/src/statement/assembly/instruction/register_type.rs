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

use crate::{Identifier, Literal, MemberAccess, Node, ProgramId, Type};

use leo_span::Span;

use core::fmt;
use serde::{Deserialize, Serialize};

/// A register type in the AVM.
// The body of `RegisterType` must contain all variants defined in `snarkVM/console/program/src/data_types/register_type/mod.rs`.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum RegisterType {
    /// The operand is an external record.
    ExternalRecord(ExternalRecordType),
    /// The operand is a register.
    Record(RecordType),
    /// The operand is the program ID.
    PlaintextType(PlaintextType),
}

impl fmt::Display for RegisterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ExternalRecord(n) => n.fmt(f),
            Self::Record(n) => n.fmt(f),
            Self::PlaintextType(n) => n.fmt(f),
        }
    }
}

impl Node for RegisterType {
    fn span(&self) -> Span {
        match self {
            Self::ExternalRecord(n) => n.span(),
            Self::Record(n) => n.span(),
            Self::PlaintextType(n) => n.span(),
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            Self::ExternalRecord(n) => n.set_span(span),
            Self::Record(n) => n.set_span(span),
            Self::PlaintextType(n) => n.set_span(span),
        }
    }
}

/// An external record type.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ExternalRecordType {
    pub program_id: ProgramId,
    pub record_type: RecordType,
    pub span: Span,
}

impl fmt::Display for ExternalRecordType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.program_id, self.record_type)
    }
}

crate::simple_node_impl!(ExternalRecordType);

/// A record type.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct RecordType {
    pub name: Identifier,
    pub span: Span,
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.record", self.name)
    }
}

crate::simple_node_impl!(RecordType);

/// A plaintext type.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PlaintextType {
    pub type_: Type,
    pub span: Span,
}

impl fmt::Display for PlaintextType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.type_)
    }
}

crate::simple_node_impl!(PlaintextType);






