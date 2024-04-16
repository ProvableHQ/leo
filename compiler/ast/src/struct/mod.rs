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

pub mod member;
pub use member::*;

use crate::{Identifier, Mode, Node, NodeID, Type};
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
/// In some languages these are called `struct`s.
///
/// Type identity is decided by the full path including `struct_name`,
/// as the record is nominal, not structural.
/// The fields are named so `struct Foo(u8, u16)` is not allowed.
#[derive(Clone, Serialize, Deserialize)]
pub struct Composite {
    /// The name of the type in the type system in this module.
    pub identifier: Identifier,
    /// The fields, constant variables, and functions of this structure.
    pub members: Vec<Member>,
    /// The external program the struct is defined in.
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
        self.identifier == other.identifier
    }
}

impl Eq for Composite {}

impl Composite {
    /// Returns the composite name as a Symbol.
    pub fn name(&self) -> Symbol {
        self.identifier.name
    }

    pub fn from_external_record<N: Network>(input: &RecordType<N>, external_program: Symbol) -> Self {
        Self {
            identifier: Identifier::from(input.name()),
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
                            Public(t) => Type::from_snarkvm(t, None),
                            Private(t) => Type::from_snarkvm(t, None),
                            Constant(t) => Type::from_snarkvm(t, None),
                        },
                        span: Default::default(),
                        id: Default::default(),
                    })
                    .collect_vec(),
            ]
            .concat(),
            external: Some(external_program),
            is_record: true,
            span: Default::default(),
            id: Default::default(),
        }
    }

    pub fn from_snarkvm<N: Network>(input: &StructType<N>) -> Self {
        Self {
            identifier: Identifier::from(input.name()),
            members: input
                .members()
                .iter()
                .map(|(id, type_)| Member {
                    mode: Mode::None,
                    identifier: Identifier::from(id),
                    type_: Type::from_snarkvm(type_, None),
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

impl fmt::Debug for Composite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for Composite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(if self.is_record { "record" } else { "struct" })?;
        writeln!(f, " {} {{ ", self.identifier)?;
        for field in self.members.iter() {
            writeln!(f, "        {field}")?;
        }
        write!(f, "    }}")
    }
}

crate::simple_node_impl!(Composite);
