// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use crate::{ConsoleArgs, Expression, Node};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A console logging function to invoke.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ConsoleFunction {
    /// A `console.assert(expr)` call to invoke,
    /// asserting that the expression evaluates to `true`.
    Assert(Expression),
    /// A `console.error(args)` call to invoke,
    /// resulting in an error at runtime.
    Error(ConsoleArgs),
    /// A `console.log(args)` call to invoke,
    /// resulting in a log message at runtime.
    Log(ConsoleArgs),
}

impl fmt::Display for ConsoleFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsoleFunction::Assert(assert) => write!(f, "assert({})", assert),
            ConsoleFunction::Error(error) => write!(f, "error{})", error),
            ConsoleFunction::Log(log) => write!(f, "log({})", log),
        }
    }
}

impl Node for ConsoleFunction {
    fn span(&self) -> &Span {
        match self {
            ConsoleFunction::Assert(assert) => assert.span(),
            ConsoleFunction::Error(formatted) | ConsoleFunction::Log(formatted) => &formatted.span,
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            ConsoleFunction::Assert(assert) => assert.set_span(span),
            ConsoleFunction::Error(formatted) | ConsoleFunction::Log(formatted) => formatted.set_span(span),
        }
    }
}
