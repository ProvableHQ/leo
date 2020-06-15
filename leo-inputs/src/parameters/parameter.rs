use crate::{
    ast::Rule,
    common::{Identifier, Visibility},
    types::Type,
};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::parameter))]
pub struct Parameter<'ast> {
    pub variable: Identifier<'ast>,
    pub visibility: Option<Visibility>,
    pub type_: Type<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
