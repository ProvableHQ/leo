use crate::{ast::Rule, functions::Function};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::test_function))]
pub struct TestFunction<'ast> {
    pub function: Function<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
