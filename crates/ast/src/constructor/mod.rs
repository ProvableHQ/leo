// Copyright (C) 2019-2026 Provable Inc.
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

use crate::{Annotation, Block, Indent, IntegerType, Location, NetworkName, Node, NodeID, Type};
use leo_span::{Span, sym};

use anyhow::{anyhow, bail};
use serde::{Deserialize, Serialize};
use snarkvm::prelude::{Address, Literal, Locator, Network};
use std::{fmt, str::FromStr};

/// A constructor definition.
#[derive(Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Constructor {
    /// Annotations on the constructor.
    pub annotations: Vec<Annotation>,
    /// The body of the constructor.
    pub block: Block,
    /// The entire span of the constructor definition.
    pub span: Span,
    /// The ID of the node.
    pub id: NodeID,
}

/// The upgrade variant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UpgradeVariant {
    Admin { address: String },
    Custom,
    Checksum { mapping: Location, key: String, key_type: Type },
    NoUpgrade,
}

impl Constructor {
    pub fn get_upgrade_variant_with_network(&self, network: NetworkName) -> anyhow::Result<UpgradeVariant> {
        match network {
            NetworkName::MainnetV0 => self.get_upgrade_variant::<snarkvm::prelude::MainnetV0>(),
            NetworkName::TestnetV0 => self.get_upgrade_variant::<snarkvm::prelude::TestnetV0>(),
            NetworkName::CanaryV0 => self.get_upgrade_variant::<snarkvm::prelude::CanaryV0>(),
        }
    }

    /// Checks that the constructor's annotations are valid and returns the upgrade variant.
    pub fn get_upgrade_variant<N: Network>(&self) -> anyhow::Result<UpgradeVariant> {
        // Check that there is exactly one annotation.
        if self.annotations.len() != 1 {
            bail!(
                "A constructor must have exactly one of the following annotations: `@admin`, `@checksum`, `@custom`, or `@noupgrade`."
            );
        }
        // Get the annotation.
        let annotation = &self.annotations[0];
        match annotation.identifier.name {
            sym::admin => {
                // Parse the address string from the annotation.
                let Some(address_string) = annotation.map.get(&sym::address) else {
                    bail!("An `@admin` annotation must have an 'address' key.")
                };
                // Parse the address.
                let address = Address::<N>::from_str(address_string)
                    .map_err(|e| anyhow!("Invalid address in `@admin` annotation: `{e}`."))?;
                Ok(UpgradeVariant::Admin { address: address.to_string() })
            }
            sym::checksum => {
                // Parse the mapping string from the annotation.
                let Some(mapping_string) = annotation.map.get(&sym::mapping) else {
                    bail!("A `@checksum` annotation must have a 'mapping' key.")
                };
                // Parse the mapping string as a locator.
                let mapping = Locator::<N>::from_str(mapping_string)
                    .map_err(|e| anyhow!("Invalid mapping in `@checksum` annotation: `{e}`."))?;

                // Parse the key string from the annotation.
                let Some(key_string) = annotation.map.get(&sym::key) else {
                    bail!("A `@checksum` annotation must have a 'key' key.")
                };
                // Parse the key as a plaintext value.
                let key = Literal::<N>::from_str(key_string)
                    .map_err(|e| anyhow!("Invalid key in `@checksum` annotation: `{e}`."))?;
                // Get the literal type.
                let key_type = get_type_from_snarkvm_literal(&key);
                Ok(UpgradeVariant::Checksum { mapping: mapping.into(), key: key.to_string(), key_type })
            }
            sym::custom => Ok(UpgradeVariant::Custom),
            sym::noupgrade => Ok(UpgradeVariant::NoUpgrade),
            _ => bail!(
                "Invalid annotation on constructor: `{}`. Expected one of `@admin`, `@checksum`, `@custom`, or `@noupgrade`.",
                annotation.identifier.name
            ),
        }
    }
}

impl fmt::Display for Constructor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for annotation in &self.annotations {
            writeln!(f, "{annotation}")?;
        }

        writeln!(f, "async constructor() {{")?;
        for stmt in self.block.statements.iter() {
            writeln!(f, "{}{}", Indent(stmt), stmt.semicolon())?;
        }
        write!(f, "}}")
    }
}

impl fmt::Debug for Constructor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

crate::simple_node_impl!(Constructor);

// A helper function to get the type from a snarkVM literal.
fn get_type_from_snarkvm_literal<N: Network>(literal: &Literal<N>) -> Type {
    match literal {
        Literal::Field(_) => Type::Field,
        Literal::Group(_) => Type::Group,
        Literal::Address(_) => Type::Address,
        Literal::Scalar(_) => Type::Scalar,
        Literal::Boolean(_) => Type::Boolean,
        Literal::String(_) => Type::String,
        Literal::I8(_) => Type::Integer(IntegerType::I8),
        Literal::I16(_) => Type::Integer(IntegerType::I16),
        Literal::I32(_) => Type::Integer(IntegerType::I32),
        Literal::I64(_) => Type::Integer(IntegerType::I64),
        Literal::I128(_) => Type::Integer(IntegerType::I128),
        Literal::U8(_) => Type::Integer(IntegerType::U8),
        Literal::U16(_) => Type::Integer(IntegerType::U16),
        Literal::U32(_) => Type::Integer(IntegerType::U32),
        Literal::U64(_) => Type::Integer(IntegerType::U64),
        Literal::U128(_) => Type::Integer(IntegerType::U128),
        Literal::Signature(_) => Type::Signature,
    }
}
