// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{
    statement::*,
    BoolAnd,
    Expression,
    Monoid,
    MonoidalReducerExpression,
    MonoidalReducerStatement,
    Node,
    Span,
};

use std::sync::Arc;

pub struct ReturnPathReducer {
    pub errors: Vec<(Span, String)>,
}

impl ReturnPathReducer {
    fn record_error(&mut self, span: Option<&Span>, error: String) {
        self.errors.push((span.cloned().unwrap_or_default(), error));
    }

    pub fn new() -> ReturnPathReducer {
        ReturnPathReducer { errors: vec![] }
    }
}

impl Default for ReturnPathReducer {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused_variables)]
impl MonoidalReducerExpression<BoolAnd> for ReturnPathReducer {
    fn reduce_expression(&mut self, input: &Arc<Expression>, value: BoolAnd) -> BoolAnd {
        BoolAnd(false)
    }
}

#[allow(unused_variables)]
impl MonoidalReducerStatement<BoolAnd> for ReturnPathReducer {
    fn reduce_assign_access(&mut self, input: &AssignAccess, left: Option<BoolAnd>, right: Option<BoolAnd>) -> BoolAnd {
        BoolAnd(false)
    }

    fn reduce_assign(&mut self, input: &AssignStatement, accesses: Vec<BoolAnd>, value: BoolAnd) -> BoolAnd {
        BoolAnd(false)
    }

    fn reduce_block(&mut self, input: &BlockStatement, statements: Vec<BoolAnd>) -> BoolAnd {
        if statements.is_empty() {
            BoolAnd(false)
        } else if let Some(index) = statements[..statements.len() - 1].iter().map(|x| x.0).position(|x| x) {
            self.record_error(
                input.statements[index].span(),
                "dead code due to unconditional early return".to_string(),
            );
            BoolAnd(true)
        } else {
            BoolAnd(statements[statements.len() - 1].0)
        }
    }

    fn reduce_conditional_statement(
        &mut self,
        input: &ConditionalStatement,
        condition: BoolAnd,
        if_true: BoolAnd,
        if_false: Option<BoolAnd>,
    ) -> BoolAnd {
        if if_false.as_ref().map(|x| x.0).unwrap_or(false) != if_true.0 {
            self.record_error(
                input.span(),
                "cannot have asymmetrical return in if statement".to_string(),
            );
        }
        if_true.append(if_false.unwrap_or(BoolAnd(false)))
    }

    fn reduce_formatted_string(&mut self, input: &FormattedString, parameters: Vec<BoolAnd>) -> BoolAnd {
        BoolAnd(false)
    }

    fn reduce_console(&mut self, input: &ConsoleStatement, argument: BoolAnd) -> BoolAnd {
        BoolAnd(false)
    }

    fn reduce_definition(&mut self, input: &DefinitionStatement, value: BoolAnd) -> BoolAnd {
        BoolAnd(false)
    }

    fn reduce_expression_statement(&mut self, input: &ExpressionStatement, expression: BoolAnd) -> BoolAnd {
        BoolAnd(false)
    }

    fn reduce_iteration(
        &mut self,
        input: &IterationStatement,
        start: BoolAnd,
        stop: BoolAnd,
        body: BoolAnd,
    ) -> BoolAnd {
        // loops are const defined ranges, so we could probably check if they run one and emit here
        BoolAnd(false)
    }

    fn reduce_return(&mut self, input: &ReturnStatement, value: BoolAnd) -> BoolAnd {
        BoolAnd(true)
    }
}
