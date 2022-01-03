// Copyright (C) 2019-2022 Aleo Systems Inc.
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

use serde::{Deserialize, Serialize};
use snarkvm_r1cs::Index;

#[derive(Serialize, Deserialize)]
pub enum SerializedIndex {
    Public(usize),
    Private(usize),
}

impl From<Index> for SerializedIndex {
    fn from(index: Index) -> Self {
        match index {
            Index::Public(idx) => Self::Public(idx),
            Index::Private(idx) => Self::Private(idx),
        }
    }
}

impl From<&SerializedIndex> for Index {
    fn from(serialized_index: &SerializedIndex) -> Self {
        match serialized_index {
            SerializedIndex::Public(idx) => Index::Public(*idx),
            SerializedIndex::Private(idx) => Index::Private(*idx),
        }
    }
}
