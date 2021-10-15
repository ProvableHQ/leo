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

use eyre::{eyre, ErrReport};

/// SnarkVMError enum that represents all the errors from SnarkVM.
/// Currently implements default for some SnarkVM locations.
/// Ideally SnarkVM would implement a similar error code system to LeoError
/// then we could just bubble the error up with additional information.
#[derive(Debug, Error)]
pub enum SnarkVMError {
    /// Implments from a eyre ErrReport which is a fork of anyhow.
    #[error(transparent)]
    SnarkVMError(#[from] ErrReport),
}

impl Clone for SnarkVMError {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Default for SnarkVMError {
    fn default() -> Self {
        Self::SnarkVMError(eyre!("snarkvm error"))
    }
}
