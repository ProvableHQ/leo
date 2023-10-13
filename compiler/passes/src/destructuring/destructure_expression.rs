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

use crate::Destructurer;

use leo_ast::{Expression, ExpressionReconstructor, Statement, TupleAccess};

impl ExpressionReconstructor for Destructurer<'_> {
    type AdditionalOutput = Vec<Statement>;

    /// Replaces a tuple access expression with the appropriate expression.
    fn reconstruct_tuple_access(&mut self, input: TupleAccess) -> (Expression, Self::AdditionalOutput) {
        // Lookup the expression in the tuple map.
        match input.tuple.as_ref() {
            Expression::Identifier(identifier) => {
                match self.tuples.get(&identifier.name).and_then(|tuple| tuple.elements.get(input.index.value())) {
                    Some(element) => (element.clone(), Default::default()),
                    None => {
                        unreachable!("SSA guarantees that all tuples are declared and indices are valid.")
                    }
                }
            }
            _ => unreachable!("SSA guarantees that subexpressions are identifiers or literals."),
        }
    }
}
