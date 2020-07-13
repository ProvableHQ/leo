use leo_ast::macros::Debug as AstDebug;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Debug {}

impl<'ast> From<AstDebug<'ast>> for Debug {
    fn from(_debug: AstDebug<'ast>) -> Self {
        Self {}
    }
}

impl fmt::Display for Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "debug")
    }
}
