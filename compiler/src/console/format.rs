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

//! Evaluates a formatted string in a compiled Leo program.

use crate::{errors::ConsoleError, program::Program};
use leo_asg::FormatString;
use leo_ast::FormatStringPart;
use snarkvm_ir::{Instruction, LogData, LogLevel, Value};

impl<'a> Program<'a> {
    pub fn emit_log(&mut self, log_level: LogLevel, formatted: &FormatString<'a>) -> Result<(), ConsoleError> {
        // Check that containers and parameters match
        let container_count = formatted
            .parts
            .iter()
            .filter(|x| matches!(x, FormatStringPart::Container))
            .count();
        if container_count != formatted.parameters.len() {
            return Err(ConsoleError::length(
                container_count,
                formatted.parameters.len(),
                &formatted.span,
            ));
        }

        let mut executed_containers = Vec::with_capacity(formatted.parameters.len());
        for parameter in formatted.parameters.iter() {
            executed_containers.push(self.enforce_expression(parameter.get())?);
        }

        let mut out = vec![];
        let mut parameters = executed_containers.into_iter();
        for part in formatted.parts.iter() {
            match part {
                FormatStringPart::Const(c) => out.push(Value::Str(c.to_string())),
                FormatStringPart::Container => out.push(parameters.next().unwrap()),
            }
        }
        self.emit(Instruction::Log(LogData { log_level, parts: out }));
        Ok(())
    }
}
