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

use crate::FrameError;
use leo_typed::Error as FormattedError;

use std::path::Path;

/// Errors encountered when running dynamic type inference checks.
#[derive(Debug, Error)]
pub enum DynamicCheckError {
    #[error("{}", _0)]
    Error(#[from] FormattedError),

    #[error("{}", _0)]
    FrameError(#[from] FrameError),
}

impl DynamicCheckError {
    ///
    /// Set the filepath for the error stacktrace.
    ///
    pub fn set_path(&mut self, path: &Path) {
        match self {
            DynamicCheckError::Error(error) => error.set_path(path),
            DynamicCheckError::FrameError(error) => error.set_path(path),
        }
    }
}
