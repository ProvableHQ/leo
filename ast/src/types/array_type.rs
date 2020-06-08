use crate::{ast::Rule, types::DataType, values::Value};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::type_array))]
pub struct ArrayType<'ast> {
    pub _type: DataType,
    pub dimensions: Vec<Value<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
