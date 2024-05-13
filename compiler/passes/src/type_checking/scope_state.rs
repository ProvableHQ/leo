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

use indexmap::IndexMap;
use leo_ast::{Location, Variant};
use leo_span::Symbol;

pub struct ScopeState {
    /// The name of the function that we are currently traversing.
    pub(crate) function: Option<Symbol>,
    /// The variant of the function that we are currently traversing.
    pub(crate) variant: Option<Variant>,
    /// Whether or not the function that we are currently traversing has a return statement.
    pub(crate) has_return: bool,
    /// Whether or not we are currently traversing a return statement.
    pub(crate) is_return: bool,
    /// Current program name.
    pub(crate) program_name: Option<Symbol>,
    /// Whether or not we are currently traversing a stub.
    pub(crate) is_stub: bool,
    /// The futures that must be propagated to an async function.
    pub(crate) futures: IndexMap<Symbol, Location>,
    /// Whether the finalize caller has called the finalize function.
    pub(crate) has_called_finalize: bool,
    /// Whether currently traversing a conditional statement.
    pub(crate) is_conditional: bool,
    /// Whether the current function is a call.
    pub(crate) is_call: bool,
    /// Location of most recent external call that produced a future.
    pub(crate) call_location: Option<Location>,
}

impl ScopeState {
    /// Initializes a new `ScopeState`.
    pub fn new() -> Self {
        Self {
            function: None,
            variant: None,
            has_return: false,
            is_return: false,
            program_name: None,
            is_stub: true,
            futures: IndexMap::new(),
            has_called_finalize: false,
            is_conditional: false,
            is_call: false,
            call_location: None,
        }
    }

    /// Initialize state variables for new function.
    pub fn initialize_function_state(&mut self, variant: Variant) {
        self.variant = Some(variant);
        self.has_called_finalize = false;
        self.futures = IndexMap::new();
    }

    /// Get the current location.
    pub fn location(&self) -> Location {
        Location::new(self.program_name, self.function.unwrap())
    }
}
