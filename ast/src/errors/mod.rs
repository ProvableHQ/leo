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

pub mod deprecated;
pub use deprecated::*;

pub mod error;
pub use error::*;

use error::Error as FormattedError;

use leo_grammar::ParserError;

#[derive(Debug, Error)]
pub enum AstError {
    #[error("{}", _0)]
    DeprecatedError(#[from] DeprecatedError),

    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    IoError(#[from] std::io::Error),

    #[error("{}", _0)]
    ParserError(#[from] ParserError),

    #[error("{}", _0)]
    JsonError(#[from] serde_json::error::Error),
}
