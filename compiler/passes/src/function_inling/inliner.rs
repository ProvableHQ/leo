// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::DiGraph;

use leo_ast::Function;
use leo_errors::emitter::Handler;
use leo_span::Symbol;

use indexmap::IndexMap;

pub struct Inliner<'a> {
    /// An error handler used for any errors found during unrolling.
    pub(crate) _handler: &'a Handler,
    /// The call graph of the program.
    pub(crate) call_graph: &'a DiGraph<Symbol>,
    /// The functions in the program.
    pub(crate) functions: IndexMap<Symbol, Function>,
}

impl<'a> Inliner<'a> {
    pub(crate) fn new(_handler: &'a Handler, call_graph: &'a DiGraph<Symbol>) -> Self {
        Self {
            _handler,
            call_graph,
            functions: Default::default(),
        }
    }
}
