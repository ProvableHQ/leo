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
use snarkvm::prelude::{FromBytes, IoResult, error};
use std::io::Read;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Metadata<N: Network> {
    /// The network ID of the block.
    pub network: u16,
    /// The round that produced this block - 8 bytes.
    pub round: u64,
    /// The height of this block - 4 bytes.
    pub height: u32,
    /// The cumulative weight for this block - 16 bytes.
    pub cumulative_weight: u128,
    /// The cumulative proof target for this block - 16 bytes.
    pub cumulative_proof_target: u128,
    /// The coinbase target for this block - 8 bytes.
    pub coinbase_target: u64,
    /// The proof target for this block - 8 bytes.
    pub proof_target: u64,
    /// The coinbase target for the last coinbase - 8 bytes.
    pub last_coinbase_target: u64,
    /// The Unix timestamp (UTC) for the last coinbase - 8 bytes.
    pub last_coinbase_timestamp: i64,
    /// The Unix timestamp (UTC) for this block - 8 bytes.
    pub timestamp: i64,
    pub _phantom: PhantomData<N>,
}

impl<N: Network> FromBytes for Metadata<N> {
    /// Reads the metadata from the buffer.
    #[inline]
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        // Read the version.
        let version = u8::read_le(&mut reader)?;
        // Ensure the version is valid.
        if version != 1 {
            return Err(error("Invalid metadata version"));
        }

        // Read from the buffer.
        let network = u16::read_le(&mut reader)?;
        let round = u64::read_le(&mut reader)?;
        let height = u32::read_le(&mut reader)?;
        let cumulative_weight = u128::read_le(&mut reader)?;
        let cumulative_proof_target = u128::read_le(&mut reader)?;
        let coinbase_target = u64::read_le(&mut reader)?;
        let proof_target = u64::read_le(&mut reader)?;
        let last_coinbase_target = u64::read_le(&mut reader)?;
        let last_coinbase_timestamp = i64::read_le(&mut reader)?;
        let timestamp = i64::read_le(&mut reader)?;

        // Check the network.
        if network != N::ID {
            return Err(error(format!("Got network ID '{network}', expected '{}", N::ID)));
        }

        // Construct the metadata.
        Ok(Self {
            network,
            round,
            height,
            cumulative_weight,
            cumulative_proof_target,
            coinbase_target,
            proof_target,
            last_coinbase_target,
            last_coinbase_timestamp,
            timestamp,
            _phantom: Default::default(),
        })
    }
}

impl<'de, N: Network> Deserialize<'de> for Metadata<N> {
    /// Deserializes the metadata from a JSON-string or buffer.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match deserializer.is_human_readable() {
            true => {
                let mut metadata = serde_json::Value::deserialize(deserializer)?;
                let cumulative_weight: String =
                    DeserializeExt::take_from_value::<D>(&mut metadata, "cumulative_weight")?;
                let cumulative_proof_target: String =
                    DeserializeExt::take_from_value::<D>(&mut metadata, "cumulative_proof_target")?;
                let cumulative_weight = cumulative_weight.parse::<u128>().map_err(de::Error::custom)?;
                let cumulative_proof_target = cumulative_proof_target.parse::<u128>().map_err(de::Error::custom)?;
                let network = DeserializeExt::take_from_value::<D>(&mut metadata, "network")?;
                let round = DeserializeExt::take_from_value::<D>(&mut metadata, "round")?;
                let height = DeserializeExt::take_from_value::<D>(&mut metadata, "height")?;
                let coinbase_target = DeserializeExt::take_from_value::<D>(&mut metadata, "coinbase_target")?;
                let proof_target = DeserializeExt::take_from_value::<D>(&mut metadata, "proof_target")?;
                let last_coinbase_target = DeserializeExt::take_from_value::<D>(&mut metadata, "last_coinbase_target")?;
                let last_coinbase_timestamp =
                    DeserializeExt::take_from_value::<D>(&mut metadata, "last_coinbase_timestamp")?;
                let timestamp = DeserializeExt::take_from_value::<D>(&mut metadata, "timestamp")?;

                // Check the network.
                if network != N::ID {
                    return Err(de::Error::custom(format!("Got network ID '{network}', expected '{}", N::ID)));
                }

                Ok(Self {
                    network,
                    round,
                    height,
                    cumulative_weight,
                    cumulative_proof_target,
                    coinbase_target,
                    proof_target,
                    last_coinbase_target,
                    last_coinbase_timestamp,
                    timestamp,
                    _phantom: Default::default(),
                })
            }
            false => FromBytesDeserializer::<Self>::deserialize_with_size_encoding(deserializer, "metadata"),
        }
    }
}
