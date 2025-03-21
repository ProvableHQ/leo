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
use leo_span::Span;

mod const_propagate_expression;

mod const_propagate_program;

mod const_propagate_statement;

mod const_propagator;

pub use const_propagator::ConstPropagator;

pub struct ConstPropagatorOutput {
    /// Something about the program was actually changed during the pass.
    pub changed: bool,
    /// A const declaration whose RHS was not able to be evaluated.
    pub const_not_evaluated: Option<Span>,
    /// An array index which was not able to be evaluated.
    pub array_index_not_evaluated: Option<Span>,
}

impl<'a> Pass for ConstPropagator<'a> {
    type Input = (Ast, &'a Handler, &'a mut SymbolTable, &'a TypeTable, &'a NodeBuilder);
    type Output = Result<(Ast, ConstPropagatorOutput)>;

    const NAME: &'static str = "ConstPropagator";

    fn do_pass((ast, handler, symbol_table, type_table, node_builder): Self::Input) -> Self::Output {
        let mut reconstructor = ConstPropagator::new(handler, symbol_table, type_table, node_builder);
        let program = reconstructor.reconstruct_program(ast.into_repr());
        handler.last_err().map_err(|e| *e)?;

        Ok((Ast::new(program), ConstPropagatorOutput {
            changed: reconstructor.changed,
            const_not_evaluated: reconstructor.const_not_evaluated,
            array_index_not_evaluated: reconstructor.array_index_not_evaluated,
        }))
    }
}
