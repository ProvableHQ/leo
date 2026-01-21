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

use crate::CompilerState;

use leo_ast::Function;
use leo_span::Symbol;

pub struct FunctionInliningVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// A map of reconstructed functions in the current program scope.
    pub reconstructed_functions: Vec<(Vec<Symbol>, Function)>,
    /// The main program.
    pub program: Symbol,
    /// A map to provide faster lookup of functions.
    pub function_map: indexmap::IndexMap<Vec<Symbol>, Function>,
    /// Whether or not we are currently traversing an async function block.
    pub is_async: bool,
}
