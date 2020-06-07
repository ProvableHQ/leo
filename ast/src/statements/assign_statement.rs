use crate::{ast::{Rule, LineEnd, Assignee}, expressions::Expression, operations::AssignOperation};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement_assign))]
pub struct AssignStatement<'ast> {
    pub assignee: Assignee<'ast>,
    pub assign: AssignOperation,
    pub expression: Expression<'ast>,
    pub line_end: LineEnd,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for AssignStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {};", self.assignee, self.expression)
    }
}
