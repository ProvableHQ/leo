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

use crate::Pass;

use indexmap::IndexMap;
use leo_ast::ProgramReconstructor as _;
use leo_errors::Result;
use leo_span::Symbol;

mod ast;

mod program;

mod visitor;
use visitor::*;

pub struct FunctionInlining;

impl Pass for FunctionInlining {
    type Input = ();
    type Output = ();

    const NAME: &str = "FunctionInlining";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = FunctionInliningVisitor {
            state,
            reconstructed_functions: Vec::new(),
            program: Symbol::intern(""),
            function_map: IndexMap::new(),
            is_async: false,
        };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err().map_err(|e| *e)?;
        visitor.state.ast = ast;
        Ok(())
    }
}
