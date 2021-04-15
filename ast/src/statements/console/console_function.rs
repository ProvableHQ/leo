// Copyright (C) 2019-2021 Aleo Systems Inc.
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

use crate::{Expression, FormatString, Node, Span};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum ConsoleFunction {
    Assert(Expression),
    Debug(FormatString),
    Error(FormatString),
    Log(FormatString),
}

impl fmt::Display for ConsoleFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsoleFunction::Assert(assert) => write!(f, "assert({})", assert),
            ConsoleFunction::Debug(debug) => write!(f, "debug({})", debug),
            ConsoleFunction::Error(error) => write!(f, "error{})", error),
            ConsoleFunction::Log(log) => write!(f, "log({})", log),
        }
    }
}

impl Node for ConsoleFunction {
    fn span(&self) -> &Span {
        match self {
            ConsoleFunction::Assert(assert) => assert.span(),
            ConsoleFunction::Debug(formatted) | ConsoleFunction::Error(formatted) | ConsoleFunction::Log(formatted) => {
                &formatted.span
            }
        }
    }

    fn set_span(&mut self, span: Span) {
        match self {
            ConsoleFunction::Assert(assert) => assert.set_span(span),
            ConsoleFunction::Debug(formatted) | ConsoleFunction::Error(formatted) | ConsoleFunction::Log(formatted) => {
                formatted.set_span(span)
            }
        }
    }
}
