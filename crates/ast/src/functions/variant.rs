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

/// The kind of a function definition.
///
/// - `Fn`: a regular function, callable from other Leo code.
/// - `FinalFn`: a `final fn`, runs in the on-chain finalize context.
/// - `EntryPoint`: a top-level program function — compiles to an Aleo
///   `transition`. May or may not have a `final {}` block.
/// - `Finalize`: the synthesized `final {}` block of an `EntryPoint`. Created
///   during compilation, not written by the user.
/// - `Query`: a read-only `query fn` (V15). Top-level program component that
///   reads finalize-store state and returns plaintext to external callers.
///   Off-consensus, no transitions, no proofs, no state writes.
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum Variant {
    #[default]
    Fn,
    FinalFn,
    EntryPoint,
    Finalize,
    Query,
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

    /// Returns true if the variant is a query function.
    pub fn is_query(self) -> bool {
        matches!(self, Variant::Query)
    }

    /// Returns true if the variant is callable from outside the program
    /// (transition entry point or query).
    pub fn is_externally_callable(self) -> bool {
        matches!(self, Variant::EntryPoint | Variant::Query)
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FinalFn => write!(f, "final fn"),
            Self::Fn => write!(f, "fn"),
            Self::EntryPoint => write!(f, "entry"),
            Self::Finalize => write!(f, "finalize"),
            Self::Query => write!(f, "query fn"),
        }
    }
}
