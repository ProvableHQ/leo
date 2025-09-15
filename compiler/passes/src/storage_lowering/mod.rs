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

//! Performs lowering of `storage` variables and storage vectors into explicit `Mapping` operations.
//!
//! This pass rewrites high-level storage constructs—such as `storage` declarations and `Vector` methods concrete
//! calls to the underlying `Mapping` API used in Aleo programs. Each `storage` variable is de-sugared into one or more
//! `Mapping` instances, and all read/write operations are rewritten as `Mapping::get`, `Mapping::set`, or
//! `Mapping::get_or_use` calls.
//!
//! ### Overview
//!
//! - **Storage Variables:** Each top-level `storage` variable (e.g., `storage counter: u32;`) is represented by a
//!   `Mapping` that persists across transitions. The pass introduces ternary expressions that check whether the
//!   underlying mapping contains a value before reading it.
//!
//! - **Storage Vectors:** A `storage` vector (e.g., `storage vec: [u32];`) is lowered into two mappings:
//!   - `<vec_name>__` stores the vector elements keyed by a `u32` index.
//!   - `<vec_name>__len__` stores the current vector length under key `false`.
//!
//!   Vector operations such as `.push()`, `.pop()`, `.len()`, `.get()`, `.set()`, `.swap_remove()`, and `.clear()`
//!   are translated into combinations of `Mapping` calls and helper expressions that manipulate the length mapping and
//!   element mappings directly.
//!
//! ### Example: Storage Vector
//!
//! ```leo
//! storage vec: [u32];
//!
//! async transition test_vector_ops() -> Future {
//!     return async {
//!         vec.push(10u32);
//!         let x = vec.get(0u32).unwrap();
//!         let y = vec.pop();
//!     };
//! }
//! ```
//!
//! is lowered to:
//!
//! ```leo
//! mapping vec__: u32 => u32;       // vector values
//! mapping vec__len__: bool => u32; // length
//!
//! // vec.push(10u32);
//! let $len_var = Mapping::get_or_use(vec__len__, false, 0u32);
//! Mapping::set(vec__, $len_var, 10u32);
//! Mapping::set(vec__len__, false, $len_var + 1u32);
//!
//! // let x = vec.get(3u32);
//! let $len_var = Mapping::get_or_use(vec__len__, false, 0u32);
//! let x = 3u32 < $len_var ? Mapping::get_or_use(vec__, 3u32, 0u32) : None;
//!
//! // let y = vec.pop();
//! let $len_var = Mapping::get_or_use(vec__len__, false, 0u32);
//! if ($len_var > 0u32) { Mapping::set(vec__len__, false, $len_var - 1u32); }
//! let y = $len_var > 0u32 ? Mapping::get_or_use(vec__, $len_var - 1u32, 0u32) : None;
//! ```
//!
//! ### Example: Singleton Storage
//!
//! ```leo
//! storage counter: u32;
//!
//! async transition increment() -> Future {
//!     return async {
//!         let old = counter.unwrap_or(0u32);
//!         counter = old + 1u32;
//!     };
//! }
//! ```
//!
//! is lowered to:
//!
//! ```leo
//! mapping counter__: bool => u32;
//!
//! // let old = counter.unwrap_or(0u32)
//! let old = counter__.contains(false) ? counter__.get_or_use(false, 0u32) : 0u32
//!
//! // counter = old + 1u32
//! Mapping::set(counter__, false, old + 1u32)
//! ```

use crate::{Pass, PathResolution, SymbolTable, SymbolTableCreation, TypeChecking, TypeCheckingInput};

use leo_ast::ProgramReconstructor as _;
use leo_errors::Result;
use leo_span::Symbol;

use indexmap::IndexMap;

mod ast;

mod program;

mod visitor;
use visitor::*;

pub struct StorageLowering;

impl Pass for StorageLowering {
    type Input = TypeCheckingInput;
    type Output = ();

    const NAME: &str = "StorageLowering";

    fn do_pass(input: TypeCheckingInput, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = StorageLoweringVisitor { state, program: Symbol::intern(""), new_mappings: IndexMap::new() };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;

        // We need to recreate the symbol table and run type checking again because this pass may introduce new mappings
        // and new statements and expressions.
        visitor.state.symbol_table = SymbolTable::default();
        PathResolution::do_pass((), state)?;
        SymbolTableCreation::do_pass((), state)?;
        TypeChecking::do_pass(input.clone(), state)?;

        Ok(())
    }
}
