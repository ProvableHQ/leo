use crate::{ast::Rule, common::Identifier, expressions::Expression, statements::Statement, SpanDef};

use pest::Span;
use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::statement_for))]
pub struct ForStatement<'ast> {
    pub index: Identifier<'ast>,
    pub start: Expression<'ast>,
    pub stop: Expression<'ast>,
    pub statements: Vec<Statement<'ast>>,
    #[pest_ast(outer())]
    #[serde(with = "SpanDef")]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for ForStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "for {} in {}..{} {{ {:#?} }}",
            self.index, self.start, self.stop, self.statements
        )
    }
}
