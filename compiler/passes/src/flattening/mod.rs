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

//! The flattening pass traverses the AST after the SSA pass and converts into a sequential code.
//! The pass flattens `ConditionalStatement`s into a sequence of `AssignStatement`s.
//! The pass rewrites `ReturnStatement`s into `AssignStatement`s and consolidates the returned values as a single `ReturnStatement` at the end of the function.
//! The pass rewrites ternary expressions over composite data types, into ternary expressions over the individual fields of the composite data type, followed by an expression constructing the composite data type.
//! Note that this transformation is not applied to async functions.
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

use crate::Pass;

use leo_ast::ProgramReconstructor as _;
use leo_errors::Result;
use leo_span::Symbol;

mod ast;

mod program;

mod visitor;
use visitor::*;

pub struct Flattening;

impl Pass for Flattening {
    type Input = ();
    type Output = ();

    const NAME: &str = "Flattening";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = FlatteningVisitor {
            state,
            condition_stack: Vec::new(),
            returns: Vec::new(),
            program: Symbol::intern(""),
            is_async: false,
        };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err().map_err(|e| *e)?;
        visitor.state.ast = ast;
        Ok(())
    }
}
