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

/// The LeoError type that contains all sub error types.
/// This allows a unified error type throughout the Leo crates.
use crate::LeoMessageCode;

/// Contains the Parser warning definitions.
pub mod parser;
pub use self::parser::*;

/// The LeoWarning type that contains all sub error types.
/// This allows a unified error type throughout the Leo crates.
#[derive(Debug, Error)]
pub enum LeoWarning {
    /// Represents an Parser Error in a Leo Error.
    #[error(transparent)]
    ParserWarning(#[from] ParserWarning),
}

impl LeoWarning {
    /// Implement warning code for each type of Warning.
    pub fn error_code(&self) -> String {
        use LeoWarning::*;

        match self {
            ParserWarning(warning) => warning.warning_code(),
        }
    }
}
