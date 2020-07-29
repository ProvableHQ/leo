use crate::{Identifier, Span, Type};
use leo_inputs::parameters::Parameter as AstParameter;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Parameter {
    pub variable: Identifier,
    pub type_: Type,
    pub span: Span,
}

impl<'ast> From<AstParameter<'ast>> for Parameter {
    fn from(parameter: AstParameter<'ast>) -> Self {
        Self {
            variable: Identifier::from(parameter.variable),
            type_: Type::from(parameter.type_),
            span: Span::from(parameter.span),
        }
    }
}
