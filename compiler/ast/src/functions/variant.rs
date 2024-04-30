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

use serde::{Deserialize, Serialize};

/// Functions are always one of five variants.
/// A transition function is permitted the ability to manipulate records.
/// An asynchronous transition function is a transition function that calls an asynchronous function.
/// A regular function is not permitted to manipulate records.
/// An asynchronous function contains on-chain operations.
/// An inline function is directly copied at the call site.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Variant {
    Inline,
    Function,
    Transition,
    AsyncTransition,
    AsyncFunction,
}

impl Variant {
    /// Returns true if the variant is async.
    pub fn is_async(self) -> bool {
        matches!(self, Variant::AsyncFunction | Variant::AsyncTransition)
    }

    /// Returns true if the variant is a transition.
    pub fn is_transition(self) -> bool {
        matches!(self, Variant::Transition | Variant::AsyncTransition)
    }

    /// Returns true if the variant is a function.
    pub fn is_function(self) -> bool {
        matches!(self, Variant::AsyncFunction | Variant::Function)
    }

    /// Returns true if the variant is an async function.
    pub fn is_async_function(self) -> bool {
        matches!(self, Variant::AsyncFunction)
    }
}
