use crate::{
    ast::Rule,
    statements::{ConditionalStatement, Statement},
};

use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::conditional_nested_or_end_statement))]
pub enum ConditionalNestedOrEndStatement<'ast> {
    Nested(Box<ConditionalStatement<'ast>>),
    End(Vec<Statement<'ast>>),
}

impl<'ast> fmt::Display for ConditionalNestedOrEndStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConditionalNestedOrEndStatement::Nested(ref nested) => write!(f, "else {}", nested),
            ConditionalNestedOrEndStatement::End(ref statements) => write!(f, "else {{\n \t{:#?}\n }}", statements),
        }
    }
}
