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
use leo_grammar::console::{
    ConsoleAssert as GrammarConsoleAssert,
    ConsoleDebug as GrammarConsoleDebug,
    ConsoleError as GrammarConsoleError,
    ConsoleFunction as GrammarConsoleFunction,
    ConsoleLog as GrammarConsoleLog,
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

impl<'ast> From<GrammarConsoleFunction<'ast>> for ConsoleFunction {
    fn from(console_function: GrammarConsoleFunction<'ast>) -> Self {
        match console_function {
            GrammarConsoleFunction::Assert(assert) => ConsoleFunction::from(assert),
            GrammarConsoleFunction::Debug(debug) => ConsoleFunction::from(debug),
            GrammarConsoleFunction::Error(error) => ConsoleFunction::from(error),
            GrammarConsoleFunction::Log(log) => ConsoleFunction::from(log),
        }
    }
}

impl<'ast> From<GrammarConsoleAssert<'ast>> for ConsoleFunction {
    fn from(assert: GrammarConsoleAssert<'ast>) -> Self {
        ConsoleFunction::Assert(Expression::from(assert.expression))
    }
}

impl<'ast> From<GrammarConsoleDebug<'ast>> for ConsoleFunction {
    fn from(debug: GrammarConsoleDebug<'ast>) -> Self {
        ConsoleFunction::Debug(FormattedString::from(debug.string))
    }
}

impl<'ast> From<GrammarConsoleError<'ast>> for ConsoleFunction {
    fn from(error: GrammarConsoleError<'ast>) -> Self {
        ConsoleFunction::Error(FormattedString::from(error.string))
    }
}

impl<'ast> From<GrammarConsoleLog<'ast>> for ConsoleFunction {
    fn from(log: GrammarConsoleLog<'ast>) -> Self {
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
