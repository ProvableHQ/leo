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
    pub exit_code: u32,
    pub code_identifier: u32,
    pub error_type: String,
    #[derivative(PartialEq = "ignore")]
    #[derivative(Hash = "ignore")]
    pub backtrace: Backtrace,
}

impl BacktracedError {
    pub fn new_from_backtrace<S>(
        message: S,
        help: Option<String>,
        exit_code: u32,
        code_identifier: u32,
        error_type: String,
        backtrace: Backtrace,
    ) -> Self
    where
        S: ToString,
    {
        Self {
            message: message.to_string(),
            help: help.map(|help| help.to_string()),
            exit_code,
            code_identifier,
            error_type,
            backtrace,
        }
    }

    pub fn exit_code(&self) -> u32 {
        0
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

        if std::env::var("LEO_BACKTRACE").unwrap_or_default().trim() == "1" {
            write!(f, "stack backtrace:\n{:?}", self.backtrace)?;
        }

        Ok(())
    }
}

impl std::error::Error for BacktracedError {
    fn description(&self) -> &str {
        &self.message
    }
}
