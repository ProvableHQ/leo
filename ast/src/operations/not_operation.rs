use crate::ast::Rule;

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::operation_not))]
pub struct NotOperation<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
