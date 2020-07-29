use crate::{
    access::{ArrayAccess, MemberAccess},
    ast::Rule,
};

use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::access_assignee))]
pub enum AssigneeAccess<'ast> {
    Array(ArrayAccess<'ast>),
    Member(MemberAccess<'ast>),
}

impl<'ast> fmt::Display for AssigneeAccess<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssigneeAccess::Array(ref array) => write!(f, "[{}]", array.expression),
            AssigneeAccess::Member(ref member) => write!(f, ".{}", member.identifier),
        }
    }
}
