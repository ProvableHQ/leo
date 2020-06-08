use crate::{access::*, ast::Rule};

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::access))]
pub enum Access<'ast> {
    Array(ArrayAccess<'ast>),
    Call(CallAccess<'ast>),
    Object(MemberAccess<'ast>),
    StaticObject(StaticMemberAccess<'ast>),
}
