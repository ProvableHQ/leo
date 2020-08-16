use crate::{FormattedContainer, FormattedParameter, Span};
use leo_ast::console::FormattedString as AstFormattedString;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormattedString {
    pub string: String,
    pub containers: Vec<FormattedContainer>,
    pub parameters: Vec<FormattedParameter>,
    pub span: Span,
}

impl<'ast> From<AstFormattedString<'ast>> for FormattedString {
    fn from(formatted: AstFormattedString<'ast>) -> Self {
        let string = formatted.string;
        let span = Span::from(formatted.span);
        let containers = formatted
            .containers
            .into_iter()
            .map(|container| FormattedContainer::from(container))
            .collect();
        let parameters = formatted
            .parameters
            .into_iter()
            .map(|parameter| FormattedParameter::from(parameter))
            .collect();

        Self {
            string,
            containers,
            parameters,
            span,
        }
    }
}

impl fmt::Display for FormattedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}
