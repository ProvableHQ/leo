// Copyright (C) 2019-2026 Provable Inc.
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

//! Forward repeated local finalize storage reads until a conservative effect boundary.
//!
//! This pass only handles lowered static mapping intrinsics. Dynamic storage
//! reads carry target program and network operands and are left to a separate
//! optimization so their invalidation model can be reviewed independently.
//!
//! Read facts are always cleared at branch joins. Aliases created inside a
//! branch are only exposed to the matching arm of same-condition SSA join
//! ternaries emitted immediately after the branch, so branch-local definitions
//! are not treated as globally available after the join. Pending branch aliases
//! are discarded as soon as a non-join statement is encountered.
//!
//! Duplicate storage reads are rewritten to explicit local copies instead of
//! removed in this pass. This keeps the transformed AST well-formed even when a
//! later path-sensitive use is intentionally not rewritten; the following DCE
//! pass is responsible for deleting copies whose uses were fully forwarded.

use crate::{CompilerState, Pass};

use leo_ast::UnitReconstructor as _;
use leo_errors::Result;

mod ast;
mod program;
#[cfg(test)]
mod tests;
mod visitor;

use visitor::StorageReadForwardingVisitor;

pub struct StorageReadForwarding;

impl Pass for StorageReadForwarding {
    type Input = ();
    type Output = ();

    const NAME: &str = "StorageReadForwarding";

    fn do_pass(_input: Self::Input, state: &mut CompilerState) -> Result<Self::Output> {
        let ast = std::mem::take(&mut state.ast);
        let mut visitor = StorageReadForwardingVisitor {
            state,
            reads: Default::default(),
            aliases: Default::default(),
            then_join_aliases: Default::default(),
            otherwise_join_aliases: Default::default(),
            join_condition: None,
            in_finalize_context: false,
        };

        let ast = ast.map(
            |program| visitor.reconstruct_program(program),
            |library| library, // no-op for libraries
        );

        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;
        Ok(())
    }
}
