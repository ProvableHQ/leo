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
use leo_ast::{Location, NodeID, Path};

use leo_span::Symbol;

pub struct PathResolutionVisitor<'a> {
    pub state: &'a mut CompilerState,
    /// The current program.
    pub program: Symbol,
    /// The current module.
    pub module: Vec<Symbol>,
}

impl PathResolutionVisitor<'_> {
    /// Enter module scope with path `module`, execute `func`, and then return to the parent module.
    pub fn in_module_scope<T>(&mut self, module: &[Symbol], func: impl FnOnce(&mut Self) -> T) -> T {
        let parent_module = self.module.clone();
        self.module = module.to_vec();
        let result = func(self);
        self.module = parent_module;
        result
    }

    pub fn in_scope<T>(&mut self, id: NodeID, func: impl FnOnce(&mut Self) -> T) -> T {
        self.state.symbol_table.enter_scope(Some(id));
        let result = func(self);
        self.state.symbol_table.enter_parent();
        result
    }

    /// Resolve a composite-init shorthand identifier to a fully-resolved `Path`. Mirrors
    /// the type checker's shorthand lookup in `type_checking::ast::visit_composite_init`:
    /// a local binding wins over a top-level (program-scope) global. Returns `None` when
    /// neither resolution succeeds — callers leave the shorthand untouched in that case
    /// so the type checker can emit its focused error.
    pub fn resolve_shorthand(&self, identifier: leo_ast::Identifier) -> Option<Path> {
        if self.state.symbol_table.lookup_local(identifier.name).is_some() {
            return Some(Path::from(identifier).to_local());
        }
        let loc = Location::new(self.program, vec![identifier.name]);
        if self.state.symbol_table.lookup_global(self.program, &loc).is_some() {
            return Some(Path::from(identifier).to_global(loc));
        }
        None
    }
}
