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

mod duplicate;

mod range_iterator;
pub(crate) use range_iterator::*;

pub mod unroller;
pub use unroller::*;

pub mod unroll_program;

pub mod unroll_statement;

use crate::{Pass, SymbolTable, TypeTable};

use leo_ast::{Ast, NodeBuilder, ProgramReconstructor};
use leo_errors::{Result, emitter::Handler};
use leo_span::Span;

pub struct UnrollerOutput {
    /// If we encountered a loop that was not unrolled, here's it's spanned.
    pub loop_not_unrolled: Option<Span>,
    /// Did we unroll any loop?
    pub loop_unrolled: bool,
}

impl<'a> Pass for Unroller<'a> {
    type Input = (Ast, &'a Handler, &'a NodeBuilder, &'a mut SymbolTable, &'a TypeTable);
    type Output = Result<(Ast, UnrollerOutput)>;

    const NAME: &'static str = "Unroller";

    fn do_pass((ast, handler, node_builder, symbol_table, tt): Self::Input) -> Self::Output {
        let mut reconstructor = Self::new(symbol_table, tt, handler, node_builder);
        let program = reconstructor.reconstruct_program(ast.into_repr());
        handler.last_err().map_err(|e| *e)?;

        Ok((Ast::new(program), UnrollerOutput {
            loop_not_unrolled: reconstructor.loop_not_unrolled,
            loop_unrolled: reconstructor.loop_unrolled,
        }))
    }
}
