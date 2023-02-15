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

use leo_ast::{Expression, ExpressionReconstructor, Identifier, ProgramReconstructor, StatementReconstructor};

/// A `Replacer` applies `replacer` to all `Identifier`s in an AST.
/// `Replacer`s are used to rename identifiers.
/// `Replacer`s are used to interpolate function arguments.
pub struct Replacer<F>
where
    F: Fn(&Identifier) -> Expression,
{
    replace: F,
}

impl<F> Replacer<F>
where
    F: Fn(&Identifier) -> Expression,
{
    pub fn new(replace: F) -> Self {
        Self { replace }
    }
}

impl<F> ExpressionReconstructor for Replacer<F>
where
    F: Fn(&Identifier) -> Expression,
{
    type AdditionalOutput = ();

    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Self::AdditionalOutput) {
        ((self.replace)(&input), Default::default())
    }
}

impl<F> StatementReconstructor for Replacer<F> where F: Fn(&Identifier) -> Expression {}

impl<F> ProgramReconstructor for Replacer<F> where F: Fn(&Identifier) -> Expression {}
