// Copyright (C) 2019-2025 Provable Inc.
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

use crate::CompilerState;

use indexmap::{IndexMap, IndexSet};
use leo_ast::{CallExpression, Function, TypeReconstructor};
use leo_span::Symbol;

pub struct MonomorphizationVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The main program.
    pub program: Symbol,
    /// The current function being reconstructed.
    pub function: Symbol,
    /// A map of reconstructed functions in the current program scope.
    pub reconstructed_functions: IndexMap<Symbol, Function>,
    /// A vector of all the calls to const generic functions that have not been resolved.
    pub unresolved_calls: Vec<CallExpression>,
    /// A set of all functions that have been monomorphized at least once. This keeps track of the _original_ names of
    /// the functions not the names of the monomorphized versions.
    pub monomorphized_functions: IndexSet<Symbol>,
}

impl TypeReconstructor for MonomorphizationVisitor<'_> {}
