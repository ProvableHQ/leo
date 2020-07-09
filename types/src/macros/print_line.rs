use leo_ast::macros::PrintLine as AstPrintLine;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrintLine {}

impl<'ast> From<AstPrintLine<'ast>> for PrintLine {
    fn from(_print_line: AstPrintLine<'ast>) -> Self {
        Self {}
    }
}

impl fmt::Display for PrintLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "println")
    }
}
