// Copyright (C) 2019-2024 Aleo Systems Inc.
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

mod future_checker;

mod await_checker;

pub mod analyze_expression;

pub mod analyze_program;

pub mod analyze_statement;

pub mod analyzer;
pub use analyzer::*;

use crate::{Pass, SymbolTable, TypeTable};

use leo_ast::{Ast, ProgramVisitor};
use leo_errors::{Result, emitter::Handler};

use snarkvm::prelude::Network;

impl<'a, N: Network> Pass for StaticAnalyzer<'a, N> {
    type Input = (&'a Ast, &'a Handler, &'a SymbolTable, &'a TypeTable, usize, bool);
    type Output = Result<()>;

    fn do_pass((ast, handler, st, tt, max_depth, await_checking): Self::Input) -> Self::Output {
        let mut visitor = StaticAnalyzer::<N>::new(st, tt, handler, max_depth, await_checking);
        visitor.visit_program(ast.as_repr());
        handler.last_err().map_err(|e| *e)
    }
}
