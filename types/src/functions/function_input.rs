use crate::{Identifier, Span, Type};
use leo_ast::functions::FunctionInput as AstFunctionInput;

use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct FunctionInput {
    pub identifier: Identifier,
    pub mutable: bool,
    pub _type: Type,
    pub span: Span,
}

impl<'ast> From<AstFunctionInput<'ast>> for FunctionInput {
    fn from(parameter: AstFunctionInput<'ast>) -> Self {
        FunctionInput {
            identifier: Identifier::from(parameter.identifier),
            mutable: parameter.mutable.is_some(),
            _type: Type::from(parameter._type),
            span: Span::from(parameter.span),
        }
    }
}

impl FunctionInput {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // mut var: bool
        if self.mutable {
            write!(f, "mut ")?;
        }
        write!(f, "{}: ", self.identifier)?;
        write!(f, "{}", self._type)
    }
}

impl fmt::Display for FunctionInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for FunctionInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
