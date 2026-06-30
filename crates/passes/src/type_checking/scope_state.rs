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
    /// Current program name.
    pub(crate) unit_name: Option<Symbol>,
    /// Current module name.
    pub(crate) module_name: Vec<Symbol>,
    /// Whether or not we are currently traversing a stub.
    pub(crate) is_stub: bool,
    /// The futures that must be propagated to an async function.
    /// We only expect futures in the top level program scope at this stage so just refer to them by their names.
    pub(crate) futures: IndexMap<Symbol, Location>,
    /// Whether the finalize caller has called the finalize function.
    pub(crate) has_called_finalize: bool,
    /// Whether this function already contains an `async` block.
    pub(crate) already_contains_an_async_block: bool,
    /// Whether we are currently traversing a conditional statement.
    pub(crate) is_conditional: bool,
    /// Location of most recent external call that produced a future.
    pub(crate) call_location: Option<Location>,
    /// Whether we are currently traversing a constructor.
    pub(crate) is_constructor: bool,
}

impl ScopeState {
    /// Initializes a new `ScopeState`.
    pub fn new() -> Self {
        Self {
            function: None,
            variant: None,
            has_return: false,
            unit_name: None,
            module_name: vec![],
            is_stub: false,
            futures: IndexMap::new(),
            has_called_finalize: false,
            already_contains_an_async_block: false,
            is_conditional: false,
            call_location: None,
            is_constructor: false,
        }
    }

    /// Resets the scope state to a valid starting state, before traversing a function or constructor.
    pub fn reset(&mut self) {
        self.function = None;
        self.variant = None;
        self.has_return = false;
        self.is_stub = false;
        self.has_called_finalize = false;
        self.is_conditional = false;
        self.call_location = None;
        self.is_constructor = false;
        self.already_contains_an_async_block = false;
        self.futures = IndexMap::new();
    }

    /// Get the current location.
    pub fn location(&self) -> Location {
        let function_path = self
            .module_name
            .iter()
            .cloned()
            .chain(std::iter::once(
                self.function.expect("Only call ScopeState::location when visiting a function or function stub."),
            ))
            .collect::<Vec<Symbol>>();

        Location::new(
            self.unit_name.expect("Only call ScopeState::location when visiting a function or function stub."),
            function_path,
        )
    }

    /// Whether the item at `target_loc`, declared with source-level visibility `target_is_pub`,
    /// is reachable from the current scope. Same-file references always pass; cross-module
    /// references require the item to not be explicitly private.
    ///
    /// The caller is expected to have looked the item up first (so existence and program-import
    /// visibility are already established). This is purely the `export` policy check.
    pub fn is_accessible(&self, target_loc: &Location, target_is_pub: Option<bool>) -> bool {
        let current_program = self.unit_name.expect("scope must be inside a compilation unit");
        if current_program == target_loc.program && self.module_name == target_loc.module_path() {
            return true;
        }
        target_is_pub != Some(false)
    }
}
