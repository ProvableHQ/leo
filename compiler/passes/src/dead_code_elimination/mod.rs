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

mod eliminate_expression;

mod eliminate_statement;

mod eliminate_program;

pub mod dead_code_eliminator;
pub use dead_code_eliminator::*;

use crate::{Pass, SymbolTable};

use leo_ast::{Ast, ProgramReconstructor as _, ProgramVisitor as _};
use leo_errors::Result;

impl<'a> Pass for DeadCodeEliminator<'a> {
    type Input = (Ast,);
    /// The `bool` indicates whether the pass actually made any changes.
    type Output = Result<(Ast, bool)>;

    fn do_pass((ast,): Self::Input) -> Self::Output {
        // We make a new `SymbolTable` that will be filled in by the `VariableTracker`.
        // The SSA pass did not update the previous `SymbolTable`, so we'll need to
        // fill this one based on assignment statements as well as track usage.
        let mut symbol_table = SymbolTable::default();
        let mut tracker = VariableTracker { symbol_table: &mut symbol_table };
        tracker.visit_program(ast.as_repr());
        let mut reconstructor = DeadCodeEliminator::new(tracker.symbol_table);
        let program = reconstructor.reconstruct_program(ast.into_repr());

        Ok((Ast::new(program), reconstructor.changed))
    }
}
