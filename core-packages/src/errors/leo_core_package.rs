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

use crate::{CoreCircuitError, CorePackageListError};
use leo_core_ast::{Error as FormattedError, Span};

use std::path::Path;

#[derive(Debug, Error)]
pub enum LeoCorePackageError {
    #[error("{}", _0)]
    CoreCircuitError(#[from] CoreCircuitError),

    #[error("{}", _0)]
    CorePackageListError(#[from] CorePackageListError),

    #[error("{}", _0)]
    Error(#[from] FormattedError),
}

impl LeoCorePackageError {
    pub fn set_path(&mut self, path: &Path) {
        match self {
            LeoCorePackageError::CoreCircuitError(error) => error.set_path(path),
            LeoCorePackageError::CorePackageListError(error) => error.set_path(path),
            LeoCorePackageError::Error(error) => error.set_path(path),
        }
    }

    fn new_from_span(message: String, span: Span) -> Self {
        LeoCorePackageError::Error(FormattedError::new_from_span(message, span))
    }

    pub fn undefined_core_circuit(circuit_name: String, span: Span) -> Self {
        let message = format!("Core circuit `{}` not found in `leo-core`", circuit_name);

        Self::new_from_span(message, span)
    }
}
