use crate::{
    ast::Rule,
    console::{ConsoleAssert, ConsoleDebug, ConsoleError, ConsoleLog},
};

use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::console_function))]
pub enum ConsoleFunction<'ast> {
    Assert(ConsoleAssert<'ast>),
    Debug(ConsoleDebug<'ast>),
    Error(ConsoleError<'ast>),
    Log(ConsoleLog<'ast>),
}

impl<'ast> fmt::Display for ConsoleFunction<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsoleFunction::Assert(assert) => write!(f, "{}", assert),
            ConsoleFunction::Debug(debug) => write!(f, "{}", debug),
            ConsoleFunction::Error(error) => write!(f, "{}", error),
            ConsoleFunction::Log(log) => write!(f, "{}", log),
        }
    }
}
