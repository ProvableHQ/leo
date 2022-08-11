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

use crate::{Pass, SymbolTable};

use leo_ast::{Ast, ProgramReconstructor};
use leo_errors::emitter::Handler;
use leo_errors::Result;

pub mod inliner;
pub use inliner::*;

impl<'a> Pass for Inliner<'a> {
    type Input = (Ast, &'a Handler, SymbolTable);
    type Output = Result<(Ast, SymbolTable)>;

    fn do_pass((ast, handler, st): Self::Input) -> Self::Output {
        let mut reconstructor = Self::new(st, handler);
        let program = reconstructor.reconstruct_program(ast.into_repr());
        handler.last_err()?;

        Ok((Ast::new(program), reconstructor.symbol_table.take()))
    }
}
