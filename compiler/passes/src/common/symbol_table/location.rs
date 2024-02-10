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

use leo_span::Symbol;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// Create custom struct to wrap (Symbol, Symbol) so that it can be serialized and deserialized.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Location {
    pub program: Symbol,
    pub name: Symbol,
}

impl Location {
    // Create new Location instance.
    pub fn new(program: Symbol, name: Symbol) -> Location {
        Location { program, name }
    }
}

impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> leo_errors::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}/{}", self.program, self.name))
    }
}

impl<'de> Deserialize<'de> for Location {
    fn deserialize<D>(deserializer: D) -> leo_errors::Result<Location, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts = s.split('/');
        let program = Symbol::intern(parts.next().unwrap());
        let name = Symbol::intern(parts.next().unwrap());
        Ok(Location::new(program, name))
    }
}
