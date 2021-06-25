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

use crate::errors::ValueError;
use leo_asg::AsgConvertError;
use leo_ast::LeoError;
use snarkvm_ir::Type;

#[derive(Debug, Error)]
pub enum OutputBytesError {
    #[error("{}", _0)]
    Error(String),

    #[error("{}", _0)]
    ValueError(#[from] ValueError),

    #[error("{}", _0)]
    AsgConvertError(#[from] AsgConvertError),
}

impl LeoError for OutputBytesError {}

impl OutputBytesError {
    fn new(message: String) -> Self {
        OutputBytesError::Error(message)
    }

    pub fn not_enough_registers() -> Self {
        let message = "number of input registers must be greater than or equal to output registers".to_string();

        Self::new(message)
    }

    pub fn mismatched_output_types(left: &Type, right: &str) -> Self {
        let message = format!(
            "Mismatched types. Expected register output type `{}`, found value `{}`.",
            left, right
        );

        Self::new(message)
    }
}
