use crate::{ConditionalStatement, Statement};
use leo_ast::statements::ConditionalNestedOrEndStatement as AstConditionalNestedOrEndStatement;

use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub enum ConditionalNestedOrEndStatement {
    Nested(Box<ConditionalStatement>),
    End(Vec<Statement>),
}

impl<'ast> From<AstConditionalNestedOrEndStatement<'ast>> for ConditionalNestedOrEndStatement {
    fn from(statement: AstConditionalNestedOrEndStatement<'ast>) -> Self {
        match statement {
            AstConditionalNestedOrEndStatement::Nested(nested) => ConditionalNestedOrEndStatement::Nested(
                Box::new(ConditionalStatement::from(*nested)),
            ),
            AstConditionalNestedOrEndStatement::End(statements) => ConditionalNestedOrEndStatement::End(
                statements
                    .into_iter()
                    .map(|statement| Statement::from(statement))
                    .collect(),
            ),
        }
    }
}

impl fmt::Display for ConditionalNestedOrEndStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConditionalNestedOrEndStatement::Nested(ref nested) => write!(f, "else {}", nested),
            ConditionalNestedOrEndStatement::End(ref statements) => {
                write!(f, "else {{\n")?;
                for statement in statements.iter() {
                    write!(f, "\t\t{}\n", statement)?;
                }
                write!(f, "\t}}")
            }
        }
    }
}
