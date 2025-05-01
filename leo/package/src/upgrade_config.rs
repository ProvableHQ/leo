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

use serde::{Deserialize, Deserializer, Serialize, de, de::Visitor};
use std::fmt;

/// The `UpgradeConfig` defines the upgrade mechanism for a Leo program.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "lowercase")]
pub enum UpgradeConfig {
    #[default]
    Disabled,
    Admin,
    Custom,
    Checksum {
        mapping: MappingTarget,
        key: String,
    },
}

/// The `MappingTarget` defines the location at which the expected checksum is stored for the program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingTarget {
    Local(String),
    External { program_id: String, identifier: String },
}

impl<'de> Deserialize<'de> for MappingTarget {
    fn deserialize<D>(deserializer: D) -> Result<MappingTarget, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MappingTargetVisitor;

        impl Visitor<'_> for MappingTargetVisitor {
            type Value = MappingTarget;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a mapping like 'account' or 'program.aleo/expected'")
            }

            fn visit_str<E>(self, value: &str) -> Result<MappingTarget, E>
            where
                E: de::Error,
            {
                if let Some((program, ident)) = value.split_once('/') {
                    if !program.ends_with(".aleo") {
                        return Err(E::custom("program ID must end with '.aleo'"));
                    }
                    Ok(MappingTarget::External { program_id: program.to_string(), identifier: ident.to_string() })
                } else {
                    Ok(MappingTarget::Local(value.to_string()))
                }
            }
        }

        deserializer.deserialize_str(MappingTargetVisitor)
    }
}

impl Serialize for MappingTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MappingTarget::Local(s) => serializer.serialize_str(s),
            MappingTarget::External { program_id, identifier } => {
                serializer.serialize_str(&format!("{}/{}", program_id, identifier))
            }
        }
    }
}
