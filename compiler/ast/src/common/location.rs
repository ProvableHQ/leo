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

use crate::CompositeType;
use leo_span::Symbol;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// Create custom struct to wrap (Symbol, Symbol) so that it can be serialized and deserialized.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Location {
    pub program: Option<Symbol>,
    pub name: Symbol,
}

impl Location {
    // Create new Location instance.
    pub fn new(program: Option<Symbol>, name: Symbol) -> Location {
        Location { program, name }
    }
}

impl From<&CompositeType> for Location {
    fn from(composite: &CompositeType) -> Location {
        Location::new(composite.program, composite.id.name)
    }
}

impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> leo_errors::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let condensed_str = match self.program {
            Some(program) => format!("{}/{}", program, self.name),
            None => format!("{}", self.name),
        };
        serializer.serialize_str(&condensed_str)
    }
}

impl<'de> Deserialize<'de> for Location {
    fn deserialize<D>(deserializer: D) -> leo_errors::Result<Location, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts: Vec<&str> = s.split('/').collect();
        let program = if parts.len() == 1 { None } else { Some(Symbol::intern(parts.remove(0))) };
        let name = Symbol::intern(parts.first().unwrap());
        Ok(Location::new(program, name))
    }
}
