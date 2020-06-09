use crate::{ConditionalNestedOrEndStatement, Expression, Statement};
use leo_ast::statements::ConditionalStatement as AstConditionalStatement;

use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct ConditionalStatement {
    pub condition: Expression,
    pub statements: Vec<Statement>,
    pub next: Option<ConditionalNestedOrEndStatement>,
}

impl<'ast> From<AstConditionalStatement<'ast>> for ConditionalStatement {
    fn from(statement: AstConditionalStatement<'ast>) -> Self {
        ConditionalStatement {
            condition: Expression::from(statement.condition),
            statements: statement
                .statements
                .into_iter()
                .map(|statement| Statement::from(statement))
                .collect(),
            next: statement
                .next
                .map(|n_or_e| Some(ConditionalNestedOrEndStatement::from(n_or_e)))
                .unwrap_or(None),
        }
    }
}

impl fmt::Display for ConditionalStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "if ({}) {{\n", self.condition)?;
        for statement in self.statements.iter() {
            write!(f, "\t\t{}\n", statement)?;
        }
        match self.next.clone() {
            Some(n_or_e) => write!(f, "\t}} {}", n_or_e),
            None => write!(f, "\t}}"),
        }
    }
}
