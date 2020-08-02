use crate::{Expression, Span};
use leo_ast::macros::FormattedParameter as AstFormattedParameter;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormattedParameter {
    pub expression: Expression,
    pub span: Span,
}

impl<'ast> From<AstFormattedParameter<'ast>> for FormattedParameter {
    fn from(parameter: AstFormattedParameter<'ast>) -> Self {
        Self {
            expression: Expression::from(parameter.expression),
            span: Span::from(parameter.span),
        }
    }
}

impl fmt::Display for FormattedParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}
