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

use crate::ast::Rule;
use crate::console::ConsoleAssert;
use crate::console::ConsoleDebug;
use crate::console::ConsoleError;
use crate::console::ConsoleLog;

use pest_ast::FromPest;
use serde::Serialize;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq, Serialize)]
#[pest_ast(rule(Rule::console_function))]
pub enum ConsoleFunction<'ast> {
    Assert(ConsoleAssert<'ast>),
    Debug(ConsoleDebug<'ast>),
    Error(ConsoleError<'ast>),
    Log(ConsoleLog<'ast>),
}

impl<'ast> fmt::Display for ConsoleFunction<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsoleFunction::Assert(assert) => write!(f, "{}", assert),
            ConsoleFunction::Debug(debug) => write!(f, "{}", debug),
            ConsoleFunction::Error(error) => write!(f, "{}", error),
            ConsoleFunction::Log(log) => write!(f, "{}", log),
        }
    }
}
