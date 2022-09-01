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

// TODO: Fix documentation.

//! The Static Single Assignment pass traverses the AST and converts it into SSA form.
//! See https://en.wikipedia.org/wiki/Static_single-assignment_form for more information.
//! The pass also flattens `ConditionalStatement`s into a sequence of `AssignStatement`s.
//! The pass also rewrites `ReturnStatement`s into `AssignStatement`s and consolidates the returned values as a single `ReturnStatement` at the end of the function.
//! The pass also simplifies complex expressions into a sequence of `AssignStatement`s. For example, `(a + b) * c` is rewritten into `$var$1 = a + b; $var$2 = $var$1 * c`.
//!
//! Consider the following Leo code.
//! ```leo
//! function main(flag: u8, value: u8) -> u8 {
//!     if (flag == 0u8) {
//!         value += 1u8;
//!         return value;
//!     } else {
//!         value += 2u8;
//!     }
//!     return value;
//! }
//! ```
//!
//! The SSA pass produces the following code.
//! ```leo
//! function main(flag: u8, value: u8) -> u8 {
//!     let $cond$0 = flag == 0u8;
//!     let value$1 = value + 1u8;
//!     let $return$2 = value$1;
//!     let value$3 = value + 2u8;
//!     let value$4 = $cond$0 ? value$1 : value$4;
//!     let $return$5 = value$4;
//!     return $cond$0 ? $return$2 : $return$5;
//! ```

mod flatten_expression;

mod flatten_program;

mod flatten_statement;

pub mod flattener;
pub use flattener::*;

use crate::{Assigner, Pass, SymbolTable};

use leo_ast::{Ast, ProgramReconstructor};
use leo_errors::{Result};

impl<'a> Pass for Flattener<'a> {
    type Input = (Ast, &'a SymbolTable, Assigner);
    type Output = Result<Ast>;

    fn do_pass((ast, st, assigner): Self::Input) -> Self::Output {
        let mut reconstructor = Flattener::new(st, assigner);
        let program = reconstructor.reconstruct_program(ast.into_repr());

        Ok(Ast::new(program))
    }
}
