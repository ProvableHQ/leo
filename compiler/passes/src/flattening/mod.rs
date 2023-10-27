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

//! The flattening pass traverses the AST after the SSA pass and converts into a sequential code.
//! The pass flattens `ConditionalStatement`s into a sequence of `AssignStatement`s.
//! The pass rewrites `ReturnStatement`s into `AssignStatement`s and consolidates the returned values as a single `ReturnStatement` at the end of the function.
//! The pass rewrites ternary expressions over composite data types, into ternary expressions over the individual fields of the composite data type, followed by an expression constructing the composite data type.
//!
//! Consider the following Leo code, output by the SSA pass.
//! ```leo
//! function main(flag: u8, value: u8) -> u8 {
//!     $var$0 = flag == 0u8;
//!     if ($var$0) {
//!         $var$1 = value + 1u8;
//!         value$1 = $var$1;
//!         return value$1;
//!     } else {
//!         $var$2 = value + 2u8;
//!         value$2 = $var$2;
//!     }
//!     value$3 = $var$0 ? value$1 : value$2;
//!     return value$3;
//! }
//! ```
//!
//! The flattening pass produces the following code.
//! ```leo
//! function main(flag: u8, value: u8) -> u8 {
//!     $var$0 = flag == 0u8;
//!     $var$1 = value + 1u8;
//!     value$1 = $var$1;
//!     $var$2 = value + 2u8;
//!     value$2 = $var$2;
//!     value$3 = $var$0 ? value$1 : value$2;
//!     ret$4 = $var$0 ? value$1 : value$3;
//!     return ret$4;
//! }
//! ```

mod flatten_expression;

mod flatten_program;

mod flatten_statement;

pub mod flattener;
pub use flattener::*;

use crate::{Assigner, Pass, SymbolTable, TypeTable};

use leo_ast::{Ast, NodeBuilder, ProgramReconstructor};
use leo_errors::Result;

impl<'a> Pass for Flattener<'a> {
    type Input = (Ast, &'a SymbolTable, &'a TypeTable, &'a NodeBuilder, &'a Assigner);
    type Output = Result<Ast>;

    fn do_pass((ast, st, tt, node_builder, assigner): Self::Input) -> Self::Output {
        let mut reconstructor = Flattener::new(st, tt, node_builder, assigner);
        let program = reconstructor.reconstruct_program(ast.into_repr());

        Ok(Ast::new(program))
    }
}
