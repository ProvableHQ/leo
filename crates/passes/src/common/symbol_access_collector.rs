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

//! Collects all symbol accesses within an async block,
//! including both direct variable identifiers (`x`) and tuple field accesses (`x.0`, `x.1`, etc.).
//! Each access is recorded as a pair: (Symbol, Option<usize>).
//! - `None` means a direct variable access.
//! - `Some(index)` means a tuple field access.

use crate::CompilerState;

use leo_ast::{AstVisitor, Expression, Node as _, Path, ProgramVisitor, TupleAccess, Type};

use indexmap::IndexSet;

pub struct SymbolAccessCollector<'a> {
    state: &'a CompilerState,
    pub symbol_accesses: IndexSet<(Path, Option<usize>)>,
}

impl<'a> SymbolAccessCollector<'a> {
    pub fn new(state: &'a mut CompilerState) -> Self {
        Self { state, symbol_accesses: IndexSet::new() }
    }
}

impl AstVisitor for SymbolAccessCollector<'_> {
    type AdditionalInput = ();
    type Output = ();

    fn visit_path(&mut self, input: &Path, _: &Self::AdditionalInput) -> Self::Output {
        self.symbol_accesses.insert((input.clone(), None));
    }

    fn visit_tuple_access(&mut self, input: &TupleAccess, _: &Self::AdditionalInput) -> Self::Output {
        // Here we assume that we can't have nested tuples which is currently guaranteed by type
        // checking. This may change in the future.
        if let Expression::Path(path) = &input.tuple {
            // Futures aren't accessed by field; treat the whole thing as a direct variable
            if let Some(Type::Future(_)) = self.state.type_table.get(&input.tuple.id()) {
                self.symbol_accesses.insert((path.clone(), None));
            } else {
                self.symbol_accesses.insert((path.clone(), Some(input.index.value())));
            }
        } else {
            self.visit_expression(&input.tuple, &());
        }
    }
}

impl ProgramVisitor for SymbolAccessCollector<'_> {}
