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

use crate::{Expression, FormattedString};
use leo_ast::console::{
    ConsoleAssert as AstConsoleAssert,
    ConsoleDebug as AstConsoleDebug,
    ConsoleError as AstConsoleError,
    ConsoleFunction as AstConsoleFunction,
    ConsoleLog as AstConsoleLog,
};

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsoleFunction {
    Assert(Expression),
    Debug(FormattedString),
    Error(FormattedString),
    Log(FormattedString),
}

impl<'ast> From<AstConsoleFunction<'ast>> for ConsoleFunction {
    fn from(console_function: AstConsoleFunction<'ast>) -> Self {
        match console_function {
            AstConsoleFunction::Assert(assert) => ConsoleFunction::from(assert),
            AstConsoleFunction::Debug(debug) => ConsoleFunction::from(debug),
            AstConsoleFunction::Error(error) => ConsoleFunction::from(error),
            AstConsoleFunction::Log(log) => ConsoleFunction::from(log),
        }
    }
}

impl<'ast> From<AstConsoleAssert<'ast>> for ConsoleFunction {
    fn from(assert: AstConsoleAssert<'ast>) -> Self {
        ConsoleFunction::Assert(Expression::from(assert.expression))
    }
}

impl<'ast> From<AstConsoleDebug<'ast>> for ConsoleFunction {
    fn from(debug: AstConsoleDebug<'ast>) -> Self {
        ConsoleFunction::Debug(FormattedString::from(debug.string))
    }
}

impl<'ast> From<AstConsoleError<'ast>> for ConsoleFunction {
    fn from(error: AstConsoleError<'ast>) -> Self {
        ConsoleFunction::Error(FormattedString::from(error.string))
    }
}

impl<'ast> From<AstConsoleLog<'ast>> for ConsoleFunction {
    fn from(log: AstConsoleLog<'ast>) -> Self {
        ConsoleFunction::Log(FormattedString::from(log.string))
    }
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
