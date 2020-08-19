use crate::{ConsoleFunction, Span};
use leo_ast::console::ConsoleFunctionCall as AstConsoleFunctionCall;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsoleFunctionCall {
    pub function: ConsoleFunction,
    pub span: Span,
}

impl<'ast> From<AstConsoleFunctionCall<'ast>> for ConsoleFunctionCall {
    fn from(console: AstConsoleFunctionCall<'ast>) -> Self {
        ConsoleFunctionCall {
            function: ConsoleFunction::from(console.function),
            span: Span::from(console.span),
        }
    }
}

impl fmt::Display for ConsoleFunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "console.{};", self.function)
    }
}
