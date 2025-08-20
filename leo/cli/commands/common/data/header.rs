// Copyright (C) 2019-2025 Provable Inc.
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

use super::*;
use serde::Deserializer;
use snarkvm::prelude::{FromBytes, FromBytesDeserializer, IoResult, error};
use std::io::Read;

/// The header for the block contains metadata that uniquely identifies the block.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Header<N: Network> {
    /// The Merkle root representing the blocks in the ledger up to the previous block.
    pub previous_state_root: N::StateRoot,
    /// The Merkle root representing the transactions in the block.
    pub transactions_root: Field<N>,
    /// The Merkle root representing the on-chain finalize including the current block.
    pub finalize_root: Field<N>,
    /// The Merkle root representing the ratifications in the block.
    pub ratifications_root: Field<N>,
    /// The solutions root of the puzzle.
    pub solutions_root: Field<N>,
    /// The subdag Merkle root of the authority.
    pub subdag_root: Field<N>,
    /// The metadata of the block.
    pub metadata: Metadata<N>,
}

impl<N: Network> FromBytes for Header<N> {
    /// Reads the block header from the buffer.
    #[inline]
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        // Read the version.
        let version = u8::read_le(&mut reader)?;
        // Ensure the version is valid.
        if version != 1 {
            return Err(error("Invalid header version"));
        }

        // Read from the buffer.
        let previous_state_root = N::StateRoot::read_le(&mut reader)?;
        let transactions_root = Field::<N>::read_le(&mut reader)?;
        let finalize_root = Field::<N>::read_le(&mut reader)?;
        let ratifications_root = Field::<N>::read_le(&mut reader)?;
        let solutions_root = Field::<N>::read_le(&mut reader)?;
        let subdag_root = Field::<N>::read_le(&mut reader)?;
        let metadata = Metadata::<N>::read_le(&mut reader)?;

        // Construct the block header.
        Ok(Self {
            previous_state_root,
            transactions_root,
            finalize_root,
            ratifications_root,
            solutions_root,
            subdag_root,
            metadata,
        })
    }
}

impl<'de, N: Network> Deserialize<'de> for Header<N> {
    /// Deserializes the header from a JSON-string or buffer.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match deserializer.is_human_readable() {
            true => {
                let mut header = serde_json::Value::deserialize(deserializer)?;
                let previous_state_root = DeserializeExt::take_from_value::<D>(&mut header, "previous_state_root")?;
                let transactions_root = DeserializeExt::take_from_value::<D>(&mut header, "transactions_root")?;
                let finalize_root = DeserializeExt::take_from_value::<D>(&mut header, "finalize_root")?;
                let ratifications_root = DeserializeExt::take_from_value::<D>(&mut header, "ratifications_root")?;
                let solutions_root = DeserializeExt::take_from_value::<D>(&mut header, "solutions_root")?;
                let subdag_root = DeserializeExt::take_from_value::<D>(&mut header, "subdag_root")?;
                let metadata = DeserializeExt::take_from_value::<D>(&mut header, "metadata")?;

                Ok(Self {
                    previous_state_root,
                    transactions_root,
                    finalize_root,
                    ratifications_root,
                    solutions_root,
                    subdag_root,
                    metadata,
                })
            }
            false => FromBytesDeserializer::<Self>::deserialize_with_size_encoding(deserializer, "block header"),
        }
    }
}
