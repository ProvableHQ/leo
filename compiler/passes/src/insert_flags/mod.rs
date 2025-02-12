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

use crate::{Pass, SymbolTable, TypeTable};

use leo_ast::{Ast, NodeBuilder, ProgramReconstructor as _};
use leo_errors::{Result, emitter::Handler};

mod flag_inserter;
pub use flag_inserter::*;

mod flag_insert_expression;

mod flag_insert_program;

mod flag_insert_statement;

impl<'a> Pass for FlagInserter<'a> {
    type Input = (Ast, &'a Handler, &'a mut SymbolTable, &'a TypeTable, &'a NodeBuilder);
    type Output = Result<Ast>;

    fn do_pass((ast, handler, symbol_table, tt, node_builder): Self::Input) -> Self::Output {
        let mut reconstructor = FlagInserter::new(symbol_table, tt, handler, node_builder);
        let program = reconstructor.reconstruct_program(ast.into_repr());
        handler.last_err().map_err(|e| *e)?;
        Ok(Ast::new(program))
    }
}
