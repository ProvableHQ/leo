// Copyright (C) 2019-2023 Aleo Systems Inc.
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
use colored::Colorize;
use derivative::Derivative;
use leo_span::source_map::is_not_test_framework;

/// The indent for an error message.
pub(crate) const INDENT: &str = "    ";

/// Backtraced compiler ouput type
///     undefined value `x`
///     --> file.leo: 2:8
///      = help: Initialize a variable `x` first.
#[derive(Derivative)]
#[derivative(Clone, Debug, Default, Hash, PartialEq)]
pub struct Backtraced {
    /// The error message.
    pub message: String,
    /// The error help message if it exists.
    pub help: Option<String>,
    /// The error exit code.
    pub code: i32,
    /// The error leading digits identifier.
    pub code_identifier: i8,
    /// The characters representing the type of error.
    pub type_: String,
    /// Is this Backtrace a warning or error?
    pub error: bool,
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    /// The backtrace representing where the error occured in Leo.
    pub backtrace: Backtrace,
}

impl Backtraced {
    /// Creates a backtraced error from a backtrace.
    pub fn new_from_backtrace<S>(
        message: S,
        help: Option<String>,
        code: i32,
        code_identifier: i8,
        type_: String,
        error: bool,
        backtrace: Backtrace,
    ) -> Self
    where
        S: ToString,
    {
        Self { message: message.to_string(), help, code, code_identifier, type_, error, backtrace }
    }

    /// Gets the backtraced error exit code.
    pub fn exit_code(&self) -> i32 {
        let mut code: i32;
        if self.code_identifier > 99 {
            code = self.code_identifier as i32 * 100_000;
        } else if self.code_identifier as i32 > 9 {
            code = self.code_identifier as i32 * 10_000;
        } else {
            code = self.code_identifier as i32 * 1_000;
        }
        code += self.code;

        code
    }

    /// Gets a unique error identifier.
    pub fn error_code(&self) -> String {
        format!(
            "E{error_type}{code_identifier:0>3}{exit_code:0>4}",
            error_type = self.type_,
            code_identifier = self.code_identifier,
            exit_code = self.code,
        )
    }

    /// Gets a unique warning identifier.
    pub fn warning_code(&self) -> String {
        format!(
            "W{error_type}{code_identifier:0>3}{exit_code:0>4}",
            error_type = self.type_,
            code_identifier = self.code_identifier,
            exit_code = self.code,
        )
    }
}

impl fmt::Display for Backtraced {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (kind, code) = if self.error { ("Error", self.error_code()) } else { ("Warning", self.warning_code()) };
        let message = format!("{kind} [{code}]: {message}", message = self.message,);

        // To avoid the color enabling characters for comparison with test expectations.
        if is_not_test_framework() {
            if self.error {
                write!(f, "{}", message.bold().red())?;
            } else {
                write!(f, "{}", message.bold().yellow())?;
            }
        } else {
            write!(f, "{message}")?;
        };

        if let Some(help) = &self.help {
            write!(
                f,
                "\n{INDENT     } |\n\
            {INDENT     } = {help}",
            )?;
        }

        let leo_backtrace = std::env::var("LEO_BACKTRACE").unwrap_or_default().trim().to_owned();
        match leo_backtrace.as_ref() {
            "1" => {
                let mut printer = BacktracePrinter::default();
                printer = printer.lib_verbosity(Verbosity::Medium);
                let trace = printer.format_trace_to_string(&self.backtrace).map_err(|_| fmt::Error)?;
                write!(f, "{trace}")?;
            }
            "full" => {
                let mut printer = BacktracePrinter::default();
                printer = printer.lib_verbosity(Verbosity::Full);
                let trace = printer.format_trace_to_string(&self.backtrace).map_err(|_| fmt::Error)?;
                write!(f, "{trace}")?;
            }
            _ => {}
        }

        Ok(())
    }
}

impl std::error::Error for Backtraced {
    fn description(&self) -> &str {
        &self.message
    }
}
