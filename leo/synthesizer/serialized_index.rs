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

use snarkos_models::gadgets::r1cs::Index;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SerializedIndex {
    Input(usize),
    Aux(usize),
}

impl From<Index> for SerializedIndex {
    fn from(index: Index) -> Self {
        match index {
            Index::Input(idx) => Self::Input(idx),
            Index::Aux(idx) => Self::Aux(idx),
        }
    }
}

impl From<&SerializedIndex> for Index {
    fn from(serialized_index: &SerializedIndex) -> Self {
        match serialized_index {
            SerializedIndex::Input(idx) => Index::Input(*idx),
            SerializedIndex::Aux(idx) => Index::Aux(*idx),
        }
    }
}
