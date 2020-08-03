use crate::{FunctionInput, Identifier, Span};
use leo_ast::functions::inputs::Input as AstInput;

use serde::{Deserialize, Serialize};
use std::fmt;

pub const RECORD_VARIABLE_NAME: &str = "record";
pub const REGISTERS_VARIABLE_NAME: &str = "registers";
pub const STATE_VARIABLE_NAME: &str = "state";
pub const STATE_LEAF_VARIABLE_NAME: &str = "state_leaf";

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Input {
    FunctionInput(FunctionInput),
    Record(Identifier),
    Registers(Identifier),
    State(Identifier),
    StateLeaf(Identifier),
}

impl<'ast> From<AstInput<'ast>> for Input {
    fn from(input: AstInput<'ast>) -> Self {
        match input {
            AstInput::FunctionInput(function_input) => Input::FunctionInput(FunctionInput::from(function_input)),
            AstInput::Record(record) => {
                let id = Identifier {
                    name: RECORD_VARIABLE_NAME.to_string(),
                    span: Span::from(record.span),
                };

                Input::Record(id)
            }
            AstInput::Registers(registers) => {
                let id = Identifier {
                    name: REGISTERS_VARIABLE_NAME.to_string(),
                    span: Span::from(registers.span),
                };

                Input::Registers(id)
            }
            AstInput::State(state) => {
                let id = Identifier {
                    name: STATE_VARIABLE_NAME.to_string(),
                    span: Span::from(state.span),
                };

                Input::State(id)
            }
            AstInput::StateLeaf(state_leaf) => {
                let id = Identifier {
                    name: STATE_LEAF_VARIABLE_NAME.to_string(),
                    span: Span::from(state_leaf.span),
                };

                Input::StateLeaf(id)
            }
        }
    }
}

impl Input {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Input::FunctionInput(function_input) => write!(f, "{}", function_input),
            Input::Record(id) => write!(f, "{}", id),
            Input::Registers(id) => write!(f, "{}", id),
            Input::State(id) => write!(f, "{}", id),
            Input::StateLeaf(id) => write!(f, "{}", id),
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
