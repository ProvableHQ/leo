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

use std::fmt;

use backtrace::Backtrace;
use color_backtrace::{BacktracePrinter, Verbosity};
use derivative::Derivative;

pub const INDENT: &str = "    ";

/// Formatted compiler error type
///     undefined value `x`
///     --> file.leo: 2:8
///      |
///    2 | let a = x;
///      |         ^
///      |
///      = help: Initialize a variable `x` first.
#[derive(Derivative)]
#[derivative(Clone, Debug, Default, Hash, PartialEq)]
pub struct BacktracedError {
    pub message: String,
    pub help: Option<String>,
    pub exit_code: i32,
    pub code_identifier: i8,
    pub error_type: String,
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    pub backtrace: Backtrace,
}

impl BacktracedError {
    pub fn new_from_backtrace<S>(
        message: S,
        help: Option<String>,
        exit_code: i32,
        code_identifier: i8,
        error_type: String,
        backtrace: Backtrace,
    ) -> Self
    where
        S: ToString,
    {
        Self {
            message: message.to_string(),
            help,
            exit_code,
            code_identifier,
            error_type,
            backtrace,
        }
    }

    pub fn exit_code(&self) -> i32 {
        let mut code: i32;
        if self.code_identifier > 99 {
            code = self.code_identifier as i32 * 111_000;
        } else if self.code_identifier as i32 > 9 {
            code = self.code_identifier as i32 * 11_000;
        } else {
            code = self.code_identifier as i32 * 1_000;
        }
        code += self.exit_code;

        code
    }
}

impl fmt::Display for BacktracedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_message = format!(
            "[E{error_type}{code_identifier:0>3}{exit_code:0>4}]: {message}\
	  {indent     } ",
            indent = INDENT,
            error_type = self.error_type,
            code_identifier = self.code_identifier,
            exit_code = self.exit_code,
            message = self.message,
        );

        write!(f, "{}", error_message)?;

        if let Some(help) = &self.help {
            write!(f, "{indent     } = {help}", indent = INDENT, help = help)?;
        }

        let leo_backtrace = std::env::var("LEO_BACKTRACE").unwrap_or_default().trim().to_owned();
        match leo_backtrace.as_ref() {
            "1" => {
                let mut printer = BacktracePrinter::default();
                printer = printer.lib_verbosity(Verbosity::Medium);
                let trace = printer
                    .format_trace_to_string(&self.backtrace)
                    .map_err(|_| fmt::Error)?;
                write!(f, "{}", trace)?;
            }
            "full" => {
                let mut printer = BacktracePrinter::default();
                printer = printer.lib_verbosity(Verbosity::Full);
                let trace = printer
                    .format_trace_to_string(&self.backtrace)
                    .map_err(|_| fmt::Error)?;
                write!(f, "{}", trace)?;
            }
            _ => {}
        }

        Ok(())
    }
}

impl std::error::Error for BacktracedError {
    fn description(&self) -> &str {
        &self.message
    }
}
