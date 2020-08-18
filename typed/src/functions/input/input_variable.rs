// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

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
