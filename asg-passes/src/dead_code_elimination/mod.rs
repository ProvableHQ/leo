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

use std::cell::Cell;

use leo_asg::*;
use leo_errors::Result;

pub struct DeadCodeElimination {}

impl<'a> ReconstructingReducerExpression<'a> for DeadCodeElimination {}

impl<'a> ReconstructingReducerProgram<'a> for DeadCodeElimination {}

impl<'a> ReconstructingReducerStatement<'a> for DeadCodeElimination {
    ///
    /// Removes dead code inside a false conditional statement block.
    ///
    fn reduce_statement_alloc(
        &mut self,
        context: AsgContext<'a>,
        _input: &'a Statement<'a>,
        value: Statement<'a>,
    ) -> &'a Statement<'a> {
        match &value {
            Statement::Conditional(conditional) => match conditional.condition.get().const_value() {
                Some(ConstValue::Boolean(true)) => conditional.result.get(),
                Some(ConstValue::Boolean(false)) => {
                    if let Some(if_false) = conditional.next.get() {
                        if_false
                    } else {
                        context.alloc_statement(Statement::Empty(conditional.span.clone()))
                    }
                }
                _ => context.alloc_statement(value),
            },
            _ => context.alloc_statement(value),
        }
    }

    fn reduce_block(&mut self, input: BlockStatement<'a>, mut statements: Vec<&'a Statement<'a>>) -> Statement<'a> {
        let first_return = statements.iter().position(|x| matches!(x, Statement::Return(_)));
        if let Some(first_return) = first_return {
            statements.truncate(first_return + 1);
        }
        Statement::Block(BlockStatement {
            id: input.id,
            parent: input.parent,
            span: input.span,
            statements: statements.into_iter().map(Cell::new).collect(),
            scope: input.scope,
        })
    }
}

impl<'a> AsgPass<'a> for DeadCodeElimination {
    type Input = Program<'a>;
    type Output = Result<Program<'a>>;

    fn do_pass(asg: Self::Input) -> Self::Output {
        let pass = DeadCodeElimination {};
        let mut director = ReconstructingDirector::new(asg.context, pass);
        Ok(director.reduce_program(asg))
    }
}
