use leo_ast::common::Declare as AstDeclare;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Declare {
    Const,
    Let,
}

impl<'ast> From<AstDeclare> for Declare {
    fn from(declare: AstDeclare) -> Self {
        match declare {
            AstDeclare::Const(_) => Declare::Const,
            AstDeclare::Let(_) => Declare::Let,
        }
    }
}

impl fmt::Display for Declare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Declare::Const => write!(f, "const"),
            Declare::Let => write!(f, "let"),
        }
    }
}
