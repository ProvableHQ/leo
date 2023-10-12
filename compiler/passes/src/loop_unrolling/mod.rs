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

mod range_iterator;
pub(crate) use range_iterator::*;

pub mod unroller;
pub use unroller::*;

pub mod unroll_expression;
pub use unroll_expression::*;

pub mod unroll_program;
pub use unroll_program::*;

pub mod unroll_statement;
pub use unroll_statement::*;

use crate::{Pass, SymbolTable, TypeTable};

use leo_ast::{Ast, NodeBuilder, ProgramReconstructor};
use leo_errors::{emitter::Handler, Result};

impl<'a> Pass for Unroller<'a> {
    type Input = (Ast, &'a Handler, &'a NodeBuilder, SymbolTable, &'a TypeTable);
    type Output = Result<(Ast, SymbolTable)>;

    fn do_pass((ast, handler, node_builder, st, tt): Self::Input) -> Self::Output {
        let mut reconstructor = Self::new(st, tt, handler, node_builder);
        let program = reconstructor.reconstruct_program(ast.into_repr());
        handler.last_err().map_err(|e| *e)?;

        Ok((Ast::new(program), reconstructor.symbol_table.take()))
    }
}
