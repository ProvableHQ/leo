use crate::{
    ast::Rule,
    types::{BooleanType, FieldType, GroupType, IntegerType},
};

use crate::types::AddressType;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_data))]
pub enum DataType {
    Address(AddressType),
    Boolean(BooleanType),
    Field(FieldType),
    Group(GroupType),
    Integer(IntegerType),
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DataType::Address(_) => write!(f, "address"),
            DataType::Boolean(_) => write!(f, "bool"),
            DataType::Field(_) => write!(f, "value.field"),
            DataType::Group(_) => write!(f, "group"),
            DataType::Integer(ref integer) => write!(f, "{}", integer),
        }
    }
}
