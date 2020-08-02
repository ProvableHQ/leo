use leo_ast::macros::Print as AstPrint;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Print {}

impl<'ast> From<AstPrint<'ast>> for Print {
    fn from(_print: AstPrint<'ast>) -> Self {
        Self {}
    }
}

impl fmt::Display for Print {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "print")
    }
}
