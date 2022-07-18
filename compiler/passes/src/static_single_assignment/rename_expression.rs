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

use crate::StaticSingleAssigner;

use leo_ast::{Expression, ExpressionReconstructor, Identifier};
use leo_span::Symbol;

impl<'a> ExpressionReconstructor for StaticSingleAssigner<'a> {
    type AdditionalOutput = ();

    /// Produces a new `Identifier` with a unique name.
    /// If this function is invoked on the left-hand side of a definition or assignment, a new unique name is introduced.
    /// Otherwise, we look up the previous name in the `RenameTable`.
    fn reconstruct_identifier(&mut self, identifier: Identifier) -> (Expression, Self::AdditionalOutput) {
        match self.is_lhs {
            true => {
                let new_name = Symbol::intern(&format!("{}${}", identifier.name, self.get_unique_id()));
                self.rename_table.update(identifier.name, new_name);
                (
                    Expression::Identifier(Identifier {
                        name: new_name,
                        span: identifier.span,
                    }),
                    Default::default(),
                )
            }
            false => {
                match self.rename_table.lookup(&identifier.name) {
                    // TODO: Better error.
                    None => panic!(
                        "Error: A unique name for the variable {} is not defined.",
                        identifier.name
                    ),
                    Some(name) => (
                        Expression::Identifier(Identifier {
                            name: *name,
                            span: identifier.span,
                        }),
                        Default::default(),
                    ),
                }
            }
        }
    }
}
