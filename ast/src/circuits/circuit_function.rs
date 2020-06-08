use crate::{ast::Rule, common::Static, function::Function};

use pest::Span;
use pest_ast::FromPest;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::circuit_function))]
pub struct CircuitFunction<'ast> {
    pub _static: Option<Static>,
    pub function: Function<'ast>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}
