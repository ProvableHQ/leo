use crate::{
    ast::Rule,
    macros::{AssertEq, FormattedMacro},
};

use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::statement_macro))]
pub enum MacroStatement<'ast> {
    AssertEq(AssertEq<'ast>),
    Formatted(FormattedMacro<'ast>),
}

impl<'ast> fmt::Display for MacroStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MacroStatement::AssertEq(ref assert) => write!(f, "{}", assert),
            MacroStatement::Formatted(ref formatted) => write!(f, "{}", formatted),
        }
    }
}
