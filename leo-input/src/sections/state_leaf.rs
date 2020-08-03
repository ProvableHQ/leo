use crate::ast::Rule;

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::state_leaf))]
pub struct StateLeaf<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
