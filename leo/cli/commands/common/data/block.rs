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
use snarkvm::prelude::{
    FromBytes,
    FromBytesDeserializer,
    IoResult,
    Ratifications,
    Solutions,
    Transactions,
    error,
    puzzle::SolutionID,
};
use std::io::Read;

#[derive(Clone, PartialEq, Eq)]
pub struct BlockWrapper<N: Network> {
    /// The block hash.
    pub block_hash: N::BlockHash,
    /// The previous block hash.
    pub previous_hash: N::BlockHash,
    /// The header.
    pub header: Header<N>,
    /// The transactions in this block.
    pub transactions: Transactions<N>,
    /// The aborted transaction IDs.
    pub aborted_transaction_ids: Vec<N::TransactionID>,
}

impl<N: Network> FromBytes for BlockWrapper<N> {
    /// Reads the block from the buffer.
    #[inline]
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        // Read the version.
        let version = u8::read_le(&mut reader)?;
        // Ensure the version is valid.
        if version != 1 {
            return Err(error("Invalid block version"));
        }

        // Read the block hash.
        let block_hash: N::BlockHash = FromBytes::read_le(&mut reader)?;
        // Read the previous block hash.
        let previous_hash: N::BlockHash = FromBytes::read_le(&mut reader)?;
        // Read the header.
        let header: Header<N> = FromBytes::read_le(&mut reader)?;

        // Read the authority.
        let _: AuthorityWrapper<N> = FromBytes::read_le(&mut reader)?;

        // Read the number of ratifications.
        let _: Ratifications<N> = FromBytes::read_le(&mut reader)?;

        // Read the solutions.
        let _: Solutions<N> = FromBytes::read_le(&mut reader)?;

        // Read the number of aborted solution IDs.
        let num_aborted_solutions = u32::read_le(&mut reader)?;
        // Ensure the number of aborted solutions IDs is within bounds (this is an early safety check).
        if num_aborted_solutions as usize > Solutions::<N>::max_aborted_solutions().map_err(error)? {
            return Err(error("Invalid number of aborted solutions IDs in the block"));
        }
        // Read the aborted solution IDs.
        let mut aborted_solution_ids: Vec<SolutionID<N>> = Vec::with_capacity(num_aborted_solutions as usize);
        for _ in 0..num_aborted_solutions {
            aborted_solution_ids.push(FromBytes::read_le(&mut reader)?);
        }

        // Read the transactions.
        let transactions = FromBytes::read_le(&mut reader)?;

        // Read the number of aborted transaction IDs.
        let num_aborted_transactions = u32::read_le(&mut reader)?;
        // Ensure the number of aborted transaction IDs is within bounds (this is an early safety check).
        if num_aborted_transactions as usize > Transactions::<N>::max_aborted_transactions().map_err(error)? {
            return Err(error("Invalid number of aborted transaction IDs in the block"));
        }
        // Read the aborted transaction IDs.
        let mut aborted_transaction_ids: Vec<N::TransactionID> = Vec::with_capacity(num_aborted_transactions as usize);
        for _ in 0..num_aborted_transactions {
            aborted_transaction_ids.push(FromBytes::read_le(&mut reader)?);
        }

        Ok(Self { block_hash, previous_hash, header, transactions, aborted_transaction_ids })
    }
}

impl<'de, N: Network> Deserialize<'de> for BlockWrapper<N> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match deserializer.is_human_readable() {
            true => {
                let mut block = serde_json::Value::deserialize(deserializer)?;
                let block_hash: N::BlockHash = DeserializeExt::take_from_value::<D>(&mut block, "block_hash")?;
                let previous_hash: N::BlockHash = DeserializeExt::take_from_value::<D>(&mut block, "previous_hash")?;
                let header: Header<N> = DeserializeExt::take_from_value::<D>(&mut block, "header")?;
                let transactions = DeserializeExt::take_from_value::<D>(&mut block, "transactions")?;
                let aborted_transaction_ids =
                    DeserializeExt::take_from_value::<D>(&mut block, "aborted_transaction_ids")?;

                Ok(Self { block_hash, previous_hash, header, transactions, aborted_transaction_ids })
            }
            false => FromBytesDeserializer::<Self>::deserialize_with_size_encoding(deserializer, "block"),
        }
    }
}
