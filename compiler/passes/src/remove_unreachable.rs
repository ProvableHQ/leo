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

use crate::{CompilerState, Pass};

use leo_ast::*;
use leo_errors::Result;

pub struct RemoveUnreachableOutput {
    /// Something about the program was actually changed during the pass.
    pub changed: bool,
}

/// Pass that removes unreachable code created by early returns
pub struct RemoveUnreachable;

impl Pass for RemoveUnreachable {
    type Input = ();
    type Output = RemoveUnreachableOutput;

    const NAME: &str = "RemoveUnreachable";

    fn do_pass(_input: Self::Input, state: &mut crate::CompilerState) -> Result<Self::Output> {
        let mut ast = std::mem::take(&mut state.ast);
        let mut visitor = RemoveUnreachableVisitor { changed: false, state, has_return: false };
        ast.ast = visitor.reconstruct_program(ast.ast);
        visitor.state.ast = ast;
        Ok(RemoveUnreachableOutput { changed: visitor.changed })
    }
}

pub struct RemoveUnreachableVisitor<'state> {
    pub state: &'state mut CompilerState,
    /// Have we actually modified the program at all?
    pub changed: bool,
    /// Have we already returned in the current scope?
    pub has_return: bool,
}

impl ProgramReconstructor for RemoveUnreachableVisitor<'_> {
    fn reconstruct_function(&mut self, input: Function) -> Function {
        self.has_return = false;
        let res = Function {
            annotations: input.annotations,
            variant: input.variant,
            identifier: input.identifier,
            const_parameters: input
                .const_parameters
                .iter()
                .map(|param| ConstParameter { type_: self.reconstruct_type(param.type_.clone()).0, ..param.clone() })
                .collect(),
            input: input
                .input
                .iter()
                .map(|input| Input { type_: self.reconstruct_type(input.type_.clone()).0, ..input.clone() })
                .collect(),
            output: input
                .output
                .iter()
                .map(|output| Output { type_: self.reconstruct_type(output.type_.clone()).0, ..output.clone() })
                .collect(),
            output_type: self.reconstruct_type(input.output_type).0,
            block: self.reconstruct_block(input.block).0,
            span: input.span,
            id: input.id,
        };
        self.has_return = false;
        res
    }

    fn reconstruct_constructor(&mut self, input: Constructor) -> Constructor {
        self.has_return = false;
        let res = Constructor {
            annotations: input.annotations,
            block: self.reconstruct_block(input.block).0,
            span: input.span,
            id: input.id,
        };
        self.has_return = false;
        res
    }
}

impl AstReconstructor for RemoveUnreachableVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    fn reconstruct_block(&mut self, input: Block) -> (Block, Self::AdditionalOutput) {
        // Produce every reconstructed statement until you see an unconditional return
        // including the return itself
        let statements_with_first_return_only = input
            .statements
            .into_iter()
            .scan(false, |return_seen, s| {
                let stmt = self.reconstruct_statement(s).0;
                let res = (!*return_seen).then_some(stmt);
                *return_seen |= self.has_return;
                res
            })
            .filter_map(Some)
            .collect();
        (Block { statements: statements_with_first_return_only, span: input.span, id: input.id }, Default::default())
    }

    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        // Conditionals have their own scopes with respect to seeing returns because they are not always executed
        // We save the current has_return value and run the conditional scopes independently
        let mut then_block_has_return = false;
        let mut otherwise_block_has_return = false;
        let previous_has_return = core::mem::replace(&mut self.has_return, then_block_has_return);

        let then = self.reconstruct_block(input.then).0;
        then_block_has_return = self.has_return;

        let otherwise = input.otherwise.map(|otherwise| {
            self.has_return = otherwise_block_has_return;
            let res = Box::new(self.reconstruct_statement(*otherwise).0);
            otherwise_block_has_return = self.has_return;
            res
        });

        // Either we already had returned or return is certain because both branches have returned
        self.has_return = previous_has_return || (then_block_has_return && otherwise_block_has_return);

        (
            ConditionalStatement {
                condition: self.reconstruct_expression(input.condition, &Default::default()).0,
                then,
                otherwise,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_iteration(&mut self, input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        let prior_has_return = core::mem::take(&mut self.has_return);
        let block = self.reconstruct_block(input.block).0;
        self.has_return = prior_has_return;

        (
            IterationStatement {
                type_: input.type_.map(|ty| self.reconstruct_type(ty).0),
                start: self.reconstruct_expression(input.start, &Default::default()).0,
                stop: self.reconstruct_expression(input.stop, &Default::default()).0,
                block,
                ..input
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        self.has_return = true;
        (
            ReturnStatement {
                expression: self.reconstruct_expression(input.expression, &Default::default()).0,
                ..input
            }
            .into(),
            Default::default(),
        )
    }
}
