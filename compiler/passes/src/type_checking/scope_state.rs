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

use indexmap::{IndexMap, IndexSet};
use leo_ast::{Identifier, Variant};
use leo_span::Symbol;

pub struct ScopeState {
    /// The name of the function that we are currently traversing.
    pub(crate) function: Option<Symbol>,
    /// The variant of the function that we are currently traversing.
    pub(crate) variant: Option<Variant>,
    /// Whether or not the function that we are currently traversing has a return statement.
    pub(crate) has_return: bool,
    /// Whether or not we are currently traversing a finalize block.
    pub(crate) is_finalize: bool,
    /// Whether or not we are currently traversing a return statement.
    pub(crate) is_return: bool,
    /// Current program name.
    pub(crate) program_name: Option<Symbol>,
    /// Whether or not we are currently traversing a stub.
    pub(crate) is_stub: bool,
    /// Whether or not we are in an async transition function.
    pub(crate) is_finalize_caller: bool,
    /// The futures that must be propagated to an async function.
    pub(crate) futures: IndexSet<Identifier>,
    /// Whether the finalize caller has called the finalize function.
    pub(crate) has_finalize: bool,
    /// Whether currently traversing a conditional statement.
    pub(crate) is_conditional: bool,
    /// Finalize input types.
    pub(crate) finalize_input_types: IndexMap<(Symbol, Symbol), Identifier>,
}

impl ScopeState {
    /// Initializes a new `ScopeState`.
    pub fn new() -> Self {
        Self {
            function: None,
            variant: None,
            has_return: false,
            is_finalize: false,
            is_return: false,
            program_name: None,
            is_stub: false,
            is_finalize_caller: false,
            futures: IndexSet::new(),
            has_finalize: false,
            is_conditional: false,
            finalize_input_types: IndexMap::new(),
        }
    }

    /// Initialize state variables for new function.
    pub fn initialize_function_state(&mut self, variant: Variant, is_async: bool) {
        self.variant = Some(variant);
        self.is_finalize = variant == Variant::Standard && is_async;
        self.is_finalize_caller = variant == Variant::Transition && is_async;
        self.has_finalize = false;
        self.futures = IndexSet::new();
    }
}
