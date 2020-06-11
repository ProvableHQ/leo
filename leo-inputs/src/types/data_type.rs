use crate::{
    ast::Rule,
    types::{BooleanType, FieldType, GroupType, IntegerType},
};

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_data))]
pub enum DataType {
    Integer(IntegerType),
    Field(FieldType),
    Group(GroupType),
    Boolean(BooleanType),
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DataType::Integer(ref integer) => write!(f, "{}", integer),
            DataType::Field(_) => write!(f, "field"),
            DataType::Group(_) => write!(f, "group"),
            DataType::Boolean(_) => write!(f, "bool"),
        }
    }
}
