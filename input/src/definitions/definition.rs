use crate::{ast::Rule, common::LineEnd, expressions::Expression, parameters::Parameter};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::definition))]
pub struct Definition<'ast> {
    pub parameter: Parameter<'ast>,
    pub expression: Expression<'ast>,
    pub line_end: LineEnd,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
