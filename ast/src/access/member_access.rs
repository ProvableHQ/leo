use crate::{ast::Rule, common::Identifier};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::access_member))]
pub struct MemberAccess<'ast> {
    pub identifier: Identifier<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
