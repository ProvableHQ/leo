// Copyright (C) 2019-2025 Provable Inc.
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

pub mod member;
pub use member::*;

use crate::{ConstParameter, Identifier, Indent, Mode, Node, NodeID, Type};
use leo_span::{Span, Symbol};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt;

use snarkvm::{
    console::program::{RecordType, StructType},
    prelude::{
        EntryType::{Constant, Private, Public},
        Network,
    },
};

/// A composite type definition, e.g., `struct Foo { my_field: Bar }` and `record Token { owner: address, amount: u64}`.
///
/// Type identity is decided by the full path including `identifier`,
/// as the record is nominal, not structural.
/// The fields are named so `struct Foo(u8, u16)` is not allowed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Composite {
    /// The name of the type in the type system in this module.
    pub identifier: Identifier,
    /// The composite's const parameters.
    pub const_parameters: Vec<ConstParameter>,
    /// The fields, constant variables, and functions of this composite.
    pub members: Vec<Member>,
    /// The external program the composite is defined in.
    pub external: Option<Symbol>,
    /// Was this a `record Foo { ... }`?
    /// If so, it wasn't a composite.
    pub is_record: bool,
    /// The entire span of the composite definition.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

impl PartialEq for Composite {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.external == other.external
    }
}

impl Eq for Composite {}

impl Composite {
    /// Returns the composite name as a Symbol.
    pub fn name(&self) -> Symbol {
        self.identifier.name
    }

    pub fn from_external_record<N: Network>(input: &RecordType<N>, program: Symbol) -> Self {
        Self {
            identifier: Identifier::from(input.name()),
            const_parameters: Vec::new(),
            members: [
                vec![Member {
                    mode: if input.owner().is_private() { Mode::Public } else { Mode::Private },
                    identifier: Identifier::new(Symbol::intern("owner"), Default::default()),
                    type_: Type::Address,
                    span: Default::default(),
                    id: Default::default(),
                }],
                input
                    .entries()
                    .iter()
                    .map(|(id, entry)| Member {
                        mode: if input.owner().is_public() { Mode::Public } else { Mode::Private },
                        identifier: Identifier::from(id),
                        type_: match entry {
                            Public(t) => Type::from_snarkvm(t, program),
                            Private(t) => Type::from_snarkvm(t, program),
                            Constant(t) => Type::from_snarkvm(t, program),
                        },
                        span: Default::default(),
                        id: Default::default(),
                    })
                    .collect_vec(),
            ]
            .concat(),
            external: Some(program),
            is_record: true,
            span: Default::default(),
            id: Default::default(),
        }
    }

    pub fn from_snarkvm<N: Network>(input: &StructType<N>, program: Symbol) -> Self {
        Self {
            identifier: Identifier::from(input.name()),
            const_parameters: Vec::new(),
            members: input
                .members()
                .iter()
                .map(|(id, type_)| Member {
                    mode: Mode::None,
                    identifier: Identifier::from(id),
                    type_: Type::from_snarkvm(type_, program),
                    span: Default::default(),
                    id: Default::default(),
                })
                .collect(),
            external: None,
            is_record: false,
            span: Default::default(),
            id: Default::default(),
        }
    }
}

impl fmt::Display for Composite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(if self.is_record { "record" } else { "struct" })?;
        write!(f, " {}", self.identifier)?;
        if !self.const_parameters.is_empty() {
            write!(f, "::[{}]", self.const_parameters.iter().format(", "))?;
        }
        writeln!(f, " {{")?;

        for field in self.members.iter() {
            writeln!(f, "{},", Indent(field))?;
        }
        write!(f, "}}")
    }
}

crate::simple_node_impl!(Composite);
