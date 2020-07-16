use crate::ast::Rule;

use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_integer))]
pub enum IntegerType {
    U8Type(U8Type),
    U16Type(U16Type),
    U32Type(U32Type),
    U64Type(U64Type),
    U128Type(U128Type),

    I8Type(I8Type),
    I16Type(I16Type),
    I32Type(I32Type),
    I64Type(I64Type),
    I128Type(I128Type),
}

// Unsigned
#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u8))]
pub struct U8Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u16))]
pub struct U16Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u32))]
pub struct U32Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u64))]
pub struct U64Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_u128))]
pub struct U128Type {}

// Singed

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_i8))]
pub struct I8Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_i16))]
pub struct I16Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_i32))]
pub struct I32Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_i64))]
pub struct I64Type {}

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_i128))]
pub struct I128Type {}
