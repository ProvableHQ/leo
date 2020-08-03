use crate::{FunctionInput, Identifier, Span};
use leo_ast::functions::input::Input as AstInput;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputVariable {
    InputKeyword(Identifier),
    FunctionInput(FunctionInput),
}

impl<'ast> From<AstInput<'ast>> for InputVariable {
    fn from(input: AstInput<'ast>) -> Self {
        match input {
            AstInput::InputKeyword(input_keyword) => {
                let id = Identifier {
                    name: input_keyword.keyword,
                    span: Span::from(input_keyword.span),
                };

                InputVariable::InputKeyword(id)
            }
            AstInput::FunctionInput(function_input) => {
                InputVariable::FunctionInput(FunctionInput::from(function_input))
            }
        }
    }
}

impl InputVariable {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputVariable::InputKeyword(id) => write!(f, "{}", id),
            InputVariable::FunctionInput(function_input) => write!(f, "{}", function_input),
        }
    }
}

impl fmt::Display for InputVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for InputVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
