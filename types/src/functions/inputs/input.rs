use crate::FunctionInput;
use leo_ast::functions::inputs::Input as AstInput;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Input {
    FunctionInput(FunctionInput),
    Record,
    Registers,
    State,
    StateLeaf,
}

impl<'ast> From<AstInput<'ast>> for Input {
    fn from(input: AstInput<'ast>) -> Self {
        match input {
            AstInput::FunctionInput(function_input) => Input::FunctionInput(FunctionInput::from(function_input)),
            AstInput::Record(_) => Input::Record,
            AstInput::Registers(_) => Input::Registers,
            AstInput::State(_) => Input::State,
            AstInput::StateLeaf(_) => Input::StateLeaf,
        }
    }
}

impl Input {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Input::FunctionInput(function_input) => write!(f, "{}", function_input),
            Input::Record => write!(f, "record"),
            Input::Registers => write!(f, "registers"),
            Input::State => write!(f, "state"),
            Input::StateLeaf => write!(f, "state_leaf"),
        }
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
