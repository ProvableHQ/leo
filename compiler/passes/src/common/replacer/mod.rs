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

use leo_ast::{AstReconstructor, Block, Expression, IterationStatement, Node as _, ProgramReconstructor, Statement};

use crate::CompilerState;

/// A `Replacer` traverses and reconstructs the AST, applying a user-defined replacement function to each `Expression`.
///
/// For example, this can be used to:
/// - **Rename identifiers**: to systematically rename identifiers throughout the AST.
/// - **Expression interpolation**: such as substituting arguments into function bodies.
///
/// During reconstruction, node IDs for blocks and loop bodies are regenerated using a `NodeBuilder` to ensure
/// that the resulting scopes are different from the original. This avoids issues with stale or conflicting
/// parent-child relationships between scopes.
///
/// The replacement function (`replace`) is applied early in expression reconstruction. If it produces a new
/// expression (i.e., with a different node ID), it replaces the original; otherwise, the expression is
/// recursively reconstructed as usual.
///
/// Note: Only `Expression` nodes are currently subject to replacement logic; all other AST nodes are
/// reconstructed structurally.
///
/// TODO: Consider whether all nodes (not just scopes) should receive new IDs for consistency.
pub struct Replacer<'a, F>
where
    F: Fn(&Expression) -> Expression,
{
    state: &'a mut CompilerState,
    refresh_expr_ids: bool,
    replace: F,
}

impl<'a, F> Replacer<'a, F>
where
    F: Fn(&Expression) -> Expression,
{
    pub fn new(replace: F, refresh_expr_ids: bool, state: &'a mut CompilerState) -> Self {
        Self { replace, refresh_expr_ids, state }
    }
}

impl<F> AstReconstructor for Replacer<'_, F>
where
    F: Fn(&Expression) -> Expression,
{
    type AdditionalInput = ();
    type AdditionalOutput = ();

    fn reconstruct_expression(&mut self, input: Expression, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        let opt_old_type = self.state.type_table.get(&input.id());
        let replaced_expr = (self.replace)(&input);
        let (mut new_expr, additional) = if replaced_expr.id() == input.id() {
            // Replacement didn't happen, so just use the default implementation.
            match input {
                Expression::Intrinsic(intr) => self.reconstruct_intrinsic(*intr, &()),
                Expression::Async(async_) => self.reconstruct_async(async_, &()),
                Expression::Array(array) => self.reconstruct_array(array, &()),
                Expression::ArrayAccess(access) => self.reconstruct_array_access(*access, &()),
                Expression::Binary(binary) => self.reconstruct_binary(*binary, &()),
                Expression::Call(call) => self.reconstruct_call(*call, &()),
                Expression::Cast(cast) => self.reconstruct_cast(*cast, &()),
                Expression::Composite(composite) => self.reconstruct_composite_init(composite, &()),
                Expression::Err(err) => self.reconstruct_err(err, &()),
                Expression::Path(path) => self.reconstruct_path(path, &()),
                Expression::Literal(value) => self.reconstruct_literal(value, &()),
                Expression::Locator(locator) => self.reconstruct_locator(locator, &()),
                Expression::MemberAccess(access) => self.reconstruct_member_access(*access, &()),
                Expression::Repeat(repeat) => self.reconstruct_repeat(*repeat, &()),
                Expression::Ternary(ternary) => self.reconstruct_ternary(*ternary, &()),
                Expression::Tuple(tuple) => self.reconstruct_tuple(tuple, &()),
                Expression::TupleAccess(access) => self.reconstruct_tuple_access(*access, &()),
                Expression::Unary(unary) => self.reconstruct_unary(*unary, &()),
                Expression::Unit(unit) => self.reconstruct_unit(unit, &()),
            }
        } else {
            (replaced_expr, Default::default())
        };

        // Refresh IDs if required
        if self.refresh_expr_ids {
            new_expr.set_id(self.state.node_builder.next_id());
        }

        // If the expression was in the type table before, make an entry for the new expression.
        if let Some(old_type) = opt_old_type {
            self.state.type_table.insert(new_expr.id(), old_type);
        }

        (new_expr, additional)
    }

    fn reconstruct_block(&mut self, input: Block) -> (Block, Self::AdditionalOutput) {
        (
            Block {
                statements: input.statements.into_iter().map(|s| self.reconstruct_statement(s).0).collect(),
                span: input.span,
                id: self.state.node_builder.next_id(),
            },
            Default::default(),
        )
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        (
            IterationStatement {
                type_: input.type_.map(|ty| self.reconstruct_type(ty).0),
                start: self.reconstruct_expression(input.start, &()).0,
                stop: self.reconstruct_expression(input.stop, &()).0,
                block: self.reconstruct_block(input.block).0,
                id: self.state.node_builder.next_id(),
                ..input
            }
            .into(),
            Default::default(),
        )
    }
}

impl<F> ProgramReconstructor for Replacer<'_, F> where F: Fn(&Expression) -> Expression {}
