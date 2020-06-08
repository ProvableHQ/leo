use crate::{Identifier, Type};
use leo_ast::{common::{Visibility, Private}, functions::FunctionInput as AstFunctionInput};

use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct FunctionInput {
    pub identifier: Identifier,
    pub mutable: bool,
    pub private: bool,
    pub _type: Type,
}

impl<'ast> From<AstFunctionInput<'ast>> for FunctionInput {
    fn from(parameter: AstFunctionInput<'ast>) -> Self {
        FunctionInput {
            identifier: Identifier::from(parameter.identifier),
            mutable: parameter.mutable.is_some(),
            // private by default
            private: parameter.visibility.map_or(true, |visibility| {
                visibility.eq(&Visibility::Private(Private {}))
            }),
            _type: Type::from(parameter._type),
        }
    }
}

impl fmt::Display for FunctionInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // mut var: private bool
        if self.mutable {
            write!(f, "mut ")?;
        }
        write!(f, "{}: ", self.identifier)?;
        if self.private {
            write!(f, "private ")?;
        } else {
            write!(f, "public ")?;
        }
        write!(f, "{}", self._type)
    }
}
