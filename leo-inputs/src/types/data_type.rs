use crate::{
    ast::Rule,
    types::{BooleanType, FieldType, GroupType, IntegerType},
};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_data))]
pub enum DataType<'ast> {
    Integer(IntegerType<'ast>),
    Field(FieldType<'ast>),
    Group(GroupType<'ast>),
    Boolean(BooleanType<'ast>),
}

impl<'ast> DataType<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            DataType::Boolean(type_) => &type_.span,
            DataType::Integer(type_) => type_.span(),
            DataType::Field(type_) => &type_.span,
            DataType::Group(type_) => &type_.span,
        }
    }
}

impl<'ast> std::fmt::Display for DataType<'ast> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DataType::Integer(ref integer) => write!(f, "{}", integer),
            DataType::Field(_) => write!(f, "field"),
            DataType::Group(_) => write!(f, "group"),
            DataType::Boolean(_) => write!(f, "bool"),
        }
    }
}
