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

//! The destructuring pass traverses the AST and destructures tuples into individual variables.
//! This pass assumes that tuples have a depth of 1, which is ensured by the type checking pass.

mod destructure_expression;

mod destructure_program;

mod destructure_statement;

pub mod destructurer;
pub use destructurer::*;

use crate::{Assigner, Pass, TypeTable};

use leo_ast::{Ast, NodeBuilder, ProgramReconstructor};
use leo_errors::Result;

impl<'a> Pass for Destructurer<'a> {
    type Input = (Ast, &'a TypeTable, &'a NodeBuilder, &'a Assigner);
    type Output = Result<Ast>;

    fn do_pass((ast, tt, node_builder, assigner): Self::Input) -> Self::Output {
        let mut reconstructor = Destructurer::new(tt, node_builder, assigner);
        let program = reconstructor.reconstruct_program(ast.into_repr());

        Ok(Ast::new(program))
    }
}
