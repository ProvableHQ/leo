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

use leo_ast::{DefinitionPlace, DefinitionStatement, Expression, Identifier, NodeID, Statement};
use leo_span::Symbol;

use std::{cell::RefCell, fmt::Display};

/// A struct used to create assignment statements.
#[derive(Debug, Default, Clone)]
pub struct Assigner {
    /// The inner counter.
    /// `RefCell` is used here to avoid `&mut` all over the compiler.
    inner: RefCell<AssignerInner>,
}

impl Assigner {
    /// Return a new unique `Symbol` from a `&str`.
    pub fn unique_symbol(&self, arg: impl Display, separator: impl Display) -> Symbol {
        self.inner.borrow_mut().unique_symbol(arg, separator)
    }

    /// Constructs the definition statement `let place = expr;`.
    /// This function should be the only place where `DefinitionStatement`s are constructed.
    pub fn simple_definition(&self, identifier: Identifier, value: Expression, id: NodeID) -> Statement {
        self.inner.borrow_mut().simple_definition(identifier, value, id)
    }
}

/// Contains the actual data for `Assigner`.
/// Modeled this way to afford an API using interior mutability.
#[derive(Debug, Default, Clone)]
pub struct AssignerInner {
    /// A strictly increasing counter, used to ensure that new variable names are unique.
    pub(crate) counter: usize,
}

impl AssignerInner {
    /// Return a new unique `Symbol` from a `&str`.
    fn unique_symbol(&mut self, arg: impl Display, separator: impl Display) -> Symbol {
        self.counter += 1;
        Symbol::intern(&format!("{}{}{}", arg, separator, self.counter - 1))
    }

    /// Constructs the definition statement `let place = expr;`.
    /// This function should be the only place where `DefinitionStatement`s are constructed.
    fn simple_definition(&mut self, identifier: Identifier, value: Expression, id: NodeID) -> Statement {
        DefinitionStatement {
            place: DefinitionPlace::Single(identifier),
            type_: None,
            value,
            span: Default::default(),
            id,
        }
        .into()
    }
}
