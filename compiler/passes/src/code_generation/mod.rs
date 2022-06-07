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

pub mod generator;
pub use generator::*;

mod visit_expressions;

mod visit_program;

mod visit_statements;

mod visit_type;

use crate::{Pass, SymbolTable};

use leo_ast::Ast;
use leo_errors::emitter::Handler;
use leo_errors::Result;


impl<'a> Pass<'a> for CodeGenerator<'a> {
    type Input = (&'a Ast, &'a mut SymbolTable<'a>, &'a Handler);
    type Output = Result<()>;

    fn do_pass((ast, symbol_table, handler): Self::Input) -> Self::Output {
        let mut generator = Self::new(symbol_table, handler);
        generator.visit_program(ast.as_repr());
        handler.last_err()?;

        Ok(())
    }
}
