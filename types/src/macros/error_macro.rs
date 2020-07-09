use leo_ast::macros::Error as AstError;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorMacro {}

impl<'ast> From<AstError<'ast>> for ErrorMacro {
    fn from(_error: AstError<'ast>) -> Self {
        Self {}
    }
}

impl fmt::Display for ErrorMacro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error")
    }
}
