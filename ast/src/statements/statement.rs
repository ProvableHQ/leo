use crate::{ast::Rule, console::ConsoleFunctionCall, statements::*};

use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::statement))]
pub enum Statement<'ast> {
    Return(ReturnStatement<'ast>),
    Definition(DefinitionStatement<'ast>),
    Assign(AssignStatement<'ast>),
    Conditional(ConditionalStatement<'ast>),
    Iteration(ForStatement<'ast>),
    Console(ConsoleFunctionCall<'ast>),
    Expression(ExpressionStatement<'ast>),
}

impl<'ast> fmt::Display for Statement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Return(ref statement) => write!(f, "{}", statement),
            Statement::Definition(ref statement) => write!(f, "{}", statement),
            Statement::Assign(ref statement) => write!(f, "{}", statement),
            Statement::Conditional(ref statement) => write!(f, "{}", statement),
            Statement::Iteration(ref statement) => write!(f, "{}", statement),
            Statement::Console(ref statement) => write!(f, "{}", statement),
            Statement::Expression(ref statement) => write!(f, "{}", statement.expression),
        }
    }
}
