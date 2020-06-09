use crate::{ast::Rule, types::GroupType, values::NumberValue};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::value_group))]
pub struct GroupValue<'ast> {
    pub value: GroupRepresentation<'ast>,
    pub _type: GroupType,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for GroupValue<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::group_single_or_tuple))]
pub enum GroupRepresentation<'ast> {
    Single(NumberValue<'ast>),
    Tuple(GroupTuple<'ast>),
}

impl<'ast> fmt::Display for GroupRepresentation<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GroupRepresentation::Single(number) => write!(f, "{}", number),
            GroupRepresentation::Tuple(tuple) => write!(f, "{}", tuple),
        }
    }
}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::group_tuple))]
pub struct GroupTuple<'ast> {
    pub x: NumberValue<'ast>,
    pub y: NumberValue<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for GroupTuple<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
