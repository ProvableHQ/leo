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

use crate::Pass;

use leo_ast::ProgramReconstructor as _;
use leo_errors::Result;
use leo_span::{Span, Symbol};

mod ast;

mod program;

mod visitor;
pub use visitor::ConstPropagationVisitor;

pub struct ConstPropagationOutput {
    /// Something about the program was actually changed during the pass.
    pub changed: bool,
    /// A const declaration whose RHS was not able to be evaluated.
    pub const_not_evaluated: Option<Span>,
    /// An array index which was not able to be evaluated.
    pub array_index_not_evaluated: Option<Span>,
    /// An array length which was not able to be evaluated.
    pub array_length_not_evaluated: Option<Span>,
    /// A repeat expression count which was not able to be evaluated.
    pub repeat_count_not_evaluated: Option<Span>,
}

/// A pass to perform const propagation and folding.
///
/// This pass should be used in conjunction with the Unroller so that
/// loop bounds and consts in loop bodies can be evaluated.
///
/// Any of these expressions:
/// 1. unary operation,
/// 2. binary operation,
/// 3. core functions other than cheat codes, mapping ops, or rand functions,
///
/// whose arguments are consts or literals will be subject to constant folding.
/// The ternary conditional operator will also be folded if its condition is
/// a constant or literal.
///
/// This includes the LHS of assignment statements which include array indices.
pub struct ConstPropagation;

impl Pass for ConstPropagation {
    type Input = ();
    type Output = ConstPropagationOutput;

    const NAME: &str = "ConstPropagation";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = ConstPropagationVisitor {
            state,
            program: Symbol::intern(""),
            module: vec![],
            changed: false,
            const_not_evaluated: None,
            array_index_not_evaluated: None,
            array_length_not_evaluated: None,
            repeat_count_not_evaluated: None,
        };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.handler.last_err().map_err(|e| *e)?;
        visitor.state.ast = ast;
        Ok(ConstPropagationOutput {
            changed: visitor.changed,
            const_not_evaluated: visitor.const_not_evaluated,
            array_index_not_evaluated: visitor.array_index_not_evaluated,
            array_length_not_evaluated: visitor.array_length_not_evaluated,
            repeat_count_not_evaluated: visitor.repeat_count_not_evaluated,
        })
    }
}

impl<'a> ConstPropagationVisitor<'a> {
    pub fn new(state: &'a mut crate::CompilerState, program: Symbol) -> Self {
        ConstPropagationVisitor {
            state,
            program,
            module: vec![],
            changed: false,
            const_not_evaluated: None,
            array_index_not_evaluated: None,
            array_length_not_evaluated: None,
            repeat_count_not_evaluated: None,
        }
    }
}
