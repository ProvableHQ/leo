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

use leo_ast::{
    Block,
    Expression,
    ExpressionReconstructor,
    Identifier,
    IterationStatement,
    NodeBuilder,
    ProgramReconstructor,
    Statement,
    StatementReconstructor,
    TypeReconstructor,
};

/// A `Replacer` applies `replacer` to all `Identifier`s in an AST.
/// `Replacer`s are used to rename identifiers.
/// `Replacer`s are used to interpolate function arguments.
///
/// All scopes are given new node IDs to avoid stale parent/children relationship between nodes.
///
/// TODO: should we give new IDs to everything?
pub struct Replacer<'a, F>
where
    F: Fn(&Identifier) -> Expression,
{
    node_builder: &'a NodeBuilder,
    replace: F,
}

impl<'a, F> Replacer<'a, F>
where
    F: Fn(&Identifier) -> Expression,
{
    pub fn new(replace: F, node_builder: &'a NodeBuilder) -> Self {
        Self { replace, node_builder }
    }
}

impl<F> TypeReconstructor for Replacer<'_, F> where F: Fn(&Identifier) -> Expression {}

impl<F> ExpressionReconstructor for Replacer<'_, F>
where
    F: Fn(&Identifier) -> Expression,
{
    type AdditionalOutput = ();

    fn reconstruct_identifier(&mut self, input: Identifier) -> (Expression, Self::AdditionalOutput) {
        ((self.replace)(&input), Default::default())
    }
}

impl<F> StatementReconstructor for Replacer<'_, F>
where
    F: Fn(&Identifier) -> Expression,
{
    fn reconstruct_block(&mut self, input: Block) -> (Block, Self::AdditionalOutput) {
        (
            Block {
                statements: input.statements.into_iter().map(|s| self.reconstruct_statement(s).0).collect(),
                span: input.span,
                id: self.node_builder.next_id(),
            },
            Default::default(),
        )
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        (
            IterationStatement {
                type_: input.type_.map(|ty| self.reconstruct_type(ty).0),
                start: self.reconstruct_expression(input.start).0,
                stop: self.reconstruct_expression(input.stop).0,
                block: self.reconstruct_block(input.block).0,
                id: self.node_builder.next_id(),
                ..input
            }
            .into(),
            Default::default(),
        )
    }
}

impl<F> ProgramReconstructor for Replacer<'_, F> where F: Fn(&Identifier) -> Expression {}
