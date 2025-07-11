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

//! Performs monomorphization of const generic functions within a `ProgramScope`.
//!
//! This pass identifies all `inline` functions that take const generic parameters (e.g., `foo::[N: u32]()`)
//! and replaces each unique instantiation (e.g., `foo::[3]()`, `foo::[7]()`) with a concrete version of the function
//! where the const parameter is replaced with its actual value. These concrete instances are generated, added to the
//! reconstructed function list, and inserted into the final program scope.
//!
//! If a function has been monomorphized and is no longer referenced by any unresolved calls,
//! it will be removed from the reconstructed functions and pruned from the call graph.
//!
//! ### Example
//!
//! ```leo
//! transition main(x: u32, y: u32) -> u32 {
//!     return foo::[3u32]() + foo::[7u32]();
//! }
//!
//! inline foo::[N: u32]() -> u32 {
//!     return N;
//! }
//! ```
//!
//! In the example above:
//! - `foo::[3u32]()` and `foo::[7u32]()` are two distinct instantiations of `foo::[N]`.
//! - This pass will generate two monomorphized versions of `foo`, one for each unique const argument `M` and `P`.
//! - These are inserted into the output `ProgramScope` as separate functions with unique names.
//! - If `foo::[N]` is no longer referenced in any calls, the original generic function is removed.

use crate::Pass;

use leo_ast::{CompositeType, ProgramReconstructor as _, StructExpression};
use leo_errors::Result;
use leo_span::Symbol;

mod expression;

mod program;

mod type_;

mod visitor;
use visitor::*;

pub struct MonomorphizationOutput {
    /// If we encountered calls to const generic functions that were not resolved, keep track of them in this vector
    pub unresolved_calls: Vec<leo_ast::CallExpression>,
    /// If we encountered const generic struct expressions that were not resolved, keep track of them in this vector
    pub unresolved_struct_exprs: Vec<StructExpression>,
    /// If we encountered const generic struct type instantiations that were not resolved, keep track of them in this
    /// vector
    pub unresolved_struct_types: Vec<CompositeType>,
    /// Did we change anything in this program?
    pub changed: bool,
}

pub struct Monomorphization;

impl Pass for Monomorphization {
    type Input = ();
    type Output = MonomorphizationOutput;

    const NAME: &str = "Monomorphization";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = MonomorphizationVisitor {
            state,
            program: Symbol::intern(""),
            reconstructed_functions: indexmap::IndexMap::new(),
            monomorphized_functions: indexmap::IndexSet::new(),
            reconstructed_structs: indexmap::IndexMap::new(),
            monomorphized_structs: indexmap::IndexSet::new(),
            unresolved_calls: Vec::new(),
            unresolved_struct_exprs: Vec::new(),
            unresolved_struct_types: Vec::new(),
            changed: false,
        };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err().map_err(|e| *e)?;
        visitor.state.ast = ast;

        Ok(MonomorphizationOutput {
            unresolved_calls: visitor.unresolved_calls,
            unresolved_struct_exprs: visitor.unresolved_struct_exprs,
            unresolved_struct_types: visitor.unresolved_struct_types,
            changed: visitor.changed,
        })
    }
}
