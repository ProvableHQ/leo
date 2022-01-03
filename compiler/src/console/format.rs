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

//! Evaluates a formatted string in a compiled Leo program.

use crate::program::Program;
use leo_asg::{CharValue, ConsoleArgs};
use leo_errors::CompilerError;
use leo_errors::Result;
use snarkvm_ir::{Instruction, LogData, LogLevel, Value};

impl<'a> Program<'a> {
    pub fn emit_log(&mut self, log_level: LogLevel, args: &ConsoleArgs<'a>) -> Result<()> {
        let mut out = Vec::new();
        let mut in_container = false;
        let mut substring = String::new();
        let mut arg_index = 0;
        let mut escape_right_bracket = false;
        for (index, character) in args.string.iter().enumerate() {
            match character {
                _ if escape_right_bracket => {
                    escape_right_bracket = false;
                    continue;
                }
                CharValue::Scalar(scalar) => match scalar {
                    '{' if !in_container => {
                        out.push(Value::Str(substring.clone()));
                        substring.clear();
                        in_container = true;
                    }
                    '{' if in_container => {
                        substring.push('{');
                        in_container = false;
                    }
                    '}' if in_container => {
                        in_container = false;
                        let parameter = match args.parameters.get(arg_index) {
                            Some(index) => index,
                            None => {
                                return Err(CompilerError::console_container_parameter_length_mismatch(
                                    arg_index + 1,
                                    args.parameters.len(),
                                    &args.span,
                                )
                                .into());
                            }
                        };
                        out.push(self.enforce_expression(parameter.get())?);
                        arg_index += 1;
                    }
                    '}' if !in_container => {
                        if let Some(CharValue::Scalar(next)) = args.string.get(index + 1) {
                            if *next == '}' {
                                substring.push('}');
                                escape_right_bracket = true;
                            } else {
                                return Err(CompilerError::console_fmt_expected_escaped_right_brace(&args.span).into());
                            }
                        }
                    }
                    _ if in_container => {
                        return Err(CompilerError::console_fmt_expected_left_or_right_brace(&args.span).into());
                    }
                    _ => substring.push(*scalar),
                },
                CharValue::NonScalar(non_scalar) => {
                    substring.push_str(format!("\\u{{{:x}}}", non_scalar).as_str());
                    in_container = false;
                }
            }
        }
        out.push(Value::Str(substring));

        // Check that containers and parameters match
        if arg_index != args.parameters.len() {
            return Err(CompilerError::console_container_parameter_length_mismatch(
                arg_index,
                args.parameters.len(),
                &args.span,
            )
            .into());
        }

        self.emit(Instruction::Log(LogData { log_level, parts: out }));
        Ok(())
    }
}
