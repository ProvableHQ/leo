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

//! Evaluates a macro in a compiled Leo program.

use crate::program::Program;
use leo_asg::{ConsoleFunction, ConsoleStatement};
use leo_errors::Result;
use snarkvm_ir::LogLevel;

impl<'a> Program<'a> {
    pub fn evaluate_console_function_call(&mut self, console: &ConsoleStatement<'a>) -> Result<()> {
        match &console.function {
            ConsoleFunction::Assert(expression) => {
                self.evaluate_console_assert(expression.get())?;
            }
            ConsoleFunction::Error(string) => {
                self.emit_log(LogLevel::Error, string)?;
            }
            ConsoleFunction::Log(string) => {
                self.emit_log(LogLevel::Info, string)?;
            }
        }

        Ok(())
    }
}
