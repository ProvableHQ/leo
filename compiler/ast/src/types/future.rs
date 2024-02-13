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

use crate::TupleType;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A future type consisting of the type of the inputs.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FutureType {
    // Optional type specification of inputs.
    pub inputs: Option<TupleType>
}

impl Default for FutureType {
    fn default() -> Self {
        Self { inputs: None }
    }
}
impl fmt::Display for crate::FutureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.inputs {
            Some(inputs) => write!(f, "future<{inputs}>", inputs = inputs),
            None => write!(f, "future")
        }
    }
}