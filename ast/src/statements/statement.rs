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
    Assignee,
    Block,
    ConditionalStatement,
    ConsoleFunctionCall,
    Declare,
    Expression,
    Identifier,
    Span,
    Variables,
};
use leo_grammar::{
    console::ConsoleFunctionCall as GrammarConsoleFunctionCall,
    operations::AssignOperation,
    statements::{
        AssignStatement,
        DefinitionStatement,
        ExpressionStatement,
        ForStatement,
        ReturnStatement,
        Statement as GrammarStatement,
    },
};

use serde::{Deserialize, Serialize};
use std::fmt;

/// Program statement that defines some action (or expression) to be carried out.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Statement {
    Return(Expression, Span),
    Definition(Declare, Variables, Expression, Span),
    Assign(Assignee, Expression, Span),
    Conditional(ConditionalStatement, Span),
    Iteration(Identifier, Box<(Expression, Expression)>, Block, Span),
    Console(ConsoleFunctionCall),
    Expression(Expression, Span),
}

impl<'ast> From<ReturnStatement<'ast>> for Statement {
    fn from(statement: ReturnStatement<'ast>) -> Self {
        Statement::Return(Expression::from(statement.expression), Span::from(statement.span))
    }
}

impl<'ast> From<DefinitionStatement<'ast>> for Statement {
    fn from(statement: DefinitionStatement<'ast>) -> Self {
        let span = Span::from(statement.span);

        Statement::Definition(
            Declare::from(statement.declare),
            Variables::from(statement.variables),
            Expression::from(statement.expression),
            span,
        )
    }
}

impl<'ast> From<AssignStatement<'ast>> for Statement {
    fn from(statement: AssignStatement<'ast>) -> Self {
        match statement.assign {
            AssignOperation::Assign(ref _assign) => Statement::Assign(
                Assignee::from(statement.assignee),
                Expression::from(statement.expression),
                Span::from(statement.span),
            ),
            operation_assign => {
                // convert assignee into postfix expression
                let converted = Expression::from(statement.assignee.clone());

                match operation_assign {
                    AssignOperation::AddAssign(ref _assign) => Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Add(
                            Box::new((converted, Expression::from(statement.expression))),
                            Span::from(statement.span.clone()),
                        ),
                        Span::from(statement.span),
                    ),
                    AssignOperation::SubAssign(ref _assign) => Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Sub(
                            Box::new((converted, Expression::from(statement.expression))),
                            Span::from(statement.span.clone()),
                        ),
                        Span::from(statement.span),
                    ),
                    AssignOperation::MulAssign(ref _assign) => Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Mul(
                            Box::new((converted, Expression::from(statement.expression))),
                            Span::from(statement.span.clone()),
                        ),
                        Span::from(statement.span),
                    ),
                    AssignOperation::DivAssign(ref _assign) => Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Div(
                            Box::new((converted, Expression::from(statement.expression))),
                            Span::from(statement.span.clone()),
                        ),
                        Span::from(statement.span),
                    ),
                    AssignOperation::PowAssign(ref _assign) => Statement::Assign(
                        Assignee::from(statement.assignee),
                        Expression::Pow(
                            Box::new((converted, Expression::from(statement.expression))),
                            Span::from(statement.span.clone()),
                        ),
                        Span::from(statement.span),
                    ),
                    AssignOperation::Assign(ref _assign) => unimplemented!("cannot assign twice to assign statement"),
                }
            }
        }
    }
}

impl<'ast> From<ForStatement<'ast>> for Statement {
    fn from(statement: ForStatement<'ast>) -> Self {
        Statement::Iteration(
            Identifier::from(statement.index),
            Box::new((Expression::from(statement.start), Expression::from(statement.stop))),
            Block::from(statement.block),
            Span::from(statement.span),
        )
    }
}

impl<'ast> From<GrammarConsoleFunctionCall<'ast>> for Statement {
    fn from(function_call: GrammarConsoleFunctionCall<'ast>) -> Self {
        Statement::Console(ConsoleFunctionCall::from(function_call))
    }
}

impl<'ast> From<ExpressionStatement<'ast>> for Statement {
    fn from(statement: ExpressionStatement<'ast>) -> Self {
        let span = Span::from(statement.span);
        let mut expression = Expression::from(statement.expression);

        expression.set_span(span.clone());

        Statement::Expression(expression, span)
    }
}

impl<'ast> From<GrammarStatement<'ast>> for Statement {
    fn from(statement: GrammarStatement<'ast>) -> Self {
        match statement {
            GrammarStatement::Return(statement) => Statement::from(statement),
            GrammarStatement::Definition(statement) => Statement::from(statement),
            GrammarStatement::Assign(statement) => Statement::from(statement),
            GrammarStatement::Conditional(statement) => {
                let span = Span::from(statement.span.clone());
                Statement::Conditional(ConditionalStatement::from(statement), span)
            }
            GrammarStatement::Iteration(statement) => Statement::from(statement),
            GrammarStatement::Console(console) => Statement::from(console),
            GrammarStatement::Expression(statement) => Statement::from(statement),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Return(ref expression, ref _span) => write!(f, "return {}", expression),
            Statement::Definition(ref declare, ref variable, ref expression, ref _span) => {
                write!(f, "{} {} = {};", declare, variable, expression)
            }
            Statement::Assign(ref variable, ref statement, ref _span) => write!(f, "{} = {};", variable, statement),
            Statement::Conditional(ref statement, ref _span) => write!(f, "{}", statement),
            Statement::Iteration(ref var, ref start_stop, ref block, ref _span) => {
                write!(f, "for {} in {}..{} {}", var, start_stop.0, start_stop.1, block)
            }
            Statement::Console(ref console) => write!(f, "{}", console),
            Statement::Expression(ref expression, ref _span) => write!(f, "{};", expression),
        }
    }
}
