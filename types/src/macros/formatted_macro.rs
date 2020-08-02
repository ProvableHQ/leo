use crate::{FormattedString, MacroName, Span};
use leo_ast::macros::FormattedMacro as AstFormattedMacro;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormattedMacro {
    pub name: MacroName,
    pub string: Option<FormattedString>,
    pub span: Span,
}

impl<'ast> From<AstFormattedMacro<'ast>> for FormattedMacro {
    fn from(formatted: AstFormattedMacro<'ast>) -> Self {
        Self {
            name: MacroName::from(formatted.name),
            string: formatted.string.map(|string| FormattedString::from(string)),
            span: Span::from(formatted.span),
        }
    }
}

impl fmt::Display for FormattedMacro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}!({});",
            self.name,
            self.string.as_ref().map(|s| s.to_string()).unwrap_or("".to_string()),
        )
    }
}
