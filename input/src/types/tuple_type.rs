use crate::{ast::Rule, types::Type};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq, Eq)]
#[pest_ast(rule(Rule::type_tuple))]
pub struct TupleType<'ast> {
    pub types_: Vec<Type<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> std::fmt::Display for TupleType<'ast> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let tuple = self
            .types_
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "({})", tuple)
    }
}
