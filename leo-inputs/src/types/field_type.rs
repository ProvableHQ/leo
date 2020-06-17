use crate::ast::Rule;

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_field))]
pub struct FieldType<'ast> {
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
