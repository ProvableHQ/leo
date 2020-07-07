use crate::{
    ast::Rule,
    types::{AddressType, BooleanType, FieldType, GroupType, IntegerType},
};

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_data))]
pub enum DataType {
    Address(AddressType),
    Boolean(BooleanType),
    Field(FieldType),
    Group(GroupType),
    Integer(IntegerType),
}
