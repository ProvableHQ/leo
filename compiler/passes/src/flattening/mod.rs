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

use leo_ast::{Ast, Definitions, ProgramReconstructor};
use leo_errors::{emitter::Handler, Result};

pub mod flattener;
pub use flattener::*;

pub mod flatten_expression;
pub use flatten_expression::*;

pub mod flatten_instructions;
pub use flatten_instructions::*;

pub mod flatten_program;
pub use flatten_program::*;

pub mod flatten_statement;
pub use flatten_statement::*;

use crate::{Pass, SymbolTable};

impl<'a> Pass for Flattener<'a> {
    type Input = (Ast, &'a Handler, SymbolTable, Option<&'a Definitions>);
    type Output = Result<Ast>;

    fn do_pass((ast, handler, st, input_consts): Self::Input) -> Self::Output {
        // Reconstructs the AST based off any flattening work that is done.
        let mut reconstructor = Self::new(st, handler, input_consts);
        let program = reconstructor.reconstruct_program(ast.into_repr());
        handler.last_err()?;

        Ok(Ast::new(program))
    }
}
