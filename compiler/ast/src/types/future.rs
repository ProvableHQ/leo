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

use crate::{Location, Type};

use serde::{Deserialize, Serialize};
use std::fmt;

/// A future type consisting of the type of the inputs.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FutureType {
    // Optional type specification of inputs.
    pub inputs: Vec<Type>,
    // The location of the function that produced the future.
    pub location: Option<Location>,
    // Whether or not the type has been explicitly specified.
    pub is_explicit: bool,
}

impl FutureType {
    /// Initialize a new future type.
    pub fn new(inputs: Vec<Type>, location: Option<Location>, is_explicit: bool) -> Self {
        Self { inputs, location, is_explicit }
    }

    /// Returns the inputs of the future type.
    pub fn inputs(&self) -> &[Type] {
        &self.inputs
    }

    /// Returns the location of the future type.
    pub fn location(&self) -> &Option<Location> {
        &self.location
    }
}

impl Default for crate::FutureType {
    fn default() -> Self {
        Self::new(vec![], None, false)
    }
}

impl fmt::Display for crate::FutureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Future<Fn({})>", self.inputs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
    }
}
