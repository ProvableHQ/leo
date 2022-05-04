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

pub mod check;
pub use check::*;

pub mod checker;
pub use checker::*;

use crate::{Pass, SymbolTable};

use leo_ast::{Ast, VisitorDirector};
use leo_errors::{emitter::Handler, Result};

impl<'a> Pass<'a> for TypeChecker<'a> {
    type Input = (&'a Ast, &'a mut SymbolTable<'a>, &'a Handler);
    type Output = Result<()>;

    fn do_pass((ast, symbol_table, handler): Self::Input) -> Self::Output {
        let mut visitor = VisitorDirector::new(TypeChecker::new(symbol_table, handler));
        visitor.visit_program(ast.as_repr());
        handler.last_err()?;

        Ok(())
    }
}
