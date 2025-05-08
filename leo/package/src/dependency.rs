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

use crate::{Location, NetworkName};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de, ser::SerializeStruct};
use std::path::PathBuf;

/// Information about a dependency, as represented in the `program.json` manifest.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Dependency {
    /// The name of the program. As this corresponds to what appears in `program.json`,
    /// it should have the ".aleo" suffix.
    pub name: String,
    /// Network or local dependency? Note that this isn't really used, as `network`
    /// and `path` provide us this information.
    pub location: Location,
    /// For a network dependency, which network?
    /// Note: This field has been removed from the manifest, but we keep it here for backwards compatibility.
    pub network: Option<NetworkName>,
    /// For a local dependency, where is its package?
    pub path: Option<PathBuf>,
}

impl Dependency {
    pub fn new(name: String, location: Location, network: Option<NetworkName>, path: Option<PathBuf>) -> Self {
        Self { name, location, network, path }
    }
}

impl Serialize for Dependency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Dependency", 3)?;
        state.serialize_field("name", &self.name)?;
        match self.location {
            Location::Network => {
                state.serialize_field("location", "network")?;
            }
            Location::Local => {
                state.serialize_field("location", "local")?;
                if let Some(path) = &self.path {
                    state.serialize_field("path", &path)?;
                }
            }
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for Dependency {
    fn deserialize<D>(deserializer: D) -> Result<Dependency, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawDependency {
            name: String,
            location: Option<String>,
            network: Option<String>,
            path: Option<PathBuf>,
        }

        let raw = RawDependency::deserialize(deserializer)?;

        let location = match raw.location.as_deref() {
            Some("network") => Location::Network,
            Some("local") => Location::Local,
            None => {
                // Infer location based on fields (for backward compatibility)
                if raw.path.is_some() { Location::Local } else { Location::Network }
            }
            Some(other) => return Err(de::Error::unknown_variant(other, &["network", "local"])),
        };

        let network = match raw.network.as_deref() {
            Some("testnet") => Some(NetworkName::TestnetV0),
            Some("mainnet") => Some(NetworkName::MainnetV0),
            Some("canary") => Some(NetworkName::CanaryV0),
            None => None,
            Some(other) => return Err(de::Error::unknown_variant(other, &["testnet", "mainnet", "canary"])),
        };

        Ok(Dependency { name: raw.name, location, network, path: raw.path })
    }
}
