use crate::{ast::Rule, expressions::Expression, statements::{ConditionalNestedOrEndStatement, Statement}};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement_conditional))]
pub struct ConditionalStatement<'ast> {
    pub condition: Expression<'ast>,
    pub statements: Vec<Statement<'ast>>,
    pub next: Option<ConditionalNestedOrEndStatement<'ast>>,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for ConditionalStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "if ({}) {{\n", self.condition)?;
        write!(f, "\t{:#?}\n", self.statements)?;
        self.next
            .as_ref()
            .map(|n_or_e| write!(f, "}} {}", n_or_e))
            .unwrap_or(write!(f, "}}"))
    }
}
