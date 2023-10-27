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

//! The Function Inlining pass traverses the AST and inlines function at their call site.
//! See https://en.wikipedia.org/wiki/Inline_expansion for more information.
//! The pass also reorders `Function`s in a reconstructed `ProgramScope` so that they are in a post-order of the call graph.
//! In other words, a callee function will appear before a caller function in the order.
//!
//! Consider the following flattened Leo code.
//! ```leo
//! function main(flag: u8, value: u8) -> u8 {
//!     $var$0 = flag == 0u8;
//!     $var$1 = foo(value);
//!     value$2 = $var$1;
//!     value$3 = $var$0 ? value$2 : value;
//!     return value$3;
//! }
//!
//! inline foo(x: u8) -> u8 {
//!     $var$4 = x * x;
//!     return $var$4;
//! }
//! ```
//!
//! The inlining pass produces the following code.
//! ```leo
//! inline foo(x: u8) -> u8 {
//!     $var$4 = x * x;
//!     return $var$4;
//! }
//!
//! function main(flag: u8, value: u8) -> u8 {
//!     $var$0 = flag == 0u8;
//!     $var$4$5 = value * value;
//!     $var$1 = $var$4$5;
//!     value$2 = $var$1;
//!     value$3 = $var$0 ? value$2 : value;
//!     return value$3;
//! }
//! ```

pub mod assignment_renamer;
pub use assignment_renamer::*;

mod inline_expression;

mod inline_statement;

mod inline_program;

pub mod function_inliner;
pub use function_inliner::*;

use crate::{Assigner, CallGraph, Pass, TypeTable};

use leo_ast::{Ast, NodeBuilder, ProgramReconstructor};
use leo_errors::Result;

impl<'a> Pass for FunctionInliner<'a> {
    type Input = (Ast, &'a NodeBuilder, &'a CallGraph, &'a Assigner, &'a TypeTable);
    type Output = Result<Ast>;

    fn do_pass((ast, node_builder, call_graph, assigner, tt): Self::Input) -> Self::Output {
        let mut reconstructor = FunctionInliner::new(node_builder, call_graph, assigner, tt);
        let program = reconstructor.reconstruct_program(ast.into_repr());

        Ok(Ast::new(program))
    }
}
