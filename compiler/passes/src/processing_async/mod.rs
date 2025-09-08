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

//! The `ProcessingAsync` pass rewrites `async { ... }` blocks into standalone
//! `async function`s. Each block is lifted to a new top-level async function,
//! and the block is replaced with a call to that function.
//!
//! This involves:
//! - Capturing all variable and tuple field accesses used inside the block.
//! - Filtering out globals and locals (which are handled differently).
//! - Generating fresh function inputs for the captured values.
//! - Rewriting the block with replacements for captured expressions.
//! - Creating a `CallExpression` that invokes the synthesized async function.
//!
//! If any async blocks were rewritten, this pass will rebuild the symbol table
//! and rerun path resolution and type checking to account for the new functions.
//!
//! # Example
//! ```leo
//! async transition foo(x: u32) -> Future {
//!     return async {
//!         assert(x == 1);  
//!     };
//! }
//! ```
//! becomes
//! ```leo
//! async function foo_(x: u32) {
//!     assert(x == 1);
//! }
//!
//! transition foo(x: u32) -> Future {
//!     return foo_(x);
//! }
//! ```

use crate::{Pass, PathResolution, SymbolTable, SymbolTableCreation, TypeChecking, TypeCheckingInput};

use leo_ast::ProgramReconstructor as _;
use leo_errors::Result;
use leo_span::Symbol;

mod ast;

mod program;

mod visitor;
use visitor::*;

pub struct ProcessingAsync;

impl Pass for ProcessingAsync {
    type Input = TypeCheckingInput;
    type Output = ();

    const NAME: &str = "ProcessingAsync";

    fn do_pass(input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = ProcessingAsyncVisitor {
            state,
            max_inputs: input.max_inputs,
            current_program: Symbol::intern(""),
            current_function: Symbol::intern(""),
            new_async_functions: Vec::new(),
            modified: false,
        };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err().map_err(|e| *e)?;
        visitor.state.ast = ast;

        if visitor.modified {
            // If we actually changed anything in the program, then we need to recreate the symbol table and run type
            // checking again. That's because this pass introduces new `async function`s to the program.
            visitor.state.symbol_table = SymbolTable::default();
            PathResolution::do_pass((), state)?;
            SymbolTableCreation::do_pass((), state)?;
            TypeChecking::do_pass(input.clone(), state)?;
        }

        Ok(())
    }
}
