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

pub mod check_expressions;

pub use check_expressions::*;

pub mod check_program;
pub use check_program::*;

pub mod check_statements;
pub use check_statements::*;

pub mod checker;
pub use checker::*;

use crate::{CallGraph, Pass, StructGraph, SymbolTable, TypeTable};

use leo_ast::{Ast, ProgramVisitor};
use leo_errors::{emitter::Handler, Result};

impl<'a> Pass for TypeChecker<'a> {
    type Input = (&'a Ast, &'a Handler, SymbolTable, &'a TypeTable);
    type Output = Result<(SymbolTable, StructGraph, CallGraph)>;

    fn do_pass((ast, handler, st, tt): Self::Input) -> Self::Output {
        let mut visitor = TypeChecker::new(st, tt, handler);
        visitor.visit_program(ast.as_repr());
        handler.last_err().map_err(|e| *e)?;

        Ok((visitor.symbol_table.take(), visitor.struct_graph, visitor.call_graph))
    }
}
