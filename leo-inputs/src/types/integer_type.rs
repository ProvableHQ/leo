use crate::ast::Rule;

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_integer))]
pub enum IntegerType<'ast> {
    U8Type(U8Type<'ast>),
    U16Type(U16Type<'ast>),
    U32Type(U32Type<'ast>),
    U64Type(U64Type<'ast>),
    U128Type(U128Type<'ast>),
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_u8))]
pub struct U8Type<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_u16))]
pub struct U16Type<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_u32))]
pub struct U32Type<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_u64))]
pub struct U64Type<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_u128))]
pub struct U128Type<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> IntegerType<'ast> {
    pub fn span(&self) -> &Span<'ast> {
        match self {
            IntegerType::U8Type(type_) => &type_.span,
            IntegerType::U16Type(type_) => &type_.span,
            IntegerType::U32Type(type_) => &type_.span,
            IntegerType::U64Type(type_) => &type_.span,
            IntegerType::U128Type(type_) => &type_.span,
        }
    }
}

impl<'ast> std::fmt::Display for IntegerType<'ast> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IntegerType::U8Type(_) => write!(f, "u8"),
            IntegerType::U16Type(_) => write!(f, "u16"),
            IntegerType::U32Type(_) => write!(f, "u32"),
            IntegerType::U64Type(_) => write!(f, "u64"),
            IntegerType::U128Type(_) => write!(f, "u128"),
        }
    }
}
