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

use crate::{RecordVerificationError, StateLeafValuesError, StateValuesError};

use snarkvm_errors::algorithms::{CommitmentError, MerkleError};

use std::io::Error as IOError;

#[derive(Debug, Error)]
pub enum LocalDataVerificationError {
    #[error("{}", _0)]
    CommitmentError(#[from] CommitmentError),

    #[error("{}", _0)]
    MerkleError(#[from] MerkleError),

    #[error("{}", _0)]
    IOError(#[from] IOError),

    #[error("{}", _0)]
    RecordVerificationError(#[from] RecordVerificationError),

    #[error("{}", _0)]
    StateLeafValuesError(#[from] StateLeafValuesError),

    #[error("{}", _0)]
    StateValuesError(#[from] StateValuesError),
}
