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

//! The halt-safe-ternary pass makes both arms of a ternary expression safe to evaluate.
//!
//! In the circuit model a wire is always computed, so both arms of `c ? A : B` are
//! evaluated regardless of `c`. If an arm contains a halting operation (division/remainder
//! by zero, arithmetic overflow, a shift wider than the type, or an out-of-range narrowing
//! cast), that halt fires even when its arm is *not* selected and aborts the whole execution.
//!
//! This pass predicates each halting operand by the conjunction of the enclosing ternary-arm
//! conditions (the *path guard* `g`), replacing the operand with `g ? operand : neutral` where
//! `neutral` makes the operation total. For example:
//! ```leo
//! c ? a / b : x
//! ```
//! becomes
//! ```leo
//! c ? a / (c ? b : 1) : x
//! ```
//! When `c` is false the divisor is `1`, so the (discarded) untaken arm cannot halt; when `c`
//! is true the real divisor is used, so a genuine fault in the taken arm still halts.
//!
//! The pass runs before SSA forming, while ternary arms are still tree-structured. After SSA
//! the arms are hoisted into unconditional temporaries, losing the association with the
//! condition that this transform relies on.
//!
//! Scope: this pass predicates integer arithmetic (`+`, `-`, `*`, `/`, `%`, `**`), integer
//! shifts (`<<`, `>>`), and narrowing casts to an integer type. Only halting operations that
//! appear syntactically inside a ternary arm are predicated; operations reached only through a
//! function call are handled when that call is inlined (a separate, later pass), and so are not
//! covered here. Programs with no halting operation in any ternary arm are left unchanged.

use crate::Pass;

use leo_ast::UnitReconstructor as _;
use leo_errors::Result;

mod ast;

mod visitor;
use visitor::*;

pub struct HaltSafeTernary;

impl Pass for HaltSafeTernary {
    type Input = ();
    type Output = ();

    const NAME: &str = "HaltSafeTernary";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let ast = std::mem::take(&mut state.ast);

        let mut visitor = HaltSafeTernaryVisitor { state, guard: None };

        let ast = ast.map(
            |program| visitor.reconstruct_program(program),
            |library| library, // no-op for libraries
        );

        visitor.state.handler.last_err()?;
        visitor.state.ast = ast;

        Ok(())
    }
}
