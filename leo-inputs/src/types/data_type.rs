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
