use crate::{ast::Rule, values::NumberValue};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::group_coordinate))]
pub enum GroupCoordinate<'ast> {
    Number(NumberValue<'ast>),
    SignHigh(SignHigh<'ast>),
    SignLow(SignLow<'ast>),
    Inferred(Inferred<'ast>),
}

impl<'ast> GroupCoordinate<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            GroupCoordinate::Number(number) => &number.span(),
            GroupCoordinate::SignHigh(sign_high) => &sign_high.span,
            GroupCoordinate::SignLow(sign_low) => &sign_low.span,
            GroupCoordinate::Inferred(inferred) => &inferred.span,
        }
    }
}

impl<'ast> fmt::Display for GroupCoordinate<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GroupCoordinate::Number(number) => write!(f, "{}", number),
            GroupCoordinate::SignHigh(_) => write!(f, "+"),
            GroupCoordinate::SignLow(_) => write!(f, "-"),
            GroupCoordinate::Inferred(_) => write!(f, "_"),
        }
    }
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::sign_high))]
pub struct SignHigh<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::sign_low))]
pub struct SignLow<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::inferred))]
pub struct Inferred<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
