use crate::{
    ast::Rule,
    common::{Declare, LineEnd, Variable},
    expressions::Expression,
    SpanDef,
};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::statement_definition))]
pub struct DefinitionStatement<'ast> {
    pub declare: Declare,
    pub variable: Variable<'ast>,
    pub expression: Expression<'ast>,
    pub line_end: LineEnd,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for DefinitionStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "let {} = {};", self.variable, self.expression)
    }
}
