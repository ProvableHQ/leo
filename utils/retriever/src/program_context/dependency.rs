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

use crate::{Location, NetworkName};
use leo_span::Symbol;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Information required to retrieve external program
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Dependency {
    name: String,
    location: Location,
    network: Option<NetworkName>,
    path: Option<PathBuf>,
}

impl Dependency {
    pub fn new(name: String, location: Location, network: Option<NetworkName>, path: Option<PathBuf>) -> Self {
        Self { name, location, network, path }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn network(&self) -> &Option<NetworkName> {
        &self.network
    }

    pub fn path(&self) -> &Option<PathBuf> {
        &self.path
    }
}

impl From<&Dependency> for Symbol {
    fn from(context: &Dependency) -> Self {
        Symbol::intern(&context.name.clone()[..context.name.len() - 5])
    }
}
