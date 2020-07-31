use crate::{access::AssigneeAccess, ast::Rule, common::Identifier, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::assignee))]
pub struct Assignee<'ast> {
    pub identifier: Identifier<'ast>,
    pub accesses: Vec<AssigneeAccess<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for Assignee<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)?;
        for (i, access) in self.accesses.iter().enumerate() {
            write!(f, "{}", access)?;
            if i < self.accesses.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "")
    }
}
