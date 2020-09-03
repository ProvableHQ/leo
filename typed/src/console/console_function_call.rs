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

use crate::{ConsoleFunction, Span};
use leo_ast::console::ConsoleFunctionCall as AstConsoleFunctionCall;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsoleFunctionCall {
    pub function: ConsoleFunction,
    pub span: Span,
}

impl<'ast> From<AstConsoleFunctionCall<'ast>> for ConsoleFunctionCall {
    fn from(console: AstConsoleFunctionCall<'ast>) -> Self {
        ConsoleFunctionCall {
            function: ConsoleFunction::from(console.function),
            span: Span::from(console.span),
        }
    }
}

impl fmt::Display for ConsoleFunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "console.{};", self.function)
    }
}
