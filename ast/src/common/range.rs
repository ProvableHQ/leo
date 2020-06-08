use crate::{ast::Rule, expressions::Expression};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::range))]
pub struct Range<'ast> {
    pub from: Option<FromExpression<'ast>>,
    pub to: Option<ToExpression<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::from_expression))]
pub struct FromExpression<'ast>(pub Expression<'ast>);

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::to_expression))]
pub struct ToExpression<'ast>(pub Expression<'ast>);
