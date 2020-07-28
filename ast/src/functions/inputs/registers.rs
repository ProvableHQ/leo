use crate::ast::Rule;

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::registers))]
pub struct Registers<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
