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

use serde::{Deserialize, Serialize};

use std::fmt;

/// Functions are always one of six variants.
/// A transition function is permitted the ability to manipulate records.
/// An asynchronous transition function is a transition function that calls an asynchronous function.
/// A regular function is not permitted to manipulate records.
/// An asynchronous function contains on-chain operations.
/// An inline function is directly copied at the call site.
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum Variant {
    #[default]
    Fn,
    FinalFn,
    EntryPoint,
    Finalize,
}

impl Variant {
    /// Returns true if the variant is an entry point.
    pub fn is_entry(self) -> bool {
        matches!(self, Variant::EntryPoint)
    }

    pub fn is_finalize(self) -> bool {
        matches!(self, Variant::Finalize)
    }

    pub fn is_onchain(self) -> bool {
        matches!(self, Variant::Finalize | Variant::FinalFn)
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FinalFn => write!(f, "final fn"),
            Self::Fn => write!(f, "fn"),
            Self::EntryPoint => write!(f, "entry"),
            Self::Finalize => write!(f, "finalize"),
        }
    }
}
