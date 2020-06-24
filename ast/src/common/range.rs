use crate::{ast::Rule, values::NumberValue};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::range))]
pub struct Range<'ast> {
    pub from: Option<NumberValue<'ast>>,
    pub to: Option<NumberValue<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
